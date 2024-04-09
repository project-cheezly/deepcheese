use std::collections::HashMap;
use std::env::temp_dir;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use reqwest::Url;

#[cfg(not(test))]
use log::{debug, info, warn};
#[cfg(test)]
use std::{println as info, println as debug, println as warn};
use reqwest::header::HeaderMap;
use tokio::sync::RwLock;

use crate::config::{AppConfig, endpoint, uri};

pub(crate) struct KISAuth {
    config: Arc<AppConfig>,
    access_token: Arc<RwLock<AccessToken>>
}

impl KISAuth {
    pub(crate) async fn new(config: Arc<AppConfig>) -> Self {
        let access_token = get_token(&*config).await.unwrap();

        KISAuth {
            config,
            access_token: Arc::new(RwLock::new(access_token)),
        }
    }

    pub(crate) async fn get_header_map(&self) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut token = self.access_token.write().await;

        if !token.is_validate() {
            *token = refresh_token(&self.config).await?;
        }

        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json; charset=utf-8".parse()?);
        headers.insert("authorization", format!("Bearer {}", token.access_token).parse()?);
        headers.insert("appkey", self.config.auth.app_id.parse()?);
        headers.insert("appsecret", self.config.auth.app_secret.parse()?);

        Ok(headers)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    fn validate_token(self) -> Option<Self> {
        if !self.is_validate() {
            info!("토큰이 만료되었습니다.");
            return None;
        }

        Some(self)
    }

    fn is_validate(&self) -> bool {
        self.expires_at > Utc::now() + chrono::Duration::hours(1)
    }
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    access_token_token_expired: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    expires_in: i64,
}

/// 프로그램 첫 시작 시 토큰을 가져옵니다.
/// 만료된 토큰이나 토큰이 없을 경우 새로 발급합니다.
///
/// ## Error
///
/// 토큰 발급에 실패하거나 토큰 파일을 읽거나 쓰는 중 오류가 발생할 경우 에러를 반환합니다.
pub async fn get_token(config: &AppConfig) -> Result<AccessToken, Box<dyn std::error::Error>> {
    let loaded_token = load_token()
        .and_then(|token| token.validate_token());

    match loaded_token {
        Some(token) => Ok(token),
        None => refresh_token(config).await
    }
}

fn load_token() -> Option<AccessToken> {
    let token_path = get_token_path();

    if !token_path.exists() {
        info!("토큰 파일이 존재하지 않습니다.");
        return None;
    }

    let token_file = match std::fs::read_to_string(token_path) {
        Ok(token_file) => token_file,
        Err(e) => {
            info!("토큰 파일을 읽는 중 오류가 발생했습니다: {}", e);
            return None;
        }
    };

    match serde_json::from_str::<AccessToken>(&token_file) {
        Ok(token) => Some(token),
        Err(e) => {
            info!("토큰 파일을 파싱하는 중 오류가 발생했습니다: {}", e);
            None
        }
    }
}

fn get_token_path() -> std::path::PathBuf {
    let mut token_path = temp_dir();
    token_path.push("kis_rust");
    token_path.push("token.json");

    token_path
}

async fn refresh_token(config: &AppConfig) -> Result<AccessToken, Box<dyn std::error::Error>> {
    let new_token = publish_token(config).await?;
    save_token(&new_token)?;

    Ok(new_token)
}

async fn publish_token(config: &AppConfig) -> Result<AccessToken, Box<dyn std::error::Error>> {
    let body = HashMap::from([
        ("grant_type", "client_credentials"),
        ("appkey", &config.auth.app_id),
        ("appsecret", &config.auth.app_secret),
    ]);

    let url_result = Url::parse(&uri::PRODUCTION)
        .and_then(|url| url.join(&endpoint::AUTH_PUBLISH_TOKEN));

    let url = match url_result {
        Ok(url) => url,
        Err(e) => {
            warn!("URL을 생성하는 중 오류가 발생했습니다: {}", e);
            return Err(e.into());
        }
    };

    let client = reqwest::Client::new();

    let res_result = client.post(url)
        .json(&body)
        .send();

    let response = match res_result.await {
        Ok(res) if !res.status().is_success() => {
            warn!("토큰을 발급하는 중 오류가 발생했습니다: {:?}", res);
            return Err("토큰 발급 실패".into());
        },
        Err(e) => {
            warn!("토큰을 발급하는 중 오류가 발생했습니다: {}", e);
            return Err(e.into());
        },
        Ok(res) => res,
    };

    info!("토큰을 발급받았습니다.");
    Ok(parse_token(response.json::<TokenResponse>().await?))
}

fn parse_token(token_response: TokenResponse) -> AccessToken {
    let parsed_datetime = FixedOffset::east_opt(9 * 3600).unwrap()
        .from_local_datetime(&NaiveDateTime::parse_from_str(
            &token_response.access_token_token_expired,
            "%Y-%m-%d %H:%M:%S"
        ).unwrap()
    ).unwrap().to_utc();

    AccessToken {
        access_token: token_response.access_token,
        expires_at: parsed_datetime,
    }
}

fn save_token(token: &AccessToken) -> Result<(), Box<dyn std::error::Error>> {
    let token_path = get_token_path();

    if !token_path.exists() {
        std::fs::create_dir_all(token_path.parent().unwrap())?;
    }

    let token_file = serde_json::to_string::<AccessToken>(token)?;
    std::fs::write(token_path, token_file)?;

    debug!("토큰 파일을 저장했습니다.");
    Ok(())
}

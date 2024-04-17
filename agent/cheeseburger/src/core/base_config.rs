#[macro_export]
macro_rules! declare_config {
    ($name: expr, $t: ty) => {
        use config::{File, Config};
        use crate::error::CheeseburgerError;
        use serde::Deserialize;

        #[derive(Deserialize, Debug, Clone)]
        pub struct BaseConfig {
            #[serde(rename=$name)]
            pub conf: $t
        }
    }
}

#[macro_export]
macro_rules! load_mac {
    () => {
        Config::builder()
            .add_source(File::with_name("config.toml"))
            .build()
            .and_then(|conf| conf.try_deserialize::<BaseConfig>())
            .and_then(|conf| Ok(conf.conf))
            .or_else(|e| {
                eprintln!("Error: {}", e);
                Err(CheeseburgerError::ConfigLoadError.into())
            })
    }
}


pub(crate) use declare_config;
pub(crate) use load_mac;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Account;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    struct CheonMoreConfig {
        #[serde(rename = "account")]
        pub _account: Account
    }


    #[test]
    fn test_load() {
        declare_config!("cheon_more", CheonMoreConfig);
        let conf: Result<CheonMoreConfig, Box<dyn std::error::Error>> = load_mac!();

        dbg!(&conf);
        assert!(conf.is_ok());
    }
}
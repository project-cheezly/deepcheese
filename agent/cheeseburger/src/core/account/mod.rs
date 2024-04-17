use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    pub number: String,
    pub password: String,
}

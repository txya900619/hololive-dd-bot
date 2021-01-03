use std::env;
use std::error::Error;
pub struct Config {
    pub api_key: String,
    pub api_secret: String,
    pub access_token: String,
    pub access_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Config, Box<dyn Error>> {
        let api_key = env::var("TWITTER_API_KEY")?;
        let api_secret = env::var("TWITTER_API_SECRET")?;
        let access_token = env::var("TWITTER_ACCESS_TOKEN")?;
        let access_secret = env::var("TWITTER_ACCESS_SECRET")?;
        Ok(Config {
            api_key,
            api_secret,
            access_token,
            access_secret,
        })
    }
}

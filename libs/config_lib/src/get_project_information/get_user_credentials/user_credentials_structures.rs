use crate::get_project_information::project_constants::USER_CREDENTIALS_FILE_NAME;
use config::{Config, ConfigError, File};
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct UserCredentialsStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
    pub mongo_authorization: MongoAuthorization,
}

impl UserCredentialsStruct {
    pub fn new(path_to_config: &str) -> Result<Self, ConfigError> {
        // maybe add different user logic later ?
        let mut config = Config::new();
        config.merge(File::with_name(&format!(
            "{}{}",
            path_to_config, USER_CREDENTIALS_FILE_NAME
        )))?;
        config.try_into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GithubAuthorization {
    pub github_name: String,
    pub github_token: String,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditAuthorization {
    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoAuthorization {
    pub is_remote: bool,
    pub mongo_local_login: String,
    pub mongo_local_password: String,
    pub mongo_remote_login: String,
    pub mongo_remote_password: String,
}

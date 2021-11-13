use crate::config_mods::config_structs::config_struct::ConfigStruct;

use crate::constants::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;

use crate::constants::tests_constants::_USER_CREDENTIALS_DUMMY_HANDLE;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_reddit_auth_info_with_config_init() {
    let config_handle: ConfigStruct = ConfigStruct::new().expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
    let reddit_user_agent = &config_handle.reddit_authorization.reddit_user_agent;
    let reddit_client_id = &config_handle.reddit_authorization.reddit_client_id;
    let reddit_client_secret = &config_handle.reddit_authorization.reddit_client_secret;
    let reddit_username = &config_handle.reddit_authorization.reddit_username;
    let reddit_password = &config_handle.reddit_authorization.reddit_password;
    assert!(
        !(reddit_user_agent != _USER_CREDENTIALS_DUMMY_HANDLE),
        "reddit_user_agent != {}",
        _USER_CREDENTIALS_DUMMY_HANDLE
    );
    assert!(
        !(reddit_client_id != _USER_CREDENTIALS_DUMMY_HANDLE),
        "reddit_client_id != {}",
        _USER_CREDENTIALS_DUMMY_HANDLE
    );
    assert!(
        !(reddit_client_secret != _USER_CREDENTIALS_DUMMY_HANDLE),
        "reddit_client_secret != {}",
        _USER_CREDENTIALS_DUMMY_HANDLE
    );
    assert!(
        !(reddit_username != _USER_CREDENTIALS_DUMMY_HANDLE),
        "reddit_username != {}",
        _USER_CREDENTIALS_DUMMY_HANDLE
    );
    assert!(
        !(reddit_password != _USER_CREDENTIALS_DUMMY_HANDLE),
        "reddit_password != {}",
        _USER_CREDENTIALS_DUMMY_HANDLE
    );
}

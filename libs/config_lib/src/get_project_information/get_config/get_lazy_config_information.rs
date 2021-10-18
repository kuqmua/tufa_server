use crate::get_project_information::get_config::config_struct::ConfigStruct;
use crate::get_project_information::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;

lazy_static! {
    pub static ref CONFIG: ConfigStruct =
        ConfigStruct::new().expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
}


use std::collections::HashMap;
use crate::get_project_information::env_var_enum::EnvVar;
use crate::get_project_information::env_var_enum::EnvVarTypeValueHandle;
lazy_static! {
    pub static ref TEST: HashMap::<EnvVar, EnvVarTypeValueHandle> =
    EnvVar::test_something().expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
}
use crate::get_project_information::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::get_project_information::env_var_types_enum::EnvVarTypes;

#[derive(Debug)]
pub struct ConfigError<'a> {
    pub env_var_name_kind: EnvVarTypes,
    pub was_dotenv_enable: bool,
    pub env_name: &'a str,
    pub env_error: ConfigErrorInnerType,
}

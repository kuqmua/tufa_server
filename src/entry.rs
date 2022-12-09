use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::hardcode::PROJECT_NAME;
use crate::global_variables::runtime::config::CONFIG;
use crate::preparation::prepare_server::prepare_server;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::server_wrapper::server_wrapper;
use crate::telemetry::get_subscriber::get_subscriber;
use crate::telemetry::init_subscriber::init_subscriber;
use tufa_common::config_mods::print_type::PrintType;
use tufa_common::config_mods::traits::fields::GetLogType;
use tufa_common::traits::get_git_source_file_link::GetGitSourceFileLink;
// use valuable::Valuable;

// #[derive(Clone, Debug, Valuable)]
// struct User {
//     name: String,
//     age: u32,
//     something: Vec<bool>,
//     address: Address,
// }

// #[derive(Clone, Debug, Valuable)]
// struct Address {
//     country: String,
//     city: String,
//     street: String,
// }

pub fn entry() {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                tufa_common::config_mods::print_type::PrintType::WarningHigh,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                format!("Cannot build tokio runtime {e:#?}"),
            );
        }
        Ok(runtime) => {
            one(true);
            if let tufa_common::config_mods::log_type::LogType::Tracing = CONFIG.log_type {
                if let Err(e) = init_subscriber(get_subscriber(
                    PROJECT_NAME.into(),
                    CONFIG.tracing_type.to_lower_snake_case(),
                    std::io::stdout,
                )) {
                    print_colorful_message(
                        None,
                        tufa_common::config_mods::print_type::PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                        format!("tracing init_subscriber error: {:#?}", e),
                    );
                    return;
                };
            }
            // let user = User {
            //     name: "Arwen Undomiel".to_string(),
            //     age: 3000,
            //     something: vec![true, false],
            //     address: Address {
            //         country: "Middle Earth".to_string(),
            //         city: "Rivendell".to_string(),
            //         street: "leafy lane".to_string(),
            //     },
            // };
            // tracing::error!(valuable = false, user = ?user);
            if let true = CONFIG.is_preparation_enabled {
                if runtime.block_on(prepare_server(true)).is_err() {
                    return;
                }
            }
            if let Err(e) = server_wrapper() {
                print_colorful_message(
                    None,
                    tufa_common::config_mods::print_type::PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                    format!("Cannot run actix-web HttpServer, error: {:#?}", e),
                );
            }
        }
    }
}

use tufa_common::common::code_occurence::ThreeOriginError;
use impl_get_source::ImplGetSourceFromTufaCommon;
use tufa_common::traits::new_error_with_addition::NewErrorWithAddition;

#[derive(ImplGetSourceFromTufaCommon)]
pub struct OneWrapperError {
    source: OneWrapperErrorEnum,
    code_occurence: tufa_common::common::code_occurence::CodeOccurence,
}

impl tufa_common::traits::get_code_occurence::GetCodeOccurence for OneWrapperError {
    fn get_code_occurence(&self) -> &tufa_common::common::code_occurence::CodeOccurence {
        &self.code_occurence
    }
}
 
impl tufa_common::traits::new_error_test::NewErrorTestTestTest<OneWrapperErrorEnum> for OneWrapperError {
    fn new(
        source: OneWrapperErrorEnum,
        code_occurence: tufa_common::common::code_occurence::CodeOccurence,
    ) -> Self {
        Self {
            source,
            code_occurence,
        }
    }
}

#[derive(ImplGetSourceFromTufaCommon)]
pub enum OneWrapperErrorEnum {
    ThreeWrapper(ThreeOriginError),
}

impl tufa_common::traits::get_code_occurence::GetCodeOccurence for OneWrapperErrorEnum {
    fn get_code_occurence(&self) -> &tufa_common::common::code_occurence::CodeOccurence{
        match self {
            OneWrapperErrorEnum::ThreeWrapper(e) => e.get_code_occurence(),
        }
    }
}

pub fn one(should_trace: bool) -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::common::code_occurence::three() {
        return Err(Box::new(OneWrapperError::new_error_with_addition(
            OneWrapperErrorEnum::ThreeWrapper(*e), 
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG), 
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES), 
            String::from(file!()), 
            line!(), 
            column!(), 
            should_trace
        )));
    }
    Ok(())
}
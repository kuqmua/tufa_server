use serde_json::Value;

use crate::helpers::create_dir_if_it_doesnt_exist::create_dir_if_it_doesnt_exist;
use crate::helpers::write_json_into_file::write_json_into_file;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use std::fs::File;
use std::io::ErrorKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_write_error_logs_into_file_for_provider_wrapper_checker(
    json_object: Value,
    provider_kind: ProviderKind,
    dir: &str,
    warning_logs_directory_name: &str,
    link: &str,
) -> Result<(), std::io::Error> {
    let path_to_provider_log_file = format!(
        "logs/{}/{:?}/{}",
        warning_logs_directory_name, provider_kind, warning_logs_directory_name
    );
    if let Err(e) = create_dir_if_it_doesnt_exist(&path_to_provider_log_file) {
        print_colorful_message(
            Some(&provider_kind),
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            format!(
                "folder creation path is not valid: {}, error: {:#?}",
                path_to_provider_log_file, e
            ),
        );
        return Err(e);
    };
    let replaced_link = link.replace("/", "-").replace(":", "-").replace(".", "-");
    let path_to_file = format!(
        "logs/{}/{:?}/{}/{:?}-{}.json",
        &warning_logs_directory_name,
        ProviderKind::get_string_name(provider_kind),
        dir,
        ProviderKind::get_string_name(provider_kind),
        replaced_link
    ); //add save function what convert string into save path
    let result_of_opening_file = File::open(&path_to_file);
    match result_of_opening_file {
        Ok(_) => {
            print_colorful_message(
                Some(&provider_kind),
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "there is file with the same name: {}, file was not written",
                    &path_to_file
                ),
            );
        }
        Err(ref err) => {
            if err.kind() == ErrorKind::NotFound {
                //todo write into mongo collection and create flag where to write logs
                let _successfully = write_json_into_file(path_to_file, json_object);
            } else {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "unexpected error while opening file, description: {:#?}",
                        &err.kind()
                    ),
                );
                let _successfully = write_json_into_file(path_to_file, json_object);
            }
        }
    }
    Ok(())
}

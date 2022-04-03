use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::get_git_source_file_link::get_git_source_file_link;
use crate::preparation::preparation;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::server_wrapper::server_wrapper;
use std::time::Instant;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn entry() {
    let time = Instant::now();
    let cpus = num_cpus::get();
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cpus)
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![get_git_source_file_link(file!(), line!())],
                format!("Cannot build tokio runtime {e:#?}"),
            );
        }
        Ok(runtime) => {
            print_colorful_message(
                None,
                PrintType::Info,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![get_git_source_file_link(file!(), line!())],
                format!(
                    "server running on {}:{} with {cpus} CPUs",
                    CONFIG.server_ip, CONFIG.server_port
                ),
            );
            runtime.block_on(preparation());
            print_colorful_message(
                None,
                PrintType::TimeMeasurement,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![get_git_source_file_link(file!(), line!())],
                format!("preparation done in {} seconds", time.elapsed().as_secs()),
            );
            if let Err(e) = server_wrapper() {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![get_git_source_file_link(file!(), line!())],
                    format!("Cannot run actix-web HttpServer {e}"),
                );
            }
        }
    }
}

use std::time::Instant;

extern crate num_cpus;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::server_wrapper::server_wrapper;

use crate::check_net::check_net_wrapper::check_net_wrapper;

use crate::init_dbs_logic::init_dbs::init_dbs;

use crate::helpers::get_git_source_file_link::get_git_source_file_link;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn entry() {
    let time = Instant::now();
    let cpus = num_cpus::get();
    print_colorful_message(
        None,
        PrintType::Info,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![get_git_source_file_link(file!(), line!())],
        format!("We are on a multicore system with {cpus} CPUs"),
    );
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
            return;
        }
        Ok(runtime) => {
            if let Err(e) = runtime.block_on(check_net_wrapper()) {
                println!("{e}");
                // let sources = e
                //     .source
                //     .iter()
                //     .map(|e| match e {
                //         CheckNetError::Net(e) => match e {
                //             CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //             CheckNetAvailabilityErrorEnum::StatusCodeError {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //         },
                //         CheckNetError::Postgres(e) => e.where_was.source_place_with_readable_time(),
                //         CheckNetError::Mongo(e) => match e {
                //             MongoCheckAvailabilityErrorEnum::ClientOptionsParse {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //             MongoCheckAvailabilityErrorEnum::ClientWithOptions {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //             MongoCheckAvailabilityErrorEnum::ListCollectionNames {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //         },
                //     })
                //     .collect::<Vec<String>>();
                // let github_sources = e
                //     .source
                //     .iter()
                //     .map(|e| match e {
                //         CheckNetError::Net(e) => match e {
                //             CheckNetAvailabilityErrorEnum::CheckLinkStatusCodeError {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //             CheckNetAvailabilityErrorEnum::StatusCodeError {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //         },
                //         CheckNetError::Postgres(e) => e.where_was.source_place_with_readable_time(),
                //         CheckNetError::Mongo(e) => match e {
                //             MongoCheckAvailabilityErrorEnum::ClientOptionsParse {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //             MongoCheckAvailabilityErrorEnum::ClientWithOptions {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //             MongoCheckAvailabilityErrorEnum::ListCollectionNames {
                //                 source: _,
                //                 where_was,
                //             } => where_was.source_place_with_readable_time(),
                //         },
                //     })
                //     .collect::<Vec<String>>();
                // print_colorful_message(
                //     None,
                //     PrintType::WarningHigh,
                //     sources,
                //     github_sources,
                //     format!("{e:#?}"),
                // );
                return;
            };
            if CONFIG.is_dbs_initialization_enabled {
                if !CONFIG.is_mongo_initialization_enabled
                    && !CONFIG.is_postgres_initialization_enabled
                {
                    print_colorful_message(
                        None,
                        PrintType::WarningLow,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![get_git_source_file_link(file!(), line!())],
                        String::from("db initialization for mongo and postgres are disabled"),
                    );
                } else if let Err(e) = runtime.block_on(init_dbs()) {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        vec![],
                        vec![],
                        format!("{e:#?}"),
                    );
                }
            }
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

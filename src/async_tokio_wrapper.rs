use std::thread;

use crate::check_new_posts_threads_parts::check_new_posts_threads_parts;

use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;
use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::async_write_fetch_error_logs_into_mongo_wrapper;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_insert_data::mongo_insert_data;

use crate::postgres_integration::postgres_create_post::postgres_create_post;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::providers::provider_kind_enum::ProviderKind;
// for key in vec_of_provider_names.clone() {
//     let future_possible_drop_collection = mongo_drop_collection_wrapper(
//         mongo_url,
//         db_name_handle,
//         &format!("{}{}", key, db_collection_handle_second_part),
//         false,
//     );
//     match future_possible_drop_collection {
//         Ok(result_flag) => {
//             if result_flag {
//                 println!("drop done!");
//             } else {
//                 println!("drop fail with flag");
//             }
//         }
//         Err(e) => {
//             println!("drop fail with error {:#?}", e);
//         }
//     }
// }

#[deny(clippy::indexing_slicing)]
#[tokio::main]
pub async fn async_tokio_wrapper() {
    if CONFIG
        .params
        .enable_initialize_mongo_with_providers_link_parts
    {
        let vec_of_link_parts_hashmap = ProviderKind::get_providers_json_local_data();
        if !vec_of_link_parts_hashmap.is_empty() {
            //todo: add check of doc already is in collection or add flag forse
            //todo add flag for provider
            let result_postgres_establish_connection =
                PgConnection::establish(&postgres_get_db_url());
            match result_postgres_establish_connection {
                Ok(pg_connection) => {
                    let _ = postgres_create_post(&pg_connection, "post_title", "post_body");
                }
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "PgConnection::establish {} error: {:#?}",
                            &postgres_get_db_url(),
                            e
                        ),
                    );
                }
            }
            let _ = mongo_insert_data(
                &CONFIG.mongo_params.providers_db_name_handle,
                vec_of_link_parts_hashmap,
            )
            .await;
        }
    }
    let option_tuple = check_new_posts_threads_parts().await;
    match option_tuple {
        Some((_posts, error_posts)) => {
            if !error_posts.is_empty() {
                let wrong_cases_thread = thread::spawn(move || {
                    // println!("error_posts_done_len{:#?}", error_posts);
                    //todo add flag in config or if its already exists put it here
                    // pub enable_initialize_mongo_with_providers_link_parts: bool,
                    if CONFIG.params.enable_write_error_logs_in_local_folder
                        && CONFIG.params.enable_write_error_logs_in_mongo
                    {
                        async_write_fetch_error_logs_into_mongo_wrapper(error_posts.clone());
                        let cleaning_hashmap_result = ProviderKind::clean_providers_logs_directory();
                        if cleaning_hashmap_result.len() == 0 {
                            rss_async_write_fetch_error_logs_into_files_wrapper(error_posts);
                        }
                        else {
                            for (provider_kind, error) in cleaning_hashmap_result {
                                print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("ProviderKind::clean_providers_logs_directory() failed for {:#?} (todo1) error: {:#?}", provider_kind, error),
                                );
                            }
                        }
                        
                    } else if CONFIG.params.enable_write_error_logs_in_local_folder {
                        async_write_fetch_error_logs_into_mongo_wrapper(error_posts);
                    } else if CONFIG.params.enable_write_error_logs_in_mongo {
                        let cleaning_hashmap_result = ProviderKind::clean_providers_logs_directory();
                        if cleaning_hashmap_result.len() == 0 {
                            rss_async_write_fetch_error_logs_into_files_wrapper(error_posts);
                        }
                        else {
                            for (provider_kind, error) in cleaning_hashmap_result {
                                print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("ProviderKind::clean_providers_logs_directory() failed for {:#?} (todo2) error: {:#?}", provider_kind, error),
                                );
                            }
                        }
                    }
                });
                match wrong_cases_thread.join() {
                    Ok(_) => {}
                    Err(e) => {
                        print_colorful_message(
                            None,
                            PrintType::Error,
                            file!().to_string(),
                            line!().to_string(),
                            format!("wrong_cases_thread.join() error: {:#?}", e),
                        );
                    }
                }
            }
        }
        None => {
            print_colorful_message(
                None,
                PrintType::WarningLow,
                file!().to_string(),
                line!().to_string(),
                "check_new_posts_threads_parts().await - no new posts".to_string(),
            );
        }
    }
}

use crate::fetch::rss_check_provider_status::rss_check_provider_status;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use std::sync::{Arc, Mutex};
use std::thread;

pub fn rss_check_available_providers(twitter_providers_names: Vec<String>) -> Vec<String> {
    let mut threads_vector = Vec::with_capacity(twitter_providers_names.len());
    let twitter_providers_links_available = Arc::new(Mutex::new(Vec::new()));
    for provider_name in &mut twitter_providers_names.into_iter() {
        let twitter_providers_links_available_handle =
            Arc::clone(&twitter_providers_links_available);
        let handle = thread::spawn(move || {
            let provider_link: String = format!("https://{}/TheCherno/rss", provider_name); //choose random account from following
            let check_status_result = rss_check_provider_status(&provider_link);
            match check_status_result {
                Ok(fetch_tuple_result) => {
                    if fetch_tuple_result.0 {
                        let mut twitter_providers_links_available_handle_locked =
                            twitter_providers_links_available_handle.lock().unwrap();
                        twitter_providers_links_available_handle_locked.push(provider_name);
                    }
                }
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!(),
                        line!(),
                        column!(),
                        format!("UnhandledFetchStatusInfo::Failure {:#?}", e),
                    );
                }
            }
        });
        threads_vector.push(handle);
    }
    for thread in threads_vector {
        thread.join().unwrap();
    }
    let twitter_providers_links_available_done =
        twitter_providers_links_available.lock().unwrap().to_vec();
    twitter_providers_links_available_done
}

use std::thread;
use std::thread::JoinHandle;

use providers_info_lib::get_project_information::get_names::get_arxiv_names::get_arxiv_names;
use providers_info_lib::get_project_information::get_names::get_biorxiv_names::get_biorxiv_names;
use providers_info_lib::get_project_information::get_names::get_github_names::get_github_names;
use providers_info_lib::get_project_information::get_names::get_habr_names::get_habr_names;
use providers_info_lib::get_project_information::get_names::get_medrxiv_names::get_medrxiv_names;
use providers_info_lib::get_project_information::get_names::get_reddit_names::get_reddit_names;
use providers_info_lib::get_project_information::get_names::get_twitter_names::get_twitter_names;

use crate::fetch::rss_part::rss_part;

use prints_lib::print_colorful_message;
use prints_lib::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use std::sync::{Arc, Mutex};

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use providers_info_lib::get_project_information::get_twitter_providers_names::get_twitter_providers_names;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::project_constants::ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT;
use config_lib::get_project_information::project_constants::BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT;
use config_lib::get_project_information::project_constants::GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT;
use config_lib::get_project_information::project_constants::HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT;
use config_lib::get_project_information::project_constants::MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT;
use config_lib::get_project_information::project_constants::REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT;
use config_lib::get_project_information::project_constants::TWITTER_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT;

use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use providers_info_lib::get_providers_link_parts_wrapper::get_providers_link_parts_wrapper;

pub async fn check_new_posts_threads_parts() -> Option<(
    Vec<CommonRssPostStruct>,
    Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
)> {
    if CONFIG.params.enable_all_providers {
        if !CONFIG.params.vec_of_provider_names.is_empty() {
            let option_providers_link_parts = get_providers_link_parts_wrapper();
            match option_providers_link_parts {
                Some(providers_link_parts) => {
                    if !providers_link_parts.is_empty() {
                        let mut threads_vec: Vec<JoinHandle<()>> =
                            Vec::with_capacity(CONFIG.params.vec_of_provider_names.len());
                        let posts = Arc::new(Mutex::new(Vec::<CommonRssPostStruct>::new()));
                        let error_posts = Arc::new(Mutex::new(Vec::<(
                            String,
                            UnhandledFetchStatusInfo,
                            HandledFetchStatusInfo,
                            AreThereItems,
                            ProviderKind,
                        )>::new()));
                        //check if provider_names are unique
                        for provider_name in &CONFIG.params.vec_of_provider_names {
                            if provider_name
                                == ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT.config_name_value
                            {
                                if CONFIG.enable_providers.enable_arxiv {
                                    if providers_link_parts.contains_key(
                                        ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                            .config_name_value,
                                    ) {
                                        let arxiv_links = &providers_link_parts
                                            [ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                .config_name_value];
                                        if arxiv_links.is_empty() {
                                            print_colorful_message(
                                                PrintType::Error,
                                                file!().to_string(),
                                                line!().to_string(),
                                                "arxiv_links.is_empty".to_string(),
                                            );
                                        } else {
                                            if CONFIG.params.enable_all_providers_prints
                                                && CONFIG.enable_prints.enable_prints_arxiv
                                            {
                                                println!(
                                                    "{:#?} elements in {:#?} HashMap",
                                                    arxiv_links.len(),
                                                    ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type
                                                );
                                            };
                                            let posts_handle = Arc::clone(&posts);
                                            let error_posts_handle = Arc::clone(&error_posts);
                                            threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_arxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_prints
                                                            .enable_warning_prints_for_arxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_arxiv,
                                                    CONFIG.params.enable_all_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_arxiv_time_measurement,
                                                    &CONFIG.links.arxiv_link,
                                                    &ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type,
                                                    CONFIG.params.enable_error_prints_handle,
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }));
                                        }
                                    } else {
                                        print_colorful_message(
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!(
                                                "providers_link_parts.contains_key({}) is false",
                                                ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .config_name_value
                                            ),
                                        );
                                    }
                                }
                            } else if provider_name
                                == BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT.config_name_value
                            {
                                if CONFIG.enable_providers.enable_biorxiv {
                                    if providers_link_parts.contains_key(
                                        BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                            .config_name_value,
                                    ) {
                                        let biorxiv_links = &providers_link_parts
                                            [BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                .config_name_value];
                                        if biorxiv_links.is_empty() {
                                            print_colorful_message(
                                                PrintType::Error,
                                                file!().to_string(),
                                                line!().to_string(),
                                                "biorxiv_links.is_empty".to_string(),
                                            )
                                        } else {
                                            if CONFIG.params.enable_all_providers_prints
                                                && CONFIG.enable_prints.enable_prints_biorxiv
                                            {
                                                println!(
                                                    "{:#?} elements in {:#?} HashMap",
                                                    biorxiv_links.len(),
                                                    BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type
                                                );
                                            };
                                            let posts_handle = Arc::clone(&posts);
                                            let error_posts_handle = Arc::clone(&error_posts);
                                            threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG
                                                            .enable_prints
                                                            .enable_prints_biorxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_prints
                                                            .enable_warning_prints_for_biorxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_biorxiv,
                                                    CONFIG.params.enable_all_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_biorxiv_time_measurement,
                                                    &CONFIG.links.biorxiv_link,
                                                    &BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type,
                                                    CONFIG.params.enable_error_prints_handle,
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }));
                                        }
                                    } else {
                                        print_colorful_message(
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!(
                                                "providers_link_parts.contains_key({}) is false",
                                                BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .config_name_value
                                            ),
                                        );
                                    }
                                }
                            } else if provider_name
                                == GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT.config_name_value
                            {
                                if CONFIG.enable_providers.enable_github {
                                    if providers_link_parts.contains_key(
                                        GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                            .config_name_value,
                                    ) {
                                        let github_links = &providers_link_parts
                                            [GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                .config_name_value];
                                        if github_links.is_empty() {
                                            print_colorful_message(
                                                PrintType::Error,
                                                file!().to_string(),
                                                line!().to_string(),
                                                "github_links.is_empty".to_string(),
                                            )
                                        } else {
                                            if CONFIG.params.enable_all_providers_prints
                                                && CONFIG.enable_prints.enable_prints_github
                                            {
                                                println!(
                                                    "{:#?} elements in {:#?} HashMap",
                                                    github_links.len(),
                                                    GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type
                                                );
                                            };
                                            let posts_handle = Arc::clone(&posts);
                                            let error_posts_handle = Arc::clone(&error_posts);
                                            threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG
                                                            .enable_prints
                                                            .enable_prints_github,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_prints
                                                            .enable_warning_prints_for_github,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_github,
                                                    CONFIG.params.enable_all_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_github_time_measurement,
                                                    &CONFIG.links.github_link,
                                                    &GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type,
                                                    CONFIG.params.enable_error_prints_handle,
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }));
                                        }
                                    } else {
                                        print_colorful_message(
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!(
                                                "providers_link_parts.contains_key({}) is false",
                                                GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .config_name_value
                                            ),
                                        );
                                    }
                                }
                            } else if provider_name
                                == HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT.config_name_value
                            {
                                if CONFIG.enable_providers.enable_habr {
                                    if providers_link_parts.contains_key(
                                        HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                            .config_name_value,
                                    ) {
                                        let habr_links = &providers_link_parts
                                            [HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                .config_name_value];
                                        if habr_links.is_empty() {
                                            print_colorful_message(
                                                PrintType::Error,
                                                file!().to_string(),
                                                line!().to_string(),
                                                "habr_links.is_empty".to_string(),
                                            )
                                        } else {
                                            if CONFIG.params.enable_all_providers_prints
                                                && CONFIG.enable_prints.enable_prints_habr
                                            {
                                                println!(
                                                    "{:#?} elements in {:#?} HashMap",
                                                    habr_links.len(),
                                                    &HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type
                                                );
                                            };
                                            let posts_handle = Arc::clone(&posts);
                                            let error_posts_handle = Arc::clone(&error_posts);
                                            threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_habr,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_prints
                                                            .enable_warning_prints_for_habr,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_habr,
                                                    CONFIG.params.enable_all_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_habr_time_measurement,
                                                    &CONFIG.links.habr_link,
                                                    &HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type,
                                                    CONFIG.params.enable_error_prints_handle,
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }));
                                        }
                                    } else {
                                        print_colorful_message(
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!(
                                                "providers_link_parts.contains_key({}) is false",
                                                HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .config_name_value
                                            ),
                                        );
                                    }
                                }
                            } else if provider_name
                                == MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT.config_name_value
                            {
                                if CONFIG.enable_providers.enable_medrxiv {
                                    if providers_link_parts.contains_key(
                                        MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                            .config_name_value,
                                    ) {
                                        let medrxiv_links = &providers_link_parts
                                            [MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                .config_name_value];
                                        if medrxiv_links.is_empty() {
                                            print_colorful_message(
                                                PrintType::Error,
                                                file!().to_string(),
                                                line!().to_string(),
                                                "medrxiv_links.is_empty".to_string(),
                                            )
                                        } else {
                                            if CONFIG.params.enable_all_providers_prints
                                                && CONFIG.enable_prints.enable_prints_medrxiv
                                            {
                                                println!(
                                                    "{:#?} elements in {:#?} HashMap",
                                                    medrxiv_links.len(),
                                                    MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type
                                                );
                                            };
                                            let posts_handle = Arc::clone(&posts);
                                            let error_posts_handle = Arc::clone(&error_posts);
                                            threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG
                                                            .enable_prints
                                                            .enable_prints_medrxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_prints
                                                            .enable_warning_prints_for_medrxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_medrxiv,
                                                    CONFIG.params.enable_all_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_medrxiv_time_measurement,
                                                    &CONFIG.links.medrxiv_link,
                                                    &MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type,
                                                    CONFIG.params.enable_error_prints_handle,
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }));
                                        }
                                    } else {
                                        print_colorful_message(
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!(
                                                "providers_link_parts.contains_key({}) is false",
                                                MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .config_name_value
                                            ),
                                        );
                                    }
                                }
                            } else if provider_name
                                == REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT.config_name_value
                            {
                                if CONFIG.enable_providers.enable_reddit {
                                    if providers_link_parts.contains_key(
                                        REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                            .config_name_value,
                                    ) {
                                        let reddit_links = &providers_link_parts
                                            [REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                .config_name_value];
                                        if reddit_links.is_empty() {
                                            print_colorful_message(
                                                PrintType::Error,
                                                file!().to_string(),
                                                line!().to_string(),
                                                "reddit_links.is_empty".to_string(),
                                            )
                                        } else {
                                            if CONFIG.params.enable_all_providers_prints
                                                && CONFIG.enable_prints.enable_prints_reddit
                                            {
                                                println!(
                                                    "{:#?} elements in {:#?} HashMap",
                                                    reddit_links.len(),
                                                    REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type
                                                );
                                            };
                                            let posts_handle = Arc::clone(&posts);
                                            let error_posts_handle = Arc::clone(&error_posts);
                                            threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG
                                                            .enable_prints
                                                            .enable_prints_reddit,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_prints
                                                            .enable_warning_prints_for_reddit,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_reddit,
                                                    CONFIG.params.enable_all_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_reddit_time_measurement,
                                                    &CONFIG.links.reddit_link,
                                                    &REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                        .provider_kind_enum_type,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_reddit,
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }))
                                        };
                                    } else {
                                        print_colorful_message(
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!(
                                                "providers_link_parts.contains_key({}) is false",
                                                REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .config_name_value
                                            ),
                                        );
                                    }
                                }
                            } else if provider_name
                                == TWITTER_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT.config_name_value
                            {
                                if CONFIG.enable_providers.enable_twitter {
                                    let twitter_links = get_twitter_names();
                                    let twitter_providers = get_twitter_providers_names();
                                    if twitter_links.is_empty() || twitter_providers.is_empty() {
                                        print_colorful_message(
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            "twiter_links.is_empty".to_string(),
                                        )
                                    } else {
                                        if CONFIG.params.enable_all_providers_prints
                                            && CONFIG.enable_prints.enable_prints_twitter
                                        {
                                            println!(
                                                "{:#?} elements in {:#?} HashMap",
                                                twitter_links.len(),
                                                TWITTER_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .provider_kind_enum_type
                                            );
                                        };
                                        let posts_handle = Arc::clone(&posts);
                                        let error_posts_handle = Arc::clone(&error_posts);
                                        threads_vec.push(thread::spawn(move || {
                                            let enum_success_unsuccess_option_posts = rss_part(
                                                CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_twitter,
                                                CONFIG
                                                    .params
                                                    .enable_warning_prints_for_all_providers
                                                    && CONFIG
                                                        .enable_warning_prints
                                                        .enable_warning_prints_for_twitter,
                                                CONFIG.params.enable_error_prints_for_all_providers
                                                    && CONFIG
                                                        .enable_error_prints
                                                        .enable_error_prints_for_twitter,
                                                CONFIG.params.enable_all_time_measurement
                                                    && CONFIG
                                                        .enable_time_measurement
                                                        .enable_twitter_time_measurement,
                                                &CONFIG.links.twitter_link,
                                                &TWITTER_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT
                                                    .provider_kind_enum_type,
                                                CONFIG.params.enable_error_prints_handle,
                                            );
                                            if let Some(success_posts) =
                                                enum_success_unsuccess_option_posts.0
                                            {
                                                let mut posts_handle_locked =
                                                    posts_handle.lock().unwrap();
                                                for value in success_posts {
                                                    posts_handle_locked.push(value);
                                                }
                                            }
                                            if let Some(unsuccess_posts) =
                                                enum_success_unsuccess_option_posts.1
                                            {
                                                let mut error_posts_handle_locked =
                                                    error_posts_handle.lock().unwrap();
                                                for unsuccess_post in unsuccess_posts {
                                                    error_posts_handle_locked.push(unsuccess_post);
                                                }
                                            }
                                        }));
                                    }
                                }
                            } else {
                                panic!("incorrect provider_name {:#?}", provider_name)
                            }
                        }
                        for i in threads_vec {
                            i.join().unwrap();
                        }
                        let posts_done = posts.lock().unwrap().to_vec();
                        let error_posts_done = error_posts.lock().unwrap().to_vec();
                        Some((posts_done, error_posts_done))
                    } else {
                        print_colorful_message(
                            PrintType::Error,
                            file!().to_string(),
                            line!().to_string(),
                            "providers_link_parts is empty".to_string(),
                        );
                        None
                    }
                }
                None => {
                    print_colorful_message(
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "option_providers_link_parts {:#?}",
                            option_providers_link_parts
                        ),
                    );
                    None
                }
            }
        } else {
            print_colorful_message(
                PrintType::WarningLow,
                file!().to_string(),
                line!().to_string(),
                "CONFIG.params.vec_of_provider_names is empty".to_string(),
            );
            None
        }
    } else {
        print_colorful_message(
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "CONFIG.params.enable_all_providers is false".to_string(),
        );
        None
    }
}

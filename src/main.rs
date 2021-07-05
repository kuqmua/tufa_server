mod fetch {
    pub mod parse_github_html;
    pub mod rss_async_write_fetch_error_logs_into_file;
    pub mod rss_async_write_fetch_error_logs_into_files_wrapper;
    pub mod rss_check_available_providers;
    pub mod rss_check_handled_fetch_status_info;
    pub mod rss_check_provider_status;
    pub mod rss_clean_logs_directory;
    pub mod rss_clean_logs_directory_wrapper;
    pub mod rss_divide_to_equal_for_each_provider;
    pub mod rss_fetch_and_parse_provider_data;
    pub mod rss_fetch_link;
    pub mod rss_filter_fetched_and_parsed_posts;
    pub mod rss_handle_error_status_code;
    pub mod rss_handle_unfiltered_posts;
    pub mod rss_logs_create_dir_if_dont_exists;
    pub mod rss_metainfo_fetch_structures;
    pub mod rss_parse_string_into_struct;
    pub mod rss_part;
    pub mod rss_provider_kind_enum;
    pub mod rss_write_error_logs_into_file_for_provider;
    pub mod rss_write_error_logs_into_file_for_provider_wrapper_checker;
    pub mod info_structures {
        pub mod structs_for_parsing {
            pub mod arxiv_struct_for_parsing;
            pub mod biorxiv_struct_for_parsing;
            pub mod github_struct_for_parsing;
            pub mod habr_struct_for_parsing;
            pub mod medrxiv_struct_for_parsing;
            pub mod reddit_struct_for_parsing;
            pub mod twitter_struct_for_parsing;
        }
        pub mod common_rss_structures;
    }
}
mod check_net {
    pub mod check_link;
    pub mod check_link_metainfo_structures;
    pub mod fetch_link;
}
mod overriding {
    pub mod prints;
}
mod authorization {
    pub mod reddit {
        pub mod reddit_authorization;
    }
}

mod async_tokio_wrapper;
mod check_new_posts_threads_parts;
mod entry;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use providers_info_lib::init_mongo_db_and_collections::put_data_in_mongo::put_data_in_mongo;

fn main() {
    put_data_in_mongo(
        &CONFIG.mongo_params.mongo_url,
        &CONFIG.mongo_params.db_name_handle,
        &CONFIG.mongo_params.db_collection_handle_second_part,
        &CONFIG.mongo_params.db_collection_document_field_name_handle,
        &CONFIG.mongo_params.path_to_provider_link_parts_folder,
        CONFIG.mongo_params.vec_of_provider_names.clone(),
        &CONFIG.mongo_params.file_extension,
    );
    // entry::entry();
}

extern crate toml;

use crate::helpers::resource::Resource;

#[derive(Debug, Clone)] //Default,//serde_derive::Serialize, serde_derive::Deserialize
pub struct ConfigStruct {
    pub github_name: String,
    pub github_token: String,

    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,

    pub dbs_enable_initialization: bool,
    pub providers_link_parts_source: Resource,

    pub mongo_first_handle_url_part: String,
    pub mongo_second_handle_url_part: String,
    pub mongo_third_handle_url_part: String,
    pub mongo_fourth_handle_url_part: String,
    pub mongo_fifth_handle_url_part: String,

    pub mongo_login: String,
    pub mongo_password: String,
    pub mongo_ip: String,
    pub mongo_port: String,
    pub mongo_params: String,

    pub mongo_providers_logs_db_name: String,
    pub mongo_providers_logs_db_collection_handle_second_part: String,
    pub mongo_providers_logs_db_collection_document_field_name_handle: String,

    pub is_mongo_initialization_enabled: bool,
    pub is_mongo_initialization_enabled_for_providers: bool,
    pub is_mongo_initialization_enabled_for_arxiv: bool,
    pub is_mongo_initialization_enabled_for_biorxiv: bool,
    pub is_mongo_initialization_enabled_for_github: bool,
    pub is_mongo_initialization_enabled_for_habr: bool,
    pub is_mongo_initialization_enabled_for_medrxiv: bool,
    pub is_mongo_initialization_enabled_for_reddit: bool,
    pub is_mongo_initialization_enabled_for_twitter: bool,

    pub is_mongo_write_error_logs_enabled: bool,
    pub is_mongo_write_error_logs_enabled_for_providers: bool,
    pub is_mongo_write_error_logs_enabled_for_arxiv: bool,
    pub is_mongo_write_error_logs_enabled_for_biorxiv: bool,
    pub is_mongo_write_error_logs_enabled_for_github: bool,
    pub is_mongo_write_error_logs_enabled_for_habr: bool,
    pub is_mongo_write_error_logs_enabled_for_medrxiv: bool,
    pub is_mongo_write_error_logs_enabled_for_reddit: bool,
    pub is_mongo_write_error_logs_enabled_for_twitter: bool,

    pub is_mongo_cleaning_warning_logs_db_enabled: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_providers: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_arxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_biorxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_github: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_habr: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_medrxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_reddit: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_for_twitter: bool,

    pub is_mongo_cleaning_warning_logs_db_collections_enabled: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_providers: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_arxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_biorxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_github: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_habr: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_medrxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_reddit: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_for_twitter: bool,

    pub is_mongo_link_parts_randomize_order_enabled: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_providers: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_arxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_biorxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_github: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_habr: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_medrxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_reddit: bool,
    pub is_mongo_link_parts_randomize_order_enabled_for_twitter: bool,

    pub postgres_first_handle_url_part: String,
    pub postgres_second_handle_url_part: String,
    pub postgres_third_handle_url_part: String,
    pub postgres_fourth_handle_url_part: String,
    pub postgres_fifth_handle_url_part: String,

    pub postgres_login: String,
    pub postgres_password: String,
    pub postgres_ip: String,
    pub postgres_port: String,
    pub postgres_db: String,

    pub is_postgres_initialization_enabled: bool,
    pub is_postgres_initialization_enabled_for_providers: bool,
    pub is_postgres_initialization_enabled_for_arxiv: bool,
    pub is_postgres_initialization_enabled_for_biorxiv: bool,
    pub is_postgres_initialization_enabled_for_github: bool,
    pub is_postgres_initialization_enabled_for_habr: bool,
    pub is_postgres_initialization_enabled_for_medrxiv: bool,
    pub is_postgres_initialization_enabled_for_reddit: bool,
    pub is_postgres_initialization_enabled_for_twitter: bool,

    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub path_to_provider_link_parts_folder: String,
    pub log_file_extension: String,

    pub enable_write_error_logs_in_local_folder: bool,
    pub enable_write_error_logs_in_local_folder_for_provider: bool,
    pub enable_write_error_logs_in_local_folder_for_arxiv: bool,
    pub enable_write_error_logs_in_local_folder_for_biorxiv: bool,
    pub enable_write_error_logs_in_local_folder_for_github: bool,
    pub enable_write_error_logs_in_local_folder_for_habr: bool,
    pub enable_write_error_logs_in_local_folder_for_medrxiv: bool,
    pub enable_write_error_logs_in_local_folder_for_reddit: bool,
    pub enable_write_error_logs_in_local_folder_for_twitter: bool,

    pub enable_cleaning_warning_logs_directory: bool,
    pub enable_cleaning_warning_logs_directory_for_providers: bool,
    pub enable_cleaning_warning_logs_directory_for_arxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_biorxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_github: bool,
    pub enable_cleaning_warning_logs_directory_for_habr: bool,
    pub enable_cleaning_warning_logs_directory_for_medrxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_reddit: bool,
    pub enable_cleaning_warning_logs_directory_for_twitter: bool,

    pub starting_check_link: String,
    pub arxiv_check_link: String,
    pub biorxiv_check_link: String,
    pub github_check_link: String,
    pub habr_check_link: String,
    pub medrxiv_check_link: String,
    pub reddit_check_link: String,
    pub twitter_check_link: String,

    pub enable_providers: bool,
    pub enable_arxiv: bool,
    pub enable_biorxiv: bool,
    pub enable_github: bool,
    pub enable_habr: bool,
    pub enable_medrxiv: bool,
    pub enable_reddit: bool,
    pub enable_twitter: bool,

    pub enable_prints: bool,
    pub enable_prints_for_providers: bool,
    pub enable_prints_arxiv: bool,
    pub enable_prints_biorxiv: bool,
    pub enable_prints_github: bool,
    pub enable_prints_habr: bool,
    pub enable_prints_medrxiv: bool,
    pub enable_prints_reddit: bool,
    pub enable_prints_twitter: bool,

    pub enable_warning_high_prints: bool,
    pub enable_warning_high_prints_for_providers: bool,
    pub enable_warning_high_prints_for_arxiv: bool,
    pub enable_warning_high_prints_for_biorxiv: bool,
    pub enable_warning_high_prints_for_github: bool,
    pub enable_warning_high_prints_for_habr: bool,
    pub enable_warning_high_prints_for_medrxiv: bool,
    pub enable_warning_high_prints_for_reddit: bool,
    pub enable_warning_high_prints_for_twitter: bool,

    pub enable_warning_low_prints: bool,
    pub enable_warning_low_prints_for_providers: bool,
    pub enable_warning_low_prints_for_arxiv: bool,
    pub enable_warning_low_prints_for_biorxiv: bool,
    pub enable_warning_low_prints_for_github: bool,
    pub enable_warning_low_prints_for_habr: bool,
    pub enable_warning_low_prints_for_medrxiv: bool,
    pub enable_warning_low_prints_for_reddit: bool,
    pub enable_warning_low_prints_for_twitter: bool,

    pub enable_success_prints: bool,
    pub enable_success_prints_for_providers: bool,
    pub enable_success_prints_for_arxiv: bool,
    pub enable_success_prints_for_biorxiv: bool,
    pub enable_success_prints_for_github: bool,
    pub enable_success_prints_for_habr: bool,
    pub enable_success_prints_for_medrxiv: bool,
    pub enable_success_prints_for_reddit: bool,
    pub enable_success_prints_for_twitter: bool,

    pub enable_partial_success_prints: bool,
    pub enable_partial_success_prints_for_providers: bool,
    pub enable_partial_success_prints_for_arxiv: bool,
    pub enable_partial_success_prints_for_biorxiv: bool,
    pub enable_partial_success_prints_for_github: bool,
    pub enable_partial_success_prints_for_habr: bool,
    pub enable_partial_success_prints_for_medrxiv: bool,
    pub enable_partial_success_prints_for_reddit: bool,
    pub enable_partial_success_prints_for_twitter: bool,

    pub enable_error_prints: bool,
    pub enable_error_prints_for_providers: bool,
    pub enable_error_prints_for_arxiv: bool,
    pub enable_error_prints_for_biorxiv: bool,
    pub enable_error_prints_for_github: bool,
    pub enable_error_prints_for_habr: bool,
    pub enable_error_prints_for_medrxiv: bool,
    pub enable_error_prints_for_reddit: bool,
    pub enable_error_prints_for_twitter: bool,

    pub enable_time_measurement_prints: bool,
    pub enable_time_measurement_prints_for_providers: bool,
    pub enable_time_measurement_prints_for_arxiv: bool,
    pub enable_time_measurement_prints_for_biorxiv: bool,
    pub enable_time_measurement_prints_for_github: bool,
    pub enable_time_measurement_prints_for_habr: bool,
    pub enable_time_measurement_prints_for_medrxiv: bool,
    pub enable_time_measurement_prints_for_reddit: bool,
    pub enable_time_measurement_prints_for_twitter: bool,

    pub enable_info_prints: bool,
    pub enable_info_prints_for_providers: bool,
    pub enable_info_prints_for_arxiv: bool,
    pub enable_info_prints_for_biorxiv: bool,
    pub enable_info_prints_for_github: bool,
    pub enable_info_prints_for_habr: bool,
    pub enable_info_prints_for_medrxiv: bool,
    pub enable_info_prints_for_reddit: bool,
    pub enable_info_prints_for_twitter: bool,

    pub enable_links_limit: bool,
    pub enable_links_limit_for_providers: bool,
    pub enable_links_limit_for_arxiv: bool,
    pub enable_links_limit_for_biorxiv: bool,
    pub enable_links_limit_for_github: bool,
    pub enable_links_limit_for_habr: bool,
    pub enable_links_limit_for_medrxiv: bool,
    pub enable_links_limit_for_reddit: bool,
    pub enable_links_limit_for_twitter: bool,

    pub enable_common_providers_links_limit: bool,
    pub common_providers_links_limit: i64,
    pub links_limit_for_arxiv: i64,
    pub links_limit_for_biorxiv: i64,
    pub links_limit_for_github: i64,
    pub links_limit_for_habr: i64,
    pub links_limit_for_medrxiv: i64,
    pub links_limit_for_reddit: i64,
    pub links_limit_for_twitter: i64,

    pub error_red: u8,
    pub error_green: u8,
    pub error_blue: u8,
    pub warning_high_red: u8,
    pub warning_high_green: u8,
    pub warning_high_blue: u8,
    pub warning_low_red: u8,
    pub warning_low_green: u8,
    pub warning_low_blue: u8,
    pub success_red: u8,
    pub success_green: u8,
    pub success_blue: u8,
    pub partial_success_red: u8,
    pub partial_success_green: u8,
    pub partial_success_blue: u8,
    pub cleaning_red: u8,
    pub cleaning_green: u8,
    pub cleaning_blue: u8,
    pub time_measurement_red: u8,
    pub time_measurement_green: u8,
    pub time_measurement_blue: u8,
    pub info_red: u8,
    pub info_green: u8,
    pub info_blue: u8,
}

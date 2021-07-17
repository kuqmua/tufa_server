pub const CONFIG_FILE_EXTENSION: &str = ".toml";
pub static VECTOR_OF_MODES: &[&str] = &["Development", "Production", "Testing"];

//its important to have EXACT copy without spaces or Line feed character
pub const CONFIG_CONTENT: &str = r#"[params]
enable_all_providers = true
enable_all_providers_prints = true
enable_warning_high_prints_for_all_providers = true
enable_warning_low_prints_for_all_providers = true
enable_error_prints_for_all_providers = true
enable_all_cleaning_warning_logs_directory = true
enable_prints_handle = true
enable_error_prints_handle = true
warning_logs_directory_name = "warning_logs"
unhandled_success_handled_success_are_there_items_initialized_posts_dir = "unhandled_success_handled_success_are_there_items_initialized_posts"
# // pub const ERROR_LOGS_DIRECTORY_NAME: &str = "error_logs";
enable_all_time_measurement = true
enable_common_time_measurement = true
user_credentials_dummy_handle = "example"
vec_of_provider_names = ["arxiv", "biorxiv", "github", "habr", "medrxiv", "reddit", "twitter"]
#it can be only one of them: "arxiv", "biorxiv", "github", "habr", "medrxiv", "reddit", "twitter" - check project_constants.rs

[mongo_params]
is_cloud = true
db_name_handle = "provider_links"
db_collection_handle_second_part = "_link_parts"
db_collection_document_field_name_handle = "link_part"
path_to_provider_link_parts_folder = "./providers_link_parts/"
file_extension = ".json"
mongo_own_first_handle_url_part = "mongodb://"
mongo_own_second_handle_url_part = ":"
mongo_own_third_handle_url_part = "@"
mongo_own_fourth_handle_url_part = ":"
mongo_cloud_first_handle_url_part = "mongodb+srv://"
mongo_cloud_second_handle_url_part = ":"
mongo_cloud_third_handle_url_part = "@"
mongo_cloud_fourth_handle_url_part = "/"

[enable_providers]
enable_arxiv = true
enable_biorxiv = true
enable_github = true
enable_habr = true
enable_medrxiv = true
enable_reddit = true
enable_twitter = true

[links]
starting_check_link = "https://www.google.com/"
arxiv_link = "https://www.google.com/" # https://arxiv.org/   http://export.arxiv.org/rss/   http://export.arxiv.org/rss/astro-ph.CO
biorxiv_link = "https://www.google.com/" # http://connect.biorxiv.org/
github_link = "https://github.com"
habr_link = "https://www.google.com/" # https://habr.com/ru/
medrxiv_link = "https://www.google.com/" # http://connect.medrxiv.org/
reddit_link = "https://www.reddit.com/"
twitter_link = "https://www.google.com/" # must be not only 1 str but many - twitter and many nitters

[enable_prints]
enable_prints_arxiv = true
enable_prints_biorxiv = true
enable_prints_github = true
enable_prints_habr = true
enable_prints_medrxiv = true
enable_prints_reddit = true
enable_prints_twitter = true

[enable_warning_high_prints]
enable_warning_high_prints_for_arxiv = true
enable_warning_high_prints_for_biorxiv = true
enable_warning_high_prints_for_github = true
enable_warning_high_prints_for_habr = true
enable_warning_high_prints_for_medrxiv = true
enable_warning_high_prints_for_reddit = true
enable_warning_high_prints_for_twitter = true

[enable_warning_low_prints]
enable_warning_low_prints_for_arxiv = true
enable_warning_low_prints_for_biorxiv = true
enable_warning_low_prints_for_github = true
enable_warning_low_prints_for_habr = true
enable_warning_low_prints_for_medrxiv = true
enable_warning_low_prints_for_reddit = true
enable_warning_low_prints_for_twitter = true

[enable_error_prints]
enable_error_prints_for_arxiv = true
enable_error_prints_for_biorxiv = true
enable_error_prints_for_github = true
enable_error_prints_for_habr = true
enable_error_prints_for_medrxiv = true
enable_error_prints_for_reddit = true
enable_error_prints_for_twitter = true

[enable_cleaning_warning_logs_directory]
enable_cleaning_warning_logs_directory_for_arxiv = true
enable_cleaning_warning_logs_directory_for_biorxiv = true
enable_cleaning_warning_logs_directory_for_github = true
enable_cleaning_warning_logs_directory_for_habr = true
enable_cleaning_warning_logs_directory_for_medrxiv = true
enable_cleaning_warning_logs_directory_for_reddit = true
enable_cleaning_warning_logs_directory_for_twitter = true

[enable_time_measurement]
enable_arxiv_time_measurement = true
enable_biorxiv_time_measurement = true
enable_github_time_measurement = true
enable_habr_time_measurement = true
enable_medrxiv_time_measurement = true
enable_reddit_time_measurement = true
enable_twitter_time_measurement = true

[print_colors]
error_red = 255
error_green = 0
error_blue = 0
warning_high_red = 255
warning_high_green = 165
warning_high_blue = 0
warning_low_red = 255
warning_low_green = 255
warning_low_blue = 0
success_red = 0
success_green = 255
success_blue = 0
partial_success_red = 0
partial_success_green = 200
partial_success_blue = 155"#;
//its important to have EXACT copy without spaces or Line feed character
pub const USER_CREDENTIALS_CONTENT: &str = r#"[github_authorization]
github_name = "example"
github_token = "example"

[reddit_authorization]
reddit_user_agent = "example"
reddit_client_id = "example"
reddit_client_secret = "example"
reddit_username = "example"
reddit_password = "example"

[mongo_own_authorization]
mongo_own_login = "example"
mongo_own_password = "example"
mongo_own_ip = "example"
mongo_own_port = "example"

[mongo_cloud_authorization]
mongo_cloud_login = "example"
mongo_cloud_password = "example"
mongo_cloud_cluster_name = "example"
mongo_cloud_cluster_params = "example""#;

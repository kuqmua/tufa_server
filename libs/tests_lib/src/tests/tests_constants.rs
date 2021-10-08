pub const PATH_TO_ENV_FILE: &str = "./../../";
//its important to have EXACT copy without spaces or Line feed character
//todo 
pub const ENV_FILE_CONTENT: &str = r#"# for postgres container 
POSTGRES_USER="example"
POSTGRES_PASSWORD="example"
# for mongo container
MONGO_USERNAME="example"
MONGO_PASSWORD="example"

# github authorization
GITHUB_NAME = "example"
GITHUB_TOKEN = "example"

# reddit authorization 
REDDIT_USER_AGENT = "example"
REDDIT_CLIENT_ID = "example"
REDDIT_CLIENT_SECRET = "example"
REDDIT_USERNAME = "example"
REDDIT_PASSWORD = "example"

# mongo_own_authorization
MONGO_OWN_LOGIN = "example"
MONGO_OWN_PASSWORD = "example"
MONGO_OWN_IP = "example"
MONGO_OWN_PORT = "example"

# mongo_cloud_authorization
MONGO_CLOUD_LOGIN = "example"
MONGO_CLOUD_PASSWORD = "example"
MONGO_CLOUD_CLUSTER_NAME = "example" # or is it more than cluster name?
MONGO_CLOUD_CLUSTER_PARAMS = "example"

# postgres_own_authorization
POSTGRES_OWN_LOGIN = "example"
POSTGRES_OWN_PASSWORD = "example"
POSTGRES_OWN_IP = "example"
POSTGRES_OWN_DB = "example"

# ////////////////////////////////
#[params]
VEC_OF_PROVIDER_NAMES="[arxiv, biorxiv, github, habr, medrxiv, reddit, twitter]"
#it can be only one of them: arxiv, biorxiv, github, habr, medrxiv, reddit, twitter - check project_constants.rs
STARTING_CHECK_LINK="https://www.google.com/"
USER_CREDENTIALS_DUMMY_HANDLE="example"
WARNING_LOGS_DIRECTORY_NAME="warning_logs"
UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR="unhandled_success_handled_success_are_there_items_initialized_posts"
# // pub const ERROR_LOGS_DIRECTORY_NAME: &str="error_logs;
ENABLE_PROVIDERS="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY="true"
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO="true"
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO="true"
ENABLE_TIME_MEASUREMENT="true"
ENABLE_PROVIDER_LINKS_LIMIT="true"
ENABLE_COMMON_PROVIDERS_LINKS_LIMIT="true"
COMMON_PROVIDERS_LINKS_LIMIT="1" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO="true"
ENABLE_PRINTS="true"
ENABLE_ERROR_PRINTS="true"
ENABLE_WARNING_HIGH_PRINTS="true"
ENABLE_WARNING_LOW_PRINTS="true"
ENABLE_SUCCESS_PRINTS="true"
ENABLE_PARTIAL_SUCCESS_PRINTS="true"
ENABLE_TIME_MEASUREMENT_PRINTS="false"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS="true"
ENABLE_INFO_PRINTS="true"
ENABLE_ALL_PROVIDERS_PRINTS="true"
ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS="true"
ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER="false"
ENABLE_WRITE_ERROR_LOGS_IN_MONGO="true"
ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS="true"

# [mongo_params]
IS_CLOUD="true"
PROVIDERS_DB_NAME_HANDLE="provider_links" # rename this into providers_...
PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART="_link_parts" # rename this into providers_...
PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE="link_part" # rename this into providers_...
# todo: move this into few diferent mongo objects
PATH_TO_PROVIDER_LINK_PARTS_FOLDER="./providers_link_parts/"
LOG_FILE_EXTENSION=".json" # rename this into log_...
# todo: move this into few diferent mongo objects
DB_PROVIDERS_LOGS_NAME_HANDLE="logs"
DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART="second_part"
DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE="data"

# mongodb://login:password@127.0.0.1:8888
# [mongo_params.enable_mongo_own_url_parts]
MONGO_OWN_FIRST_HANDLE_URL_PART="mongodb://"
MONGO_OWN_SECOND_HANDLE_URL_PART=":"
MONGO_OWN_THIRD_HANDLE_URL_PART="@"
MONGO_OWN_FOURTH_HANDLE_URL_PART=":"

# mongodb+srv://login:password@db-name.some_random_hash.mongodb.net/cluster_params
# [mongo_params.enable_mongo_cloud_url_parts]
MONGO_CLOUD_FIRST_HANDLE_URL_PART="mongodb+srv://"
MONGO_CLOUD_SECOND_HANDLE_URL_PART=":"
MONGO_CLOUD_THIRD_HANDLE_URL_PART="@"
MONGO_CLOUD_FOURTH_HANDLE_URL_PART="/"

# [mongo_params.enable_initialize_mongo_with_providers_link_parts]
ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS="true"
ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS="true"
ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS="true"
ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS="true"
ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS="true"
ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS="true"
ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS="true"

# #postgres://login:password@127.0.0.1/db_name 
# #todo: where is a port?
#[postgres_params]
POSTGRES_OWN_FIRST_HANDLE_URL_PART="postgres://"
POSTGRES_OWN_SECOND_HANDLE_URL_PART=":"
POSTGRES_OWN_THIRD_HANDLE_URL_PART="@"
POSTGRES_OWN_FOURTH_HANDLE_URL_PART="/"

# [enable_providers]
ENABLE_ARXIV="false"
ENABLE_BIORXIV="false"
ENABLE_GITHUB="false"
ENABLE_HABR="false"
ENABLE_MEDRXIV="false"
ENABLE_REDDIT="false"
ENABLE_TWITTER="true"

#[providers_check_links]
ARXIV_LINK="https://www.google.com/" # https://arxiv.org/   http://export.arxiv.org/rss/   http://export.arxiv.org/rss/astro-ph.CO
BIORXIV_LINK="https://www.google.com/" # http://connect.biorxiv.org/
GITHUB_LINK="https://github.com/"
HABR_LINK="https://www.google.com/" # https://habr.com/ru/
MEDRXIV_LINK="https://www.google.com/" # http://connect.medrxiv.org/
REDDIT_LINK="https://www.reddit.com/"
TWITTER_LINK="https://www.google.com/" # must be not only 1 str but many - twitter and many nitters

#[enable_providers_prints]
ENABLE_PRINTS_ARXIV="true"
ENABLE_PRINTS_BIORXIV="true"
ENABLE_PRINTS_GITHUB="true"
ENABLE_PRINTS_HABR="true"
ENABLE_PRINTS_MEDRXIV="true"
ENABLE_PRINTS_REDDIT="true"
ENABLE_PRINTS_TWITTER="true"

# #[enable_warning_high_providers_prints]
ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV="true"
ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV="true"
ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB="true"
ENABLE_WARNING_HIGH_PRINTS_FOR_HABR="true"
ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV="true"
ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT="true"
ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER="true"

# #[enable_warning_low_providers_prints]
ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV="true"
ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV="true"
ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB="true"
ENABLE_WARNING_LOW_PRINTS_FOR_HABR="true"
ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV="true"
ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT="true"
ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER="true"

# #[enable_error_providers_prints]
ENABLE_ERROR_PRINTS_FOR_ARXIV="true"
ENABLE_ERROR_PRINTS_FOR_BIORXIV="true"
ENABLE_ERROR_PRINTS_FOR_GITHUB="true"
ENABLE_ERROR_PRINTS_FOR_HABR="true"
ENABLE_ERROR_PRINTS_FOR_MEDRXIV="true"
ENABLE_ERROR_PRINTS_FOR_REDDIT="true"
ENABLE_ERROR_PRINTS_FOR_TWITTER="true"

# #[enable_success_providers_prints]
ENABLE_SUCCESS_PRINTS_FOR_ARXIV="true"
ENABLE_SUCCESS_PRINTS_FOR_BIORXIV="true"
ENABLE_SUCCESS_PRINTS_FOR_GITHUB="true"
ENABLE_SUCCESS_PRINTS_FOR_HABR="true"
ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV="true"
ENABLE_SUCCESS_PRINTS_FOR_REDDIT="true"
ENABLE_SUCCESS_PRINTS_FOR_TWITTER="true"

# #[enable_partial_success_providers_prints]
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV="true"
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV="true"
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB="true"
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR="true"
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV="true"
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT="true"
ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER="true"

# #[enable_providers_cleaning_warning_logs_directory]
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT="true"
ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER="true"

# #[enable_providers_cleaning_warning_logs_db_in_mongo]
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB="true"
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR="true"
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT="true"
ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER="true"

# #[enable_providers_cleaning_warning_logs_db_collections_in_mongo]
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB="true"
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR="true"
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV="true"
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT="true"
ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER="true"

# #[enable_providers_time_measurement]
ENABLE_TIME_MEASUREMENT_FOR_ARXIV="true"
ENABLE_TIME_MEASUREMENT_FOR_BIORXIV="true"
ENABLE_TIME_MEASUREMENT_FOR_GITHUB="true"
ENABLE_TIME_MEASUREMENT_FOR_HABR="true"
ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV="true"
ENABLE_TIME_MEASUREMENT_FOR_REDDIT="true"
ENABLE_TIME_MEASUREMENT_FOR_TWITTER="true"

# #[enable_providers_info]
ENABLE_INFO_FOR_ARXIV="true"
ENABLE_INFO_FOR_BIORXIV="true"
ENABLE_INFO_FOR_GITHUB="true"
ENABLE_INFO_FOR_HABR="true"
ENABLE_INFO_FOR_MEDRXIV="true"
ENABLE_INFO_FOR_REDDIT="true"
ENABLE_INFO_FOR_TWITTER="true"

# #[enable_providers_links_limits]
ENABLE_LINKS_LIMIT_FOR_ARXIV="true"
ENABLE_LINKS_LIMIT_FOR_BIORXIV="true"
ENABLE_LINKS_LIMIT_FOR_GITHUB="true"
ENABLE_LINKS_LIMIT_FOR_HABR="true"
ENABLE_LINKS_LIMIT_FOR_MEDRXIV="true"
ENABLE_LINKS_LIMIT_FOR_REDDIT="true"
ENABLE_LINKS_LIMIT_FOR_TWITTER="true"

# #[enable_randomize_order_for_providers_link_parts_for_mongo]
ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO="true"
ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO="true"
ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO="true"
ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO="true"
ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO="true"
ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO="true"
ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO="true"

# #[providers_links_limits]
LINKS_LIMIT_FOR_ARXIV="10" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
LINKS_LIMIT_FOR_BIORXIV="10" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
LINKS_LIMIT_FOR_GITHUB="10" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
LINKS_LIMIT_FOR_HABR="10" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
LINKS_LIMIT_FOR_MEDRXIV="10" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
LINKS_LIMIT_FOR_REDDIT="10" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
LINKS_LIMIT_FOR_TWITTER="10" # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)

# [print_colors]
ERROR_RED="255" # u8
ERROR_GREEN="0" # u8
ERROR_BLUE="0" # u8
WARNING_HIGH_RED="255" # u8
WARNING_HIGH_GREEN="165" # u8
WARNING_HIGH_BLUE="0" # u8
WARNING_LOW_RED="255" # u8
WARNING_LOW_GREEN="255" # u8
WARNING_LOW_BLUE="0" # u8
SUCCESS_RED="0" # u8
SUCCESS_GREEN="255" # u8
SUCCESS_BLUE="0" # u8
PARTIAL_SUCCESS_RED="0" # u8
PARTIAL_SUCCESS_GREEN="200" # u8
PARTIAL_SUCCESS_BLUE="155" # u8
CLEANING_RED="230" # u8
CLEANING_GREEN="234" # u8
CLEANING_BLUE="255" # u8
TIME_MEASUREMENT_RED="255" # u8
TIME_MEASUREMENT_GREEN="153" # u8
TIME_MEASUREMENT_BLUE="170" # u8
INFO_RED="240" # u8
INFO_GREEN="240" # u8
INFO_BLUE="240" # u8"#;
//todo 
//its important to have EXACT copy without spaces or Line feed character
pub const DOCKER_COMPOSE_CONTENT: &str = r#"version: '3.8'
services:
  postgres_service:
    container_name: postgres_container
    image: postgres:latest
    restart: always
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: postgres
    volumes:
      - ./postgresql_volume:/data/db
    ports:
      - 5432:5432
  mongodb_service:
    container_name: mongodb_container
    image: mongo:latest
    restart: always
    environment:
      MONGO_INITDB_ROOT_USERNAME: ${MONGO_USERNAME}
      MONGO_INITDB_ROOT_PASSWORD: ${MONGO_PASSWORD}
    ports:
      - 27017:27017
    volumes:
      - ./mongodb_volume:/data/db
  tufa_backend_service:
    container_name: tufa_backend-container
    image: tufa_backend-image
    restart: always
    ports:
      - 8000:8000
    env_file:
      - ./.env
    environment: 
      PROJECT_RUN_MODE: ${PROJECT_RUN_MODE}
      GITHUB_NAME: ${GITHUB_NAME}
      GITHUB_TOKEN: ${GITHUB_TOKEN}
      REDDIT_USER_AGENT: ${REDDIT_USER_AGENT}
      REDDIT_CLIENT_ID: ${REDDIT_CLIENT_ID}
      REDDIT_CLIENT_SECRET: ${REDDIT_CLIENT_SECRET}
      REDDIT_USERNAME: ${REDDIT_USERNAME}
      REDDIT_PASSWORD: ${REDDIT_PASSWORD}
      MONGO_OWN_LOGIN: ${MONGO_OWN_LOGIN}
      MONGO_OWN_PASSWORD: ${MONGO_OWN_PASSWORD}
      MONGO_OWN_IP: ${MONGO_OWN_IP}
      MONGO_OWN_PORT: ${MONGO_OWN_PORT}
      MONGO_CLOUD_LOGIN: ${MONGO_CLOUD_LOGIN}
      MONGO_CLOUD_PASSWORD: ${MONGO_CLOUD_PASSWORD}
      MONGO_CLOUD_CLUSTER_NAME: ${MONGO_CLOUD_CLUSTER_NAME}
      MONGO_CLOUD_CLUSTER_PARAMS: ${MONGO_CLOUD_CLUSTER_PARAMS}
      POSTGRES_OWN_LOGIN: ${POSTGRES_OWN_LOGIN}
      POSTGRES_OWN_PASSWORD: ${POSTGRES_OWN_PASSWORD}
      POSTGRES_OWN_IP: ${POSTGRES_OWN_IP}
      POSTGRES_OWN_DB: ${POSTGRES_OWN_DB}
      VEC_OF_PROVIDER_NAMES: ${VEC_OF_PROVIDER_NAMES}
      #"{"strings":["arxiv", "biorxiv", "github", "habr", "medrxiv", "reddit", "twitter"]}"
      # #it can be only one of them: arxiv, biorxiv, github, habr, medrxiv, reddit, twitter - check project_constants.rs
      STARTING_CHECK_LINK: ${STARTING_CHECK_LINK}
      USER_CREDENTIALS_DUMMY_HANDLE: ${USER_CREDENTIALS_DUMMY_HANDLE}
      WARNING_LOGS_DIRECTORY_NAME: ${WARNING_LOGS_DIRECTORY_NAME}
      UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR: ${UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR}
      ENABLE_PROVIDERS: ${ENABLE_PROVIDERS}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO}
      ENABLE_TIME_MEASUREMENT: ${ENABLE_TIME_MEASUREMENT}
      ENABLE_PROVIDER_LINKS_LIMIT: ${ENABLE_PROVIDER_LINKS_LIMIT}
      ENABLE_COMMON_PROVIDERS_LINKS_LIMIT: ${ENABLE_COMMON_PROVIDERS_LINKS_LIMIT}
      COMMON_PROVIDERS_LINKS_LIMIT: ${COMMON_PROVIDERS_LINKS_LIMIT} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO}
      ENABLE_PRINTS: ${ENABLE_PRINTS}
      ENABLE_ERROR_PRINTS: ${ENABLE_ERROR_PRINTS}
      ENABLE_WARNING_HIGH_PRINTS: ${ENABLE_WARNING_HIGH_PRINTS}
      ENABLE_WARNING_LOW_PRINTS: ${ENABLE_WARNING_LOW_PRINTS}
      ENABLE_SUCCESS_PRINTS: ${ENABLE_SUCCESS_PRINTS}
      ENABLE_PARTIAL_SUCCESS_PRINTS: ${ENABLE_PARTIAL_SUCCESS_PRINTS}
      ENABLE_TIME_MEASUREMENT_PRINTS: ${ENABLE_TIME_MEASUREMENT_PRINTS}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS}
      ENABLE_INFO_PRINTS: ${ENABLE_INFO_PRINTS}
      ENABLE_ALL_PROVIDERS_PRINTS: ${ENABLE_ALL_PROVIDERS_PRINTS}
      ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER: ${ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER}
      ENABLE_WRITE_ERROR_LOGS_IN_MONGO: ${ENABLE_WRITE_ERROR_LOGS_IN_MONGO}
      ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS}
      # [mongo_params]
      IS_CLOUD: ${IS_CLOUD}
      PROVIDERS_DB_NAME_HANDLE: ${PROVIDERS_DB_NAME_HANDLE} # rename this into providers_...
      PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART: ${PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART} # rename this into providers_...
      PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE: ${PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE} # rename this into providers_...
      # todo: move this into few diferent mongo objects
      PATH_TO_PROVIDER_LINK_PARTS_FOLDER: ${PATH_TO_PROVIDER_LINK_PARTS_FOLDER}
      LOG_FILE_EXTENSION: ${LOG_FILE_EXTENSION} # rename this into log_...
      # todo: move this into few diferent mongo objects
      DB_PROVIDERS_LOGS_NAME_HANDLE: ${DB_PROVIDERS_LOGS_NAME_HANDLE}
      DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART: ${DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART}
      DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE: ${DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE}
      # mongodb://login:password@127.0.0.1:8888
      # [mongo_params.enable_mongo_own_url_parts]
      MONGO_OWN_FIRST_HANDLE_URL_PART: ${MONGO_OWN_FIRST_HANDLE_URL_PART}
      MONGO_OWN_SECOND_HANDLE_URL_PART: ${MONGO_OWN_SECOND_HANDLE_URL_PART}
      MONGO_OWN_THIRD_HANDLE_URL_PART: ${MONGO_OWN_THIRD_HANDLE_URL_PART}
      MONGO_OWN_FOURTH_HANDLE_URL_PART: ${MONGO_OWN_FOURTH_HANDLE_URL_PART}
      # mongodb+srv://login:password@db-name.some_random_hash.mongodb.net/cluster_params
      # [mongo_params.enable_mongo_cloud_url_parts]
      MONGO_CLOUD_FIRST_HANDLE_URL_PART: ${MONGO_CLOUD_FIRST_HANDLE_URL_PART}
      MONGO_CLOUD_SECOND_HANDLE_URL_PART: ${MONGO_CLOUD_SECOND_HANDLE_URL_PART}
      MONGO_CLOUD_THIRD_HANDLE_URL_PART: ${MONGO_CLOUD_THIRD_HANDLE_URL_PART}
      MONGO_CLOUD_FOURTH_HANDLE_URL_PART: ${MONGO_CLOUD_FOURTH_HANDLE_URL_PART}
      # [mongo_params.enable_initialize_mongo_with_providers_link_parts]
      ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS}
      # postgres://login:password@127.0.0.1/db_name 
      # #todo: where is a port?
      #[postgres_params]
      POSTGRES_OWN_FIRST_HANDLE_URL_PART: ${POSTGRES_OWN_FIRST_HANDLE_URL_PART}
      POSTGRES_OWN_SECOND_HANDLE_URL_PART: ${POSTGRES_OWN_SECOND_HANDLE_URL_PART}
      POSTGRES_OWN_THIRD_HANDLE_URL_PART: ${POSTGRES_OWN_THIRD_HANDLE_URL_PART}
      POSTGRES_OWN_FOURTH_HANDLE_URL_PART: ${POSTGRES_OWN_FOURTH_HANDLE_URL_PART}
      # [enable_providers]
      ENABLE_ARXIV: ${ENABLE_ARXIV}
      ENABLE_BIORXIV: ${ENABLE_BIORXIV}
      ENABLE_GITHUB: ${ENABLE_GITHUB}
      ENABLE_HABR: ${ENABLE_HABR}
      ENABLE_MEDRXIV: ${ENABLE_MEDRXIV}
      ENABLE_REDDIT: ${ENABLE_REDDIT}
      ENABLE_TWITTER: ${ENABLE_TWITTER}
      #[providers_check_links]
      ARXIV_LINK: ${ARXIV_LINK} # https://arxiv.org/   http://export.arxiv.org/rss/   http://export.arxiv.org/rss/astro-ph.CO
      BIORXIV_LINK: ${BIORXIV_LINK} # http://connect.biorxiv.org/
      GITHUB_LINK: ${GITHUB_LINK}
      HABR_LINK: ${HABR_LINK} # https://habr.com/ru/
      MEDRXIV_LINK: ${MEDRXIV_LINK} # http://connect.medrxiv.org/
      REDDIT_LINK: ${REDDIT_LINK}
      TWITTER_LINK: ${TWITTER_LINK} # must be not only 1 str but many - twitter and many nitters
      #[enable_providers_prints]
      ENABLE_PRINTS_ARXIV: ${ENABLE_PRINTS_ARXIV}
      ENABLE_PRINTS_BIORXIV: ${ENABLE_PRINTS_BIORXIV}
      ENABLE_PRINTS_GITHUB: ${ENABLE_PRINTS_GITHUB}
      ENABLE_PRINTS_HABR: ${ENABLE_PRINTS_HABR}
      ENABLE_PRINTS_MEDRXIV: ${ENABLE_PRINTS_MEDRXIV}
      ENABLE_PRINTS_REDDIT: ${ENABLE_PRINTS_REDDIT}
      ENABLE_PRINTS_TWITTER: ${ENABLE_PRINTS_TWITTER}
      # [enable_warning_high_providers_prints]
      ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV: ${ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV}
      ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV: ${ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV}
      ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB: ${ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB}
      ENABLE_WARNING_HIGH_PRINTS_FOR_HABR: ${ENABLE_WARNING_HIGH_PRINTS_FOR_HABR}
      ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV: ${ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV}
      ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT: ${ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT}
      ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER: ${ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER}
      # [enable_warning_low_providers_prints]
      ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV: ${ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV}
      ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV: ${ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV}
      ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB: ${ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB}
      ENABLE_WARNING_LOW_PRINTS_FOR_HABR: ${ENABLE_WARNING_LOW_PRINTS_FOR_HABR}
      ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV: ${ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV}
      ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT: ${ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT}
      ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER: ${ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER}
      # [enable_error_providers_prints]
      ENABLE_ERROR_PRINTS_FOR_ARXIV: ${ENABLE_ERROR_PRINTS_FOR_ARXIV}
      ENABLE_ERROR_PRINTS_FOR_BIORXIV: ${ENABLE_ERROR_PRINTS_FOR_BIORXIV}
      ENABLE_ERROR_PRINTS_FOR_GITHUB: ${ENABLE_ERROR_PRINTS_FOR_GITHUB}
      ENABLE_ERROR_PRINTS_FOR_HABR: ${ENABLE_ERROR_PRINTS_FOR_HABR}
      ENABLE_ERROR_PRINTS_FOR_MEDRXIV: ${ENABLE_ERROR_PRINTS_FOR_MEDRXIV}
      ENABLE_ERROR_PRINTS_FOR_REDDIT: ${ENABLE_ERROR_PRINTS_FOR_REDDIT}
      ENABLE_ERROR_PRINTS_FOR_TWITTER: ${ENABLE_ERROR_PRINTS_FOR_TWITTER}
      # [enable_success_providers_prints]
      ENABLE_SUCCESS_PRINTS_FOR_ARXIV: ${ENABLE_SUCCESS_PRINTS_FOR_ARXIV}
      ENABLE_SUCCESS_PRINTS_FOR_BIORXIV: ${ENABLE_SUCCESS_PRINTS_FOR_BIORXIV}
      ENABLE_SUCCESS_PRINTS_FOR_GITHUB: ${ENABLE_SUCCESS_PRINTS_FOR_GITHUB}
      ENABLE_SUCCESS_PRINTS_FOR_HABR: ${ENABLE_SUCCESS_PRINTS_FOR_HABR}
      ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV: ${ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV}
      ENABLE_SUCCESS_PRINTS_FOR_REDDIT: ${ENABLE_SUCCESS_PRINTS_FOR_REDDIT}
      ENABLE_SUCCESS_PRINTS_FOR_TWITTER: ${ENABLE_SUCCESS_PRINTS_FOR_TWITTER}
      # [enable_partial_success_providers_prints]
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER}
      # [enable_providers_cleaning_warning_logs_directory]
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER}
      # [enable_providers_cleaning_warning_logs_db_in_mongo]
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER}
      # [enable_providers_cleaning_warning_logs_db_collections_in_mongo]
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER}
      #[enable_providers_time_measurement]
      ENABLE_TIME_MEASUREMENT_FOR_ARXIV: ${ENABLE_TIME_MEASUREMENT_FOR_ARXIV}
      ENABLE_TIME_MEASUREMENT_FOR_BIORXIV: ${ENABLE_TIME_MEASUREMENT_FOR_BIORXIV}
      ENABLE_TIME_MEASUREMENT_FOR_GITHUB: ${ENABLE_TIME_MEASUREMENT_FOR_GITHUB}
      ENABLE_TIME_MEASUREMENT_FOR_HABR: ${ENABLE_TIME_MEASUREMENT_FOR_HABR}
      ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV: ${ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV}
      ENABLE_TIME_MEASUREMENT_FOR_REDDIT: ${ENABLE_TIME_MEASUREMENT_FOR_REDDIT}
      ENABLE_TIME_MEASUREMENT_FOR_TWITTER: ${ENABLE_TIME_MEASUREMENT_FOR_TWITTER}
      #[enable_providers_info]
      ENABLE_INFO_FOR_ARXIV: ${ENABLE_INFO_FOR_ARXIV}
      ENABLE_INFO_FOR_BIORXIV: ${ENABLE_INFO_FOR_BIORXIV}
      ENABLE_INFO_FOR_GITHUB: ${ENABLE_INFO_FOR_GITHUB}
      ENABLE_INFO_FOR_HABR: ${ENABLE_INFO_FOR_HABR}
      ENABLE_INFO_FOR_MEDRXIV: ${ENABLE_INFO_FOR_MEDRXIV}
      ENABLE_INFO_FOR_REDDIT: ${ENABLE_INFO_FOR_REDDIT}
      ENABLE_INFO_FOR_TWITTER: ${ENABLE_INFO_FOR_TWITTER}
      #[enable_providers_links_limits]
      ENABLE_LINKS_LIMIT_FOR_ARXIV: ${ENABLE_LINKS_LIMIT_FOR_ARXIV}
      ENABLE_LINKS_LIMIT_FOR_BIORXIV: ${ENABLE_LINKS_LIMIT_FOR_BIORXIV}
      ENABLE_LINKS_LIMIT_FOR_GITHUB: ${ENABLE_LINKS_LIMIT_FOR_GITHUB}
      ENABLE_LINKS_LIMIT_FOR_HABR: ${ENABLE_LINKS_LIMIT_FOR_HABR}
      ENABLE_LINKS_LIMIT_FOR_MEDRXIV: ${ENABLE_LINKS_LIMIT_FOR_MEDRXIV}
      ENABLE_LINKS_LIMIT_FOR_REDDIT: ${ENABLE_LINKS_LIMIT_FOR_REDDIT}
      ENABLE_LINKS_LIMIT_FOR_TWITTER: ${ENABLE_LINKS_LIMIT_FOR_TWITTER}
      #[enable_randomize_order_for_providers_link_parts_for_mongo]
      ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO}
      #[providers_links_limits]
      LINKS_LIMIT_FOR_ARXIV: ${LINKS_LIMIT_FOR_ARXIV} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_BIORXIV: ${LINKS_LIMIT_FOR_BIORXIV} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_GITHUB: ${LINKS_LIMIT_FOR_GITHUB} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_HABR: ${LINKS_LIMIT_FOR_HABR} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_MEDRXIV: ${LINKS_LIMIT_FOR_MEDRXIV} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_REDDIT: ${LINKS_LIMIT_FOR_REDDIT} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_TWITTER: ${LINKS_LIMIT_FOR_TWITTER} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      # [print_colors]
      ERROR_RED: ${ERROR_RED} # u8
      ERROR_GREEN: ${ERROR_GREEN} # u8
      ERROR_BLUE: ${ERROR_BLUE} # u8
      WARNING_HIGH_RED: ${WARNING_HIGH_RED} # u8
      WARNING_HIGH_GREEN: ${WARNING_HIGH_GREEN} # u8
      WARNING_HIGH_BLUE: ${WARNING_HIGH_BLUE} # u8
      WARNING_LOW_RED: ${WARNING_LOW_RED} # u8
      WARNING_LOW_GREEN: ${WARNING_LOW_GREEN} # u8
      WARNING_LOW_BLUE: ${WARNING_LOW_BLUE} # u8
      # WARNING_RED: ${WARNING_RED} # u8
      # WARNING_GREEN: ${WARNING_GREEN} # u8
      # WARNING_BLUE: ${WARNING_BLUE} # u8
      PARTIAL_SUCCESS_RED: ${PARTIAL_SUCCESS_RED} # u8
      PARTIAL_SUCCESS_GREEN: ${PARTIAL_SUCCESS_GREEN} # u8
      PARTIAL_SUCCESS_BLUE: ${PARTIAL_SUCCESS_BLUE} # u8
      CLEANING_RED: ${CLEANING_RED} # u8
      CLEANING_GREEN: ${CLEANING_GREEN} # u8
      CLEANING_BLUE: ${CLEANING_BLUE} # u8
      TIME_MEASUREMENT_RED: ${TIME_MEASUREMENT_RED} # u8
      TIME_MEASUREMENT_GREEN: ${TIME_MEASUREMENT_GREEN} # u8
      TIME_MEASUREMENT_BLUE: ${TIME_MEASUREMENT_BLUE} # u8
      INFO_RED: ${INFO_RED} # u8
      INFO_GREEN: ${INFO_GREEN} # u8
      INFO_BLUE: ${INFO_BLUE} # u8
volumes:
  postgresql_volume:
  mongodb_volume:"#;

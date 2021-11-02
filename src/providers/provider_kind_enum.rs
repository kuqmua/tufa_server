use std::collections::HashMap;

use crate::config_mods::config::CONFIG;
use crate::constants::project_constants::ARXIV_NAME_TO_CHECK;
use crate::constants::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::constants::project_constants::GITHUB_NAME_TO_CHECK;
use crate::constants::project_constants::HABR_NAME_TO_CHECK;
use crate::constants::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::constants::project_constants::REDDIT_NAME_TO_CHECK;
use crate::constants::project_constants::TWITTER_NAME_TO_CHECK;
use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// use crate::providers::providers_info::get_providers_link_parts::get_providers_link_parts_as_hashmap;

// use crate::helpers::resource::Resource;

// use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::prints::print_type_enum::PrintType;

use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::mongo_integration::mongo_get_possible_aggregation_with_randomization_doc_for_provider::mongo_get_possible_aggregation_with_randomization_doc_for_provider;
use crate::mongo_integration::mongo_possibly_get_documents_as_string_vector::mongo_possibly_get_documents_as_string_vector;
use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;

#[derive(
    EnumVariantCount,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
)]
pub enum ProviderKind {
    Arxiv,
    Biorxiv,
    Github,
    Habr,
    Medrxiv,
    Reddit,
    Twitter,
}

impl ProviderKind {
    pub fn get_string_name(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => ARXIV_NAME_TO_CHECK,
            ProviderKind::Biorxiv => BIORXIV_NAME_TO_CHECK,
            ProviderKind::Github => GITHUB_NAME_TO_CHECK,
            ProviderKind::Habr => HABR_NAME_TO_CHECK,
            ProviderKind::Medrxiv => MEDRXIV_NAME_TO_CHECK,
            ProviderKind::Reddit => REDDIT_NAME_TO_CHECK,
            ProviderKind::Twitter => TWITTER_NAME_TO_CHECK,
        }
    }
    pub fn get_mongo_collection_name(provider_kind: ProviderKind) -> String {
        let name = ProviderKind::get_string_name(provider_kind);
        format!(
            "{}{}",
            name,
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part
        )
    }
    pub fn is_enabled(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.enable_providers.enable_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_providers.enable_biorxiv,
            ProviderKind::Github => CONFIG.enable_providers.enable_github,
            ProviderKind::Habr => CONFIG.enable_providers.enable_habr,
            ProviderKind::Medrxiv => CONFIG.enable_providers.enable_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_providers.enable_reddit,
            ProviderKind::Twitter => CONFIG.enable_providers.enable_twitter,
        }
    }
    pub fn is_mongo_initialization_enabled(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_arxiv_link_parts
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_biorxiv_link_parts
            }
            ProviderKind::Github => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_github_link_parts
            }
            ProviderKind::Habr => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_habr_link_parts
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_medrxiv_link_parts
            }
            ProviderKind::Reddit => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_reddit_link_parts
            }
            ProviderKind::Twitter => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_twitter_link_parts
            }
        }
    }
    pub fn stringify(provider_kind: ProviderKind) -> &'static str {
        match provider_kind {
            ProviderKind::Arxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Biorxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Github => stringify!(ProviderKind::Arxiv),
            ProviderKind::Habr => stringify!(ProviderKind::Arxiv),
            ProviderKind::Medrxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Reddit => stringify!(ProviderKind::Arxiv),
            ProviderKind::Twitter => stringify!(ProviderKind::Arxiv),
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn get_enabled_string_name_vec() -> Vec<&'static str> {
        let mut string_name_vec: Vec<&'static str> = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in
            ProviderKind::iter().filter(|element| ProviderKind::is_enabled(*element))
        {
            string_name_vec.push(ProviderKind::get_string_name(provider_kind));
        }
        string_name_vec
    }
    pub fn get_mongo_initialization_vec() -> Vec<&'static str> {
        let mut vec_of_filtered_provider_names: Vec<&'static str> =
            Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|element| ProviderKind::is_mongo_initialization_enabled(*element))
        {
            vec_of_filtered_provider_names.push(ProviderKind::get_string_name(provider_kind))
        }
        vec_of_filtered_provider_names
    }
    pub fn into_vec() -> Vec<ProviderKind> {
        let mut provider_kind_vec = Vec::with_capacity(ENUM_LENGTH);
        for provider_kind in ProviderKind::iter() {
            provider_kind_vec.push(provider_kind);
        }
        provider_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, ProviderKind)> {
        let mut provider_kind_vec = Vec::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            provider_kind_vec.push((ProviderKind::get_string_name(provider_kind), provider_kind));
        }
        provider_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, ProviderKind> {
        //its String coz legacy
        let mut config_provider_string_to_enum_struct_hasmap: HashMap<&'static str, ProviderKind> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter() {
            config_provider_string_to_enum_struct_hasmap
                .insert(ProviderKind::get_string_name(provider_kind), provider_kind);
        }
        config_provider_string_to_enum_struct_hasmap
    }
    pub fn get_links_limit_for_provider(provider_kind: ProviderKind) -> i64 {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.providers_links_limits.links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.providers_links_limits.links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.providers_links_limits.links_limit_for_github,
            ProviderKind::Habr => CONFIG.providers_links_limits.links_limit_for_habr,
            ProviderKind::Medrxiv => CONFIG.providers_links_limits.links_limit_for_medrxiv,
            ProviderKind::Reddit => CONFIG.providers_links_limits.links_limit_for_reddit,
            ProviderKind::Twitter => CONFIG.providers_links_limits.links_limit_for_twitter,
        }
    }
    pub fn is_provider_kind_string_exists(potential_provider_kind_string: &str) -> bool {
        for provider_kind in ProviderKind::iter() {
            let provider_kind_string = ProviderKind::get_string_name(provider_kind);
            if provider_kind_string == potential_provider_kind_string {
                return true;
            }
        }
        false
    }
    pub fn get_provider_kind_array_from_string_vec(
        providers_vec_of_strings: Vec<String>,
    ) -> Vec<ProviderKind> {
        let mut provider_kind_vec: Vec<ProviderKind> =
            Vec::with_capacity(providers_vec_of_strings.len());
        for potential_provider_string in providers_vec_of_strings {
            match potential_provider_string.as_ref() {
                ARXIV_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Arxiv),
                BIORXIV_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Biorxiv),
                GITHUB_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Github),
                HABR_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Habr),
                MEDRXIV_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Medrxiv),
                REDDIT_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Reddit),
                TWITTER_NAME_TO_CHECK => provider_kind_vec.push(ProviderKind::Twitter),
                _ => {
                    //todo: cannot use print_colorful_message coz cyclic package dependency with prints_lib
                    panic!("potential_provider_string is not defined")
                }
            }
        }
        provider_kind_vec
    }
    ////////////////////

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn mongo_get_provider_link_parts_as_bson_string(
        provider_kind: ProviderKind,
    ) -> Result<Vec<String>, mongodb::error::Error> {
        //todo maybe option vec string
        let client_options = ClientOptions::parse(mongo_get_db_url()).await?;
        let client_result = Client::with_options(client_options);
        let vec_of_strings_to_return: Vec<String>;
        match client_result {
            Ok(client) => {
                //declare db name. there is no create db method in mongo
                let db = client.database(&CONFIG.mongo_params.providers_db_name_handle);
                let mut needed_db_collection: Option<String> = None;
                for collection_name in db.list_collection_names(None).await? {
                    if collection_name == *ProviderKind::get_mongo_collection_name(provider_kind) {
                        needed_db_collection = Some(collection_name);
                    }
                }
                match needed_db_collection {
                    Some(collection_name) => {
                        let collection = db.collection(&collection_name);
                        let documents_number_result = collection.count_documents(None, None).await;
                        match documents_number_result {
                            Ok(documents_number) => {
                                if documents_number > 0 {
                                    //rewrite as PrintType::Info or something
                                    print_colorful_message(
                                        None,
                                        PrintType::Success,
                                        file!().to_string(),
                                        line!().to_string(),
                                        format!("collection.count_documents {}", documents_number),
                                    );
                                    let option_aggregation_stage_1_get_docs_in_random_order_with_limit: Option<Document>;
                                    if CONFIG.params.enable_provider_links_limit {
                                        if CONFIG.params.enable_common_providers_links_limit {
                                            if CONFIG.params.enable_randomize_order_for_providers_link_parts_for_mongo {
                                                option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$sample" : {"size": CONFIG.params.common_providers_links_limit }});
                                            }
                                            else {
                                                option_aggregation_stage_1_get_docs_in_random_order_with_limit = Some(doc! { "$limit" :  CONFIG.params.common_providers_links_limit });
                                            }
                                        } else {
                                            option_aggregation_stage_1_get_docs_in_random_order_with_limit = mongo_get_possible_aggregation_with_randomization_doc_for_provider(
                                                        provider_kind
                                            );
                                        }
                                    } else {
                                        option_aggregation_stage_1_get_docs_in_random_order_with_limit = None;
                                    }
                                    // let aggregation_stage_1_get_docs_in_random_order_with_limit =
                                    //     doc! { "$sample" : {"size": 5 }};
                                    // let aggregation_stage_2_get_docs_with_limit = doc! { "$limit": 5 };
                                    let result = mongo_possibly_get_documents_as_string_vector(
                                        collection,
                                        &CONFIG.mongo_params.providers_db_collection_document_field_name_handle,
                                        option_aggregation_stage_1_get_docs_in_random_order_with_limit,
                                    )
                                    .await;
                                    match result {
                                        Ok(vec_of_strings) => {
                                            vec_of_strings_to_return = vec_of_strings
                                        }
                                        Err(e) => {
                                            vec_of_strings_to_return = Vec::new();
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningLow,
                                                file!().to_string(),
                                                line!().to_string(),
                                                format!("mongo_possibly_get_documents_as_string_vector error {:#?}", e),
                                            );
                                        }
                                    }
                                } else {
                                    vec_of_strings_to_return = Vec::new();
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningLow,
                                        file!().to_string(),
                                        line!().to_string(),
                                        format!("documents_number is {}", documents_number),
                                    );
                                }
                            }
                            Err(e) => {
                                vec_of_strings_to_return = Vec::new();
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("collection.count_documents error {:#?}", e),
                                );
                            }
                        }
                    }
                    None => {
                        vec_of_strings_to_return = Vec::new();
                        print_colorful_message(
                            None,
                            PrintType::WarningLow,
                            file!().to_string(),
                            line!().to_string(),
                            "needed_db_collection is None".to_string(),
                        );
                    }
                }
            }
            Err(e) => {
                vec_of_strings_to_return = Vec::new();
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    format!("Client::with_options error {:#?}", e),
                );
            }
        }
        Ok(vec_of_strings_to_return)
    }
    pub fn enable_links_limit_for(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.enable_providers_links_limits.enable_links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_providers_links_limits.enable_links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_providers_links_limits.enable_links_limit_for_github,
            ProviderKind::Habr => CONFIG.enable_providers_links_limits.enable_links_limit_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_providers_links_limits.enable_links_limit_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_providers_links_limits.enable_links_limit_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_providers_links_limits.enable_links_limit_for_twitter,
        }
    }
    pub fn enable_randomize_order_mongo_link_parts_for(provider_kind: ProviderKind) -> bool {
        match provider_kind {
            ProviderKind::Arxiv => CONFIG.enable_randomize_order_for_providers_link_parts_for_mongo.enable_randomize_order_for_arxiv_link_parts_for_mongo,
            ProviderKind::Biorxiv => CONFIG.enable_randomize_order_for_providers_link_parts_for_mongo.enable_randomize_order_for_biorxiv_link_parts_for_mongo,
            ProviderKind::Github => CONFIG.enable_randomize_order_for_providers_link_parts_for_mongo.enable_randomize_order_for_github_link_parts_for_mongo,
            ProviderKind::Habr => CONFIG.enable_randomize_order_for_providers_link_parts_for_mongo.enable_randomize_order_for_habr_link_parts_for_mongo,
            ProviderKind::Medrxiv => CONFIG.enable_randomize_order_for_providers_link_parts_for_mongo.enable_randomize_order_for_medrxiv_link_parts_for_mongo,
            ProviderKind::Reddit => CONFIG.enable_randomize_order_for_providers_link_parts_for_mongo.enable_randomize_order_for_reddit_link_parts_for_mongo,
            ProviderKind::Twitter => CONFIG.enable_randomize_order_for_providers_link_parts_for_mongo.enable_randomize_order_for_twitter_link_parts_for_mongo,
        }
    }
}

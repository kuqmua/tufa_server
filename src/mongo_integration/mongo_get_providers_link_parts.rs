use std::collections::HashMap;
use std::fmt;

use mongodb::{bson::Document, options::ClientOptions, Client};

use crate::mongo_integration::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector;
use crate::{
    config_mods::lazy_static_config::CONFIG, traits::provider_kind_trait::ProviderKindTrait,
};

use crate::{
    mongo_integration::mongo_get_db_url::mongo_get_db_url,
    providers::provider_kind_enum::ProviderKind,
};

use futures::future::join_all;

use super::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorError;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct MongoGetProvidersLinkPartsError {
    pub source: Box<MongoGetProvidersLinkPartsErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum MongoGetProvidersLinkPartsErrorEnum {
    ClientOptionsParse(ClientOptionsParseError),
    ClientWithOptions(ClientWithOptionsError),
    ListCollectionNames(ListCollectionNamesError),
    NoSuchCollections(HashMap<ProviderKind, String>),
    GetDocuments(HashMap<ProviderKind, MongoGetDocumentsAsStringVectorError>),
}

#[derive(Debug)]
pub struct ClientOptionsParseError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct ClientWithOptionsError {
    pub source: mongodb::error::Error,
}

#[derive(Debug)]
pub struct ListCollectionNamesError {
    pub source: mongodb::error::Error,
}

#[deny(clippy::indexing_slicing)] //, clippy::unwrap_used
pub async fn mongo_get_providers_link_parts(
) -> Result<HashMap<ProviderKind, Vec<String>>, MongoGetProvidersLinkPartsError> {
    match ClientOptions::parse(mongo_get_db_url()).await {
        Err(e) => {
            return Err(MongoGetProvidersLinkPartsError {
                source: Box::new(MongoGetProvidersLinkPartsErrorEnum::ClientOptionsParse(
                    ClientOptionsParseError { source: e },
                )),
            })
        }
        Ok(client_options) => {
            match Client::with_options(client_options) {
                Err(e) => {
                    return Err(MongoGetProvidersLinkPartsError {
                        source: Box::new(MongoGetProvidersLinkPartsErrorEnum::ClientWithOptions(
                            ClientWithOptionsError { source: e },
                        )),
                    })
                }
                Ok(client) => {
                    let db = client.database(&CONFIG.mongo_providers_link_parts_db_name);
                    match db.list_collection_names(None).await {
                        Err(e) => {
                            return Err(MongoGetProvidersLinkPartsError {
                                source: Box::new(
                                    MongoGetProvidersLinkPartsErrorEnum::ListCollectionNames(
                                        ListCollectionNamesError { source: e },
                                    ),
                                ),
                            })
                        }
                        Ok(vec_collection_names) => {
                            let no_collection_error_hashmap =
                                ProviderKind::get_enabled_providers_vec()
                                    .into_iter()
                                    .filter_map(|pk| {
                                        let collection_name = pk.get_mongo_log_collection_name();
                                        if !vec_collection_names.contains(&collection_name) {
                                            return Some((pk, collection_name));
                                        }
                                        None
                                    })
                                    .collect::<HashMap<ProviderKind, String>>();
                            if !no_collection_error_hashmap.is_empty() {
                                return Err(MongoGetProvidersLinkPartsError {
                                    source: Box::new(
                                        MongoGetProvidersLinkPartsErrorEnum::NoSuchCollections(
                                            no_collection_error_hashmap,
                                        ),
                                    ),
                                });
                            }
                            let result_get_documents_hashmap =
                                join_all(ProviderKind::get_enabled_providers_vec().iter().map(|pk| async {
                                    let pk_cloned = pk.clone();
                                    (
                                        pk_cloned,
                                        mongo_get_documents_as_string_vector(
                                            db.collection::<Document>(&pk.get_mongo_log_collection_name()),
                                            &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle,
                                            ProviderKind::get_mongo_provider_link_parts_aggregation(&pk_cloned),
                                        )
                                        .await,
                                    )
                                }
                            ))
                            .await;
                            let mut success_hashmap: HashMap<
                                ProviderKind,
                                Vec<String>,
                            > = HashMap::new();
                            let mut error_hashmap: HashMap<
                                ProviderKind,
                                MongoGetDocumentsAsStringVectorError,
                            > = HashMap::new();
                            for (pk, result) in result_get_documents_hashmap.into_iter() {
                                match result {
                                    Err(e) => {
                                        error_hashmap.insert(pk, e);
                                    }
                                    Ok(vec) => {
                                        success_hashmap.insert(pk, vec);
                                    }
                                }
                            }
                            if !error_hashmap.is_empty() {
                                return Err(MongoGetProvidersLinkPartsError {
                                    source: Box::new(
                                        MongoGetProvidersLinkPartsErrorEnum::GetDocuments(
                                            error_hashmap,
                                        ),
                                    ),
                                });
                            }
                            Ok(success_hashmap)
                        }
                    }
                }
            }
        }
    }
}

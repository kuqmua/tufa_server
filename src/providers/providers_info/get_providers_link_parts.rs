use std::collections::HashMap;

use crate::helpers::resource::Resource;

use crate::mongo_integration::mongo_get_providers_link_parts::mongo_get_providers_link_parts;
use crate::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsError;

// use crate::postgres_integration::postgres_get_providers_link_parts::postgres_get_providers_link_parts;
// use crate::postgres_integration::postgres_get_providers_link_parts::PostgresGetProviderLinksError;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;

#[derive(Debug)]
pub enum GetLinkPartsError {
    Local(GetLocalProvidersLinkPartsError),
    Mongodb(MongoGetProvidersLinkPartsError),
    // PostgreSql(PostgresGetProviderLinksError),
    PostgreSql,
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts(
    resource: &Resource,
) -> Result<HashMap<ProviderKind, Vec<String>>, GetLinkPartsError> {
    match resource {
        Resource::Local => match get_local_providers_link_parts().await {
            Err(error_hashmap) => Err(GetLinkPartsError::Local(error_hashmap)),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        Resource::Mongodb => match mongo_get_providers_link_parts().await {
            Err(e) => Err(GetLinkPartsError::Mongodb(e)),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        // Resource::PostgreSql => match postgres_get_providers_link_parts().await {
        //     Err(e) => Err(GetLinkPartsError::PostgreSql(e)),
        //     Ok(success_hashmap) => Ok(success_hashmap),
        // },
        Resource::PostgreSql => todo!(),
    }
}

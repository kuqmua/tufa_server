use super::provider_kind::provider_kind_enum::ProviderKind;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;

#[derive(Debug)]
pub enum CheckProvidersLinkPartsEmptyError {
    Full {
        where_was: WhereWas,
    },
    Partially {
        source: Vec<ProviderKind>,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn check_providers_link_parts_on_empty(
    providers_link_parts: HashMap<ProviderKind, Vec<String>>,
) -> Result<HashMap<ProviderKind, Vec<String>>, Box<CheckProvidersLinkPartsEmptyError>> {
    if providers_link_parts.is_empty() {
        return Err(Box::new(CheckProvidersLinkPartsEmptyError::Full {
            where_was: WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                location: *core::panic::Location::caller(),
            },
        }));
    }
    let mut non_empty_providers_link_parts = HashMap::with_capacity(providers_link_parts.len());
    let mut empty_providers_link_parts = HashMap::with_capacity(providers_link_parts.len());
    for (pk, vec) in providers_link_parts {
        if vec.is_empty() {
            empty_providers_link_parts.insert(pk, vec);
        } else {
            non_empty_providers_link_parts.insert(pk, vec);
        }
    }
    if !empty_providers_link_parts.is_empty() {
        let mut pk_vec = Vec::with_capacity(empty_providers_link_parts.len());
        for pk in empty_providers_link_parts.keys() {
            pk_vec.push(*pk);
        }
        return Err(Box::new(CheckProvidersLinkPartsEmptyError::Partially {
            source: pk_vec,
            where_was: WhereWas {
                time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("cannot convert time to unix_epoch"),
                location: *core::panic::Location::caller(),
            },
        }));
    }
    Ok(non_empty_providers_link_parts)
}

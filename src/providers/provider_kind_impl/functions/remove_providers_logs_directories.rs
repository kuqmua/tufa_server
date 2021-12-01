use crate::providers::provider_kind_enum::CleanLogsDirError;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use std::collections::HashMap;

use strum::IntoEnumIterator;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn remove_providers_logs_directories(
    ) -> Result<(), HashMap<ProviderKind, CleanLogsDirError>> {
        let mut result_hashmap: HashMap<ProviderKind, CleanLogsDirError> =
            HashMap::with_capacity(ProviderKind::get_length());
        for provider_kind in ProviderKind::iter()
            .filter(|provider_kind| provider_kind.is_cleaning_warning_logs_directory_enable())
        {
            if let Err(e) = provider_kind.remove_logs_directory() {
                result_hashmap.insert(provider_kind, e);
            }
        }
        if result_hashmap.is_empty() {
            Ok(())
        } else {
            Err(result_hashmap)
        }
    }
}

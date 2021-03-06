use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use crate::{
    config_mods::lazy_static_config::CONFIG,
    providers::provider_kind::provider_kind_enum::ProviderKind,
};
use mongodb::bson::doc;
use mongodb::bson::Document;

impl ProviderKind {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub fn get_mongo_provider_link_parts_aggregation(&self) -> Option<Document> {
        if CONFIG.is_links_limit_enabled_providers
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(doc! { "$sample" : {"size": CONFIG.links_limit_providers }});
        } else if CONFIG.is_links_limit_enabled_providers {
            Some(doc! { "$limit" :  CONFIG.links_limit_providers });
        } else if self.is_links_limit_enabled()
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(doc! { "$sample" : {"size": self.links_limit() }});
        } else if self.is_links_limit_enabled() {
            Some(doc! { "$limit" : self.links_limit() });
        } else if self.is_mongo_link_parts_randomize_order_enabled() {
            println!("todo: mongo sample(randomized aggregation) only works if size is valid number. No aggregation applied");
            return None;
        }
        None
    }
}

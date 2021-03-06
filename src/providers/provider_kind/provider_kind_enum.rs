use crate::config_mods::lazy_static_config::CONFIG;
// use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;
use convert_case::Case;
use convert_case::Casing;
use enum_extention::EnumExtenstion;
use mongodb::bson::doc;
use provider_kind_from_config::ProviderKindFromConfigTraitDerive;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct RemoveDirError {
    pub error: std::io::Error,
}

#[derive(Debug)]
pub enum CleanLogsDirError {
    PathIsNotDir { path: String },
    CannotRemoveDir { error: RemoveDirError },
}
impl From<String> for CleanLogsDirError {
    fn from(e: String) -> Self {
        CleanLogsDirError::PathIsNotDir { path: e }
    }
}
impl From<std::io::Error> for CleanLogsDirError {
    fn from(e: std::io::Error) -> Self {
        CleanLogsDirError::CannotRemoveDir {
            error: RemoveDirError { error: e },
        }
    }
}

#[derive(
    ProviderKindFromConfigTraitDerive,
    EnumExtenstion,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum ProviderKind {
    Arxiv,
    Biorxiv,
    Github,
    Habr,
    Medrxiv,
    Reddit,
    Twitter,
}

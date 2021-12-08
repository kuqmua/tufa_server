use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::env_var_enum::EnvVar;

use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvStringVar {
    fn get_env_name(&self) -> &'static str {
        match self {
            //todo: reordering
            Self::GithubName => EnvVar::GithubName.get_env_name(),
            Self::GithubToken => EnvVar::GithubToken.get_env_name(),

            Self::RedditUserAgent => EnvVar::RedditUserAgent.get_env_name(),
            Self::RedditClientId => EnvVar::RedditClientId.get_env_name(),
            Self::RedditClientSecret => EnvVar::RedditClientSecret.get_env_name(),
            Self::RedditUsername => EnvVar::RedditUsername.get_env_name(),
            Self::RedditPassword => EnvVar::RedditPassword.get_env_name(),

            Self::MongoFirstHandleUrlPart => EnvVar::MongoFirstHandleUrlPart.get_env_name(),
            Self::MongoSecondHandleUrlPart => EnvVar::MongoSecondHandleUrlPart.get_env_name(),
            Self::MongoThirdHandleUrlPart => EnvVar::MongoThirdHandleUrlPart.get_env_name(),
            Self::MongoFourthHandleUrlPart => EnvVar::MongoFourthHandleUrlPart.get_env_name(),
            Self::MongoFifthHandleUrlPart => EnvVar::MongoFifthHandleUrlPart.get_env_name(),

            Self::MongoLogin => EnvVar::MongoLogin.get_env_name(),
            Self::MongoPassword => EnvVar::MongoPassword.get_env_name(),
            Self::MongoIp => EnvVar::MongoIp.get_env_name(),
            Self::MongoPort => EnvVar::MongoPort.get_env_name(),
            Self::MongoParams => EnvVar::MongoParams.get_env_name(),

            Self::MongoProvidersLogsDbName => EnvVar::MongoProvidersLogsDbName.get_env_name(),
            Self::MongoProvidersLogsDbCollectionHandleSecondPart => {
                EnvVar::MongoProvidersLogsDbCollectionHandleSecondPart.get_env_name()
            }
            Self::MongoProvidersLogsDbCollectionDocumentFieldNameHandle => {
                EnvVar::MongoProvidersLogsDbCollectionDocumentFieldNameHandle.get_env_name()
            }

            Self::PostgresFirstHandleUrlPart => EnvVar::PostgresFirstHandleUrlPart.get_env_name(),
            Self::PostgresSecondHandleUrlPart => EnvVar::PostgresSecondHandleUrlPart.get_env_name(),
            Self::PostgresThirdHandleUrlPart => EnvVar::PostgresThirdHandleUrlPart.get_env_name(),
            Self::PostgresFourthHandleUrlPart => EnvVar::PostgresFourthHandleUrlPart.get_env_name(),
            Self::PostgresFifthHandleUrlPart => EnvVar::PostgresFifthHandleUrlPart.get_env_name(),

            Self::PostgresLogin => EnvVar::PostgresLogin.get_env_name(),
            Self::PostgresPassword => EnvVar::PostgresPassword.get_env_name(),
            Self::PostgresIp => EnvVar::PostgresIp.get_env_name(),
            Self::PostgresPort => EnvVar::PostgresPort.get_env_name(),
            Self::PostgresDb => EnvVar::PostgresDb.get_env_name(),

            Self::WarningLogsDirectoryName => EnvVar::WarningLogsDirectoryName.get_env_name(),
            Self::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => {
                EnvVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir
                    .get_env_name()
            }
            Self::PathToProviderLinkPartsFolder => {
                EnvVar::PathToProviderLinkPartsFolder.get_env_name()
            }
            Self::LogFileExtension => EnvVar::LogFileExtension.get_env_name(),

            Self::StartingCheckLink => EnvVar::StartingCheckLink.get_env_name(),
            Self::ArxivCheckLink => EnvVar::ArxivCheckLink.get_env_name(),
            Self::BiorxivCheckLink => EnvVar::BiorxivCheckLink.get_env_name(),
            Self::GithubCheckLink => EnvVar::GithubCheckLink.get_env_name(),
            Self::HabrCheckLink => EnvVar::HabrCheckLink.get_env_name(),
            Self::MedrxivCheckLink => EnvVar::MedrxivCheckLink.get_env_name(),
            Self::RedditCheckLink => EnvVar::RedditCheckLink.get_env_name(),
            Self::TwitterCheckLink => EnvVar::TwitterCheckLink.get_env_name(),
        }
    }
    fn into_array() -> &'static [Self] {
        Self::all_variants()
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, Self> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, Self> =
            HashMap::with_capacity(Self::get_length());
        for env_var_name_kind_kind in Self::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                env_var_name_kind_kind.get_env_name(),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, Self)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push((env_var_name_kind.get_env_name(), env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_vec() -> Vec<Self> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
}

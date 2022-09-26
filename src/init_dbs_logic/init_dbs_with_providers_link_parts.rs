use crate::init_dbs_logic::init_mongo::init_mongo;
use crate::init_dbs_logic::init_mongo::InitMongoError;
use crate::init_dbs_logic::init_postgres::init_postgres;
use crate::init_dbs_logic::init_postgres::PostgresInitError;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_get_source_for_source_error_enum::ImplGetSourceForSourceErrorEnum;
use impl_get_source_for_struct_with_method::ImplGetSourceForStructWithMethod;
use impl_get_where_was_one_or_many_for_enum::ImplGetWhereWasOneOrManyForEnum;
use impl_get_where_was_one_or_many_for_struct_with_source_enum_method::ImplGetWhereWasOneOrManyForStructWithSourceEnumMethod;
use init_error::InitError;
use init_error_with_tracing::InitErrorWithTracing;
use tufa_common::traits::get_bunyan_with_additional_where_was::GetBunyanWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplGetSourceForStructWithMethod,
    InitErrorWithTracing,
    ImplGetWhereWasOneOrManyForStructWithSourceEnumMethod,
)]
pub struct InitDbsProvidersLinkPartsError {
    source: InitDbsProvidersLinkPartsErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetWhereWasOneOrManyForEnum, ImplGetSourceForSourceErrorEnum)]
pub enum InitDbsProvidersLinkPartsErrorEnum {
    GetLocalProvidersLinkParts(GetLocalProvidersLinkPartsError),
    PostgresInit(PostgresInitError),
    MongoInit(InitMongoError),
    MongoAndPostgresInit {
        mongo: InitMongoError,
        postgres: PostgresInitError,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_dbs_with_providers_link_parts(
    should_trace: bool,
) -> Result<(), Box<InitDbsProvidersLinkPartsError>> {
    match get_local_providers_link_parts(false).await {
        Err(e) => Err(Box::new(
            InitDbsProvidersLinkPartsError::init_error_with_possible_trace(
                InitDbsProvidersLinkPartsErrorEnum::GetLocalProvidersLinkParts(*e),
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(providers_json_local_data_hashmap) => {
            let providers_json_local_data_hashmap_clone = providers_json_local_data_hashmap.clone();
            let (mongo_insert_data_option_result, postgres_insert_data_option_result) = tokio::join!(
                async {
                    match CONFIG.is_mongo_initialization_enabled {
                        false => None,
                        true => Some(init_mongo(providers_json_local_data_hashmap, false).await),
                    }
                },
                async {
                    match CONFIG.is_postgres_initialization_enabled {
                        false => None,
                        true => Some(
                            init_postgres(providers_json_local_data_hashmap_clone, false).await,
                        ),
                    }
                }
            );
            match (
                mongo_insert_data_option_result,
                postgres_insert_data_option_result,
            ) {
                (None, None) => (),
                (None, Some(pg_result)) => {
                    if let Err(e) = pg_result {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsErrorEnum::PostgresInit(*e),
                                WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                                &CONFIG.source_place_type,
                                &GIT_INFO.data,
                                should_trace,
                            ),
                        ));
                    }
                }
                (Some(mongo_result), None) => {
                    if let Err(e) = mongo_result {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsErrorEnum::MongoInit(*e),
                                WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                                &CONFIG.source_place_type,
                                &GIT_INFO.data,
                                should_trace,
                            ),
                        ));
                    }
                }
                (Some(mongo_result), Some(pg_result)) => match (mongo_result, pg_result) {
                    (Ok(_), Ok(_)) => (),
                    (Ok(_), Err(e)) => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsErrorEnum::PostgresInit(*e),
                                WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                                &CONFIG.source_place_type,
                                &GIT_INFO.data,
                                should_trace,
                            ),
                        ));
                    }
                    (Err(e), Ok(_)) => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsErrorEnum::MongoInit(*e),
                                WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                                &CONFIG.source_place_type,
                                &GIT_INFO.data,
                                should_trace,
                            ),
                        ));
                    }
                    (Err(mongo_error), Err(postgres_error)) => {
                        return Err(Box::new(
                            InitDbsProvidersLinkPartsError::init_error_with_possible_trace(
                                InitDbsProvidersLinkPartsErrorEnum::MongoAndPostgresInit {
                                    mongo: *mongo_error,
                                    postgres: *postgres_error,
                                },
                                WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                                &CONFIG.source_place_type,
                                &GIT_INFO.data,
                                should_trace,
                            ),
                        ));
                    }
                },
            }
            Ok(())
        }
    }
}

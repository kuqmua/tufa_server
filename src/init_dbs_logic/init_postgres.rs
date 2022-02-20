use std::collections::HashMap;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;

use crate::helpers::where_was::WhereWas;

// use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::postgres_delete_all_from_providers_link_parts_tables;
use crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesError;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::postgres_insert_link_parts_into_providers_tables;
use crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesError;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::postgres_check_providers_link_parts_tables_are_empty;
use crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyError;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists;
use crate::postgres_integration::postgres_create_providers_tables_if_not_exists::PostgresCreateProvidersDbsError;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;
use crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError;

#[derive(Debug)]
pub struct PostgresInitError {
    pub source: Box<PostgresInitErrorEnum>,
}

#[derive(Debug)]
pub enum PostgresInitErrorEnum {
    EstablishConnection {
        source: sqlx::Error,
        where_was: WhereWas,
    },
    CreateTableQueries {
        source: PostgresCreateProvidersDbsError,
        where_was: WhereWas,
    },
    CheckProviderLinksTablesAreEmpty {
        source: PostgresCheckProvidersLinkPartsTablesEmptyError,
        where_was: WhereWas,
    },
    DeleteAllFromProvidersTables {
        source: PostgresDeleteAllFromProvidersTablesError,
        where_was: WhereWas,
    },
    CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength {
        source: PostgresCheckProvidersLinksTablesLengthRowsEqualInitializationDataLengthError,
        where_was: WhereWas,
    },
    InsertLinkPartsIntoProvidersTables {
        source: PostgresInsertLinkPartsIntoProvidersTablesError,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn init_postgres(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), PostgresInitError> {
    match PgPoolOptions::new()
        .max_connections(providers_json_local_data_hashmap.len() as u32)
        .connect_timeout(Duration::from_millis(10000)) //todo add timeout constant or env var
        .connect(&postgres_get_db_url())
        .await
    {
        Err(e) => Err(PostgresInitError {
            source: Box::new(PostgresInitErrorEnum::EstablishConnection {
                source: e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }),
        }),
        Ok(pool) => {
            if let Err(e) = postgres_create_providers_tables_if_not_exists(
                &providers_json_local_data_hashmap,
                &pool,
            )
            .await
            {
                return Err(PostgresInitError {
                    source: Box::new(PostgresInitErrorEnum::CreateTableQueries {
                        source: e,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                });
            }
            if let Err(e) = postgres_check_providers_link_parts_tables_are_empty(
                &providers_json_local_data_hashmap,
                &pool,
            )
            .await
            {
                return Err(PostgresInitError {
                    source: Box::new(PostgresInitErrorEnum::CheckProviderLinksTablesAreEmpty {
                        source: e,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                });
            }
            if let Err(e) = postgres_delete_all_from_providers_link_parts_tables(
                &providers_json_local_data_hashmap,
                &pool,
            )
            .await
            {
                return Err(PostgresInitError {
                    source: Box::new(PostgresInitErrorEnum::DeleteAllFromProvidersTables {
                        source: e,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                });
            }
            // if let Err(e) = postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
            //     &providers_json_local_data_hashmap,
            //     &pool,
            // )
            // .await {
            //     return Err(PostgresInitError{
            //         source: Box::new(PostgresInitErrorEnum::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength{
            //             source: e,
            //             file: file!(),
            //             line: line!(),
            //             column: column!(),
            //         })
            //     });
            // }
            if let Err(e) = postgres_insert_link_parts_into_providers_tables(
                &providers_json_local_data_hashmap,
                &pool,
            )
            .await
            {
                return Err(PostgresInitError {
                    source: Box::new(PostgresInitErrorEnum::InsertLinkPartsIntoProvidersTables {
                        source: e,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                });
            }
            Ok(())
        }
    }
}

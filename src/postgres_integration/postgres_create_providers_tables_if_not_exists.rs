use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::where_was::WhereWas;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PostgresCreateProvidersDbsError {
    pub source: HashMap<ProviderKind, sqlx::Error>,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_create_providers_tables_if_not_exists(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    db: &Pool<Postgres>,
) -> Result<(), Box<PostgresCreateProvidersDbsError>> {
    let table_creation_error_hashmap = join_all(
        providers_json_local_data_hashmap.keys().map(|pk| async {
            let query_string = format!(
                "CREATE TABLE IF NOT EXISTS {} (id integer GENERATED ALWAYS AS IDENTITY NOT NULL, link_part text, PRIMARY KEY (id));",
                pk.get_postgres_table_name()
            );
            (*pk, sqlx::query(&query_string).execute(db).await)
        })).await
        .into_iter()
        .filter_map(|(pk, result)| {
            if let Err(e) = result {
                return Some((pk, e));
            }
            None
        })
        .collect::<HashMap<ProviderKind, sqlx::Error>>();
    if !table_creation_error_hashmap.is_empty() {
        return Err(Box::new(PostgresCreateProvidersDbsError {
            source: table_creation_error_hashmap,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}

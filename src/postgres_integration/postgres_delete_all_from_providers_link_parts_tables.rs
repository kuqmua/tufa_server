pub async fn postgres_delete_all_from_providers_link_parts_tables<'a>(
    providers_json_local_data_hashmap: &std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
    pool: &sqlx::Pool<sqlx::Postgres>,
    should_trace: bool,
) -> Result<(), Box<tufa_common::server::postgres::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesErrorNamed<'a>>> {
    let delete_from_tables_error_hashmap =
        futures::future::join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
            let query_string = format!("DELETE FROM {} ;", {
                use tufa_common::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods;
                pk.get_postgres_table_name()
            });
            (*pk, sqlx::query(&query_string).execute(pool).await)
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| {
            if let Err(e) = result {
                return Some((pk.to_string(), e));
            }
            None
        })
        .collect::<std::collections::HashMap<std::string::String, sqlx::Error>>();
    if !delete_from_tables_error_hashmap.is_empty() {
        return Err(Box::new(
            tufa_common::server::postgres::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesErrorNamed::DeleteTables { 
                error_hashmap: delete_from_tables_error_hashmap, 
                code_occurence: tufa_common::code_occurence!()
            }
        ));
    }
    Ok(())
}

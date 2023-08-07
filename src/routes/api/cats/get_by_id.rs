pub(crate) async fn get_by_id(
    path_parameters_extraction_result: Result<
        axum::extract::Path<tufa_common::repositories_types::tufa_server::routes::api::cats::GetByIdPathParameters>,
        axum::extract::rejection::PathRejection,
    >,
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> impl axum::response::IntoResponse {
    let path_parameters = match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
        tufa_common::repositories_types::tufa_server::routes::api::cats::GetByIdPathParameters,
        tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants
    >::try_extract_value(
        path_parameters_extraction_result,
        &app_info_state
    ) {
        Ok(path_parameters) => path_parameters,
        Err(err) => {
            return err;
        },
    };
    println!("get_by_id path_parameters id {}", path_parameters.id);
    let bigserial_id = match tufa_common::server::postgres::bigserial::Bigserial::try_from_i64(
        path_parameters.id,
    ) {
        Ok(bigserial_id) => bigserial_id,
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::Bigserial { 
                bigserial: e, 
                code_occurence: tufa_common::code_occurence!()
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                app_info_state.as_ref()
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error);
        }
    };
    match sqlx::query_as!(
        tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
        "SELECT * FROM cats WHERE id = $1",
        *bigserial_id.bigserial()
        // *path_parameters.id.bigserial()
    )
    .fetch_one(&*app_info_state.get_postgres_pool())
    .await
    {
        Ok(value) => tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::Desirable(value),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetById::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                app_info_state.as_ref()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::get_by_id::TryGetByIdResponseVariants::from(error)
        }
    }
}
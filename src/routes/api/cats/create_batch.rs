pub(crate) async fn create_batch(
    app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
    payload_extraction_result: Result<
        axum::Json<
            Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CreateBatchPayloadElement>,
        >,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = tufa_common::repositories_types::tufa_server::routes::api::cats::CreateBatchParameters {
        payload: match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
            Vec<tufa_common::repositories_types::tufa_server::routes::api::cats::CreateBatchPayloadElement>,
            tufa_common::repositories_types::tufa_server::routes::api::cats::create_batch::TryCreateBatchResponseVariants
        >::try_extract_value(
            payload_extraction_result,
            &app_info_state
        ) {
            Ok(payload) => payload,
            Err(err) => {
                return err;
            },
        },
    };
    println!("{parameters:#?}");
    parameters.prepare_and_execute_query(&app_info_state).await
}
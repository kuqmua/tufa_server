// pub(crate) async fn update<'a>(
//     //todo how to check with type system what http request function params and route path query and payload params are same?
//     path_parameters_extraction_result: Result<
//         axum::extract::Path<
//             tufa_common::repositories_types::tufa_server::routes::api::cats::UpdatePath,
//         >,
//         axum::extract::rejection::PathRejection,
//     >,
//     app_info_state: axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
//     payload_extraction_result: Result<
//         axum::Json<tufa_common::repositories_types::tufa_server::routes::api::cats::UpdatePayload>,
//         axum::extract::rejection::JsonRejection,
//     >,
// ) -> impl axum::response::IntoResponse {
//     let parameters =
//         tufa_common::repositories_types::tufa_server::routes::api::cats::UpdateParameters {
//             path: match tufa_common::server::routes::helpers::path_extractor_error::PathValueResultExtractor::<
//                 tufa_common::repositories_types::tufa_server::routes::api::cats::UpdatePath,
//                 tufa_common::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants
//             >::try_extract_value(
//                 path_parameters_extraction_result,
//                 &app_info_state
//             ) {
//                 Ok(path_parameters) => path_parameters,
//                 Err(err) => {
//                     return err;
//                 },
//             },
//             payload: match tufa_common::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
//                 tufa_common::repositories_types::tufa_server::routes::api::cats::UpdatePayload,
//                 tufa_common::repositories_types::tufa_server::routes::api::cats::update::TryUpdateResponseVariants
//             >::try_extract_value(
//                 payload_extraction_result,
//                 &app_info_state
//             ) {
//                 Ok(payload) => payload,
//                 Err(err) => {
//                     return err;
//                 },
//             },
//         };
//     println!("{parameters:#?}");
//     parameters.prepare_and_execute_query(&app_info_state).await
// }
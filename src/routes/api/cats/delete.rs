pub(crate) async fn delete_axum<'a>(
    axum::extract::Query(query_parameters): axum::extract::Query<tufa_common::repositories_types::tufa_server::routes::api::cats::delete::DeleteQueryParameters>,
    axum::extract::State(app_info): axum::extract::State<tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
) -> tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants {
    println!(
        "delete name {:?}, color {:?}",
        query_parameters.name, query_parameters.color
    );
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::NoParameters {
                no_parameters: std::string::String::from("no parameters provided"),
                code_occurence: tufa_common::code_occurence!(),
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.get_config(),
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error);
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE color = $1",
                color,
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE name = $1",
                name,
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE name = $1 AND color = $2",
                name,
                color
            )
            .fetch_all(&*app_info.get_postgres_pool())
            .await
        }
    };
    match query_result {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::DesirableType(()),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.get_config()
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error)
        }
    }
}

#[actix_web::delete("/")]
pub(crate) async fn delete<'a>(
    _project_commit_extractor: tufa_common::server::extractors::project_commit_extractor::ProjectCommitExtractor,
    query_parameters: actix_web::web::Query<tufa_common::repositories_types::tufa_server::routes::api::cats::delete::DeleteQueryParameters>,
    app_info: actix_web::web::Data<
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
    >,
) -> actix_web::HttpResponse {
    println!(
        "delete name {:?}, color {:?}",
        query_parameters.name, query_parameters.color
    );
    let query_result = match (&query_parameters.name, &query_parameters.color) {
        (None, None) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::NoParameters {
                no_parameters: std::string::String::from("no parameters provided"),
                code_occurence: tufa_common::code_occurence!(),
            };
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error,
                &app_info.config,
            );
            return tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error).into();
        }
        (None, Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE color = $1",
                color,
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), None) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE name = $1",
                name,
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
        (Some(name), Some(color)) => {
            sqlx::query_as!(
                tufa_common::repositories_types::tufa_server::routes::api::cats::Cat,
                "DELETE FROM cats WHERE name = $1 AND color = $2",
                name,
                color
            )
            .fetch_all(&app_info.postgres_pool)
            .await
        }
    };
    match query_result {
        Ok(_) => tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::DesirableType(()).into(),
        Err(e) => {
            let error = tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDelete::from(e);
            tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                &error, 
                &app_info.config
            );
            tufa_common::repositories_types::tufa_server::routes::api::cats::delete::TryDeleteResponseVariants::from(error).into()
        }
    }
}
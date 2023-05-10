#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() -> Result<(), Box<tufa_common::repositories_types::tufa_server::startup::ApplicationBuildErrorEnum>> {
    let configuration = tufa_common::repositories_types::tufa_server::configuration::Settings {
        database: tufa_common::repositories_types::tufa_server::configuration::DatabaseSettings {
            host: crate::global_variables::runtime::config::CONFIG.postgres_ip.clone(),
            port: crate::global_variables::runtime::config::CONFIG.postgres_port,
            username: crate::global_variables::runtime::config::CONFIG.postgres_login.clone(),
            password: secrecy::Secret::new(crate::global_variables::runtime::config::CONFIG.postgres_password.clone()),
            database_name: crate::global_variables::runtime::config::CONFIG.postgres_db.clone(),
            require_ssl: crate::global_variables::runtime::config::CONFIG.require_ssl,
        },
        application: tufa_common::repositories_types::tufa_server::configuration::ApplicationSettings {
            port: crate::global_variables::runtime::config::CONFIG.server_port,
            host: crate::global_variables::runtime::config::CONFIG.server_ip.clone(),
            base_url: crate::global_variables::runtime::config::CONFIG.base_url.clone(),
            hmac_secret: secrecy::Secret::new(crate::global_variables::runtime::config::CONFIG.hmac_secret.clone()),
        },
        email_client: tufa_common::repositories_types::tufa_server::configuration::EmailClientSettings {
            base_url: crate::global_variables::runtime::config::CONFIG.base_url.clone(),
            sender_email: "test@gmail.com".to_string(),
            authorization_token: secrecy::Secret::new("my-secret-token".to_string()),
            timeout_milliseconds: 10000,
        },
        redis_uri: secrecy::Secret::new(tufa_common::server::redis::get_redis_url::get_redis_url(&{
            use std::ops::Deref;
            crate::global_variables::runtime::config::CONFIG.deref()
        })),
    };
    let application = match tufa_common::repositories_types::tufa_server::startup::Application::build(configuration.clone()).await {
        Ok(app) => app,
        Err(e) => return Err(e),
    };
    let application_task = tokio::spawn(application.run_until_stopped()).await;
    //remove this coz too much spam
    // match application_task {
    //     Ok(_) => todo!(),
    //     Err(_) => todo!(),
    // }
    // let worker_task = tokio::spawn(crate::issue_delivery_worker::run_worker_until_stopped(configuration));
    // tokio::select! {
    //     o = application_task => report_exit("API", o),
    //     o = worker_task => report_exit("Background worker", o),
    // };
    Ok(())
}

// fn report_exit(task_name: &str, outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, tokio::task::JoinError>) {
//     match outcome {
//         Ok(Ok(())) => {
//             tracing::info!("{} has exited", task_name)
//         }
//         Ok(Err(e)) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{} failed",
//                 task_name
//             )
//         }
//         Err(e) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{}' task failed to complete",
//                 task_name
//             )
//         }
//     }
// }

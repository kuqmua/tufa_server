use super::http_request_version_error::HttpRequestVersionError;
use crate::helpers::http_request::get::version::http_request_version_error::HttpRequestVersionErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Version;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_http_request_version(
    link: &str,
    headers: Option<HeaderMap<HeaderValue>>,
    should_trace: bool,
) -> Result<Version, Box<HttpRequestVersionError>> {
    let request_builder = match headers {
        Some(h) => reqwest::Client::new().get(link).headers(h),
        None => reqwest::Client::new().get(link),
    };
    match request_builder.send().await {
        Err(e) => Err(Box::new(
            HttpRequestVersionError::init_error_with_possible_trace(
                HttpRequestVersionErrorEnum::ReqwestGet(e),
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
        Ok(res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(
                    HttpRequestVersionError::init_error_with_possible_trace(
                        HttpRequestVersionErrorEnum::StatusCode(e),
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
            Ok(res.version())
        }
    }
}

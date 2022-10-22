use super::http_request_bytes_error::HttpRequestBytesError;
use crate::helpers::http_request::get::bytes::http_request_bytes_error::HttpRequestBytesErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use reqwest::RequestBuilder;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_http_request_bytes(
    request_builder: RequestBuilder,
    should_trace: bool,
) -> Result<actix_web::web::Bytes, Box<HttpRequestBytesError>> {
    match request_builder.send().await {
        Err(e) => Err(Box::new(
            HttpRequestBytesError::init_error_with_possible_trace(
                HttpRequestBytesErrorEnum::ReqwestGet(e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(res) => {
            if let Err(e) = res.error_for_status_ref() {
                return Err(Box::new(
                    HttpRequestBytesError::init_error_with_possible_trace(
                        HttpRequestBytesErrorEnum::StatusCode(e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            match res.bytes().await {
                Err(e) => Err(Box::new(
                    HttpRequestBytesError::init_error_with_possible_trace(
                        HttpRequestBytesErrorEnum::ResponseBytes(e),
                        WhereWas {
                            time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .expect("cannot convert time to unix_epoch"),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                )),
                Ok(bytes) => Ok(bytes),
            }
        }
    }
}

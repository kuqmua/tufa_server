use super::http_request_json_error::HttpRequestJsonError;
use crate::helpers::http_request::get::json::http_request_json_error::HttpRequestJsonErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use reqwest::blocking::RequestBuilder;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn sync_http_request_json(
    request_builder: RequestBuilder,
    should_trace: bool,
) -> Result<String, Box<HttpRequestJsonError>> {
    match request_builder.send() {
        Err(e) => Err(Box::new(
            HttpRequestJsonError::init_error_with_possible_trace(
                HttpRequestJsonErrorEnum::ReqwestGet(e),
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
                    HttpRequestJsonError::init_error_with_possible_trace(
                        HttpRequestJsonErrorEnum::StatusCode(e),
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
            match res.json() {
                Err(e) => Err(Box::new(
                    HttpRequestJsonError::init_error_with_possible_trace(
                        HttpRequestJsonErrorEnum::ResponseJson(e),
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
                Ok(json) => Ok(json),
            }
        }
    }
}

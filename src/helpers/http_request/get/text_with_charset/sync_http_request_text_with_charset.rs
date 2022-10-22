use super::http_request_text_with_charset_error::HttpRequestTextWithCharsetError;
use crate::helpers::http_request::get::text_with_charset::http_request_text_with_charset_error::HttpRequestTextWithCharsetErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use reqwest::blocking::RequestBuilder;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn sync_http_request_text_with_charset(
    request_builder: RequestBuilder,
    default_encoding: &str,
    should_trace: bool,
) -> Result<String, Box<HttpRequestTextWithCharsetError>> {
    match request_builder.send() {
        Err(e) => Err(Box::new(
            HttpRequestTextWithCharsetError::init_error_with_possible_trace(
                HttpRequestTextWithCharsetErrorEnum::ReqwestGet(e),
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
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
                    HttpRequestTextWithCharsetError::init_error_with_possible_trace(
                        HttpRequestTextWithCharsetErrorEnum::StatusCode(e),
                        WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                ));
            }
            match res.text_with_charset(default_encoding) {
                Err(e) => Err(Box::new(
                    HttpRequestTextWithCharsetError::init_error_with_possible_trace(
                        HttpRequestTextWithCharsetErrorEnum::ResponseTextWithCharset(e),
                        WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            location: *core::panic::Location::caller(),
                        },
                        &CONFIG.source_place_type,
                        &GIT_INFO.data,
                        should_trace,
                    ),
                )),
                Ok(text_with_charset) => Ok(text_with_charset),
            }
        }
    }
}

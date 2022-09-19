use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use git_info::GitInfo;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_get_source_for_parent_error_struct::ImplGetSourceForParentErrorStruct;
use impl_get_source_for_simple_error_enum::ImplGetSourceForSimpleErrorEnum;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use init_error_with_tracing_for_original_error_struct::InitErrorWithTracingForOriginalErrorStruct;
use reqwest::StatusCode;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplDisplayForErrorStruct,
    ImplGetSourceForParentErrorStruct,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    InitError,
    InitErrorWithTracingForOriginalErrorStruct,
)]
pub struct NetCheckAvailabilityError {
    source: NetCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, GitInfo, ImplDisplayForSimpleErrorEnum, ImplGetSourceForSimpleErrorEnum)]
pub enum NetCheckAvailabilityErrorEnum {
    ReqwestGet(reqwest::Error),
    Client(StatusCode),
    Server(StatusCode),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn net_check_availability(
    link: &str,
    should_trace: bool,
) -> Result<(), Box<NetCheckAvailabilityError>> {
    match reqwest::get(link).await {
        Err(e) => Err(Box::new(
            NetCheckAvailabilityError::init_error_with_possible_trace(
                NetCheckAvailabilityErrorEnum::ReqwestGet(e),
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
            let status = res.status();
            if status.is_client_error() {
                return Err(Box::new(
                    NetCheckAvailabilityError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::Client(status),
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
            if status.is_server_error() {
                return Err(Box::new(
                    NetCheckAvailabilityError::init_error_with_possible_trace(
                        NetCheckAvailabilityErrorEnum::Server(status),
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
            Ok(())
        }
    }
}

use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::fetch::fetch_link_error::FetchLinkErrorEnum;
use crate::helpers::where_was::WhereWas;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn blocking_fetch_link(link: &str) -> Result<String, Box<FetchLinkErrorEnum>> {
    match reqwest::blocking::get(link) {
        Err(e) => Err(Box::new(FetchLinkErrorEnum::ReqwestBlockingGet {
            source: e,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        Ok(res) => {
            let status = res.status();
            if status != reqwest::StatusCode::OK {
                return Err(Box::new(FetchLinkErrorEnum::StatusCode {
                    source: status,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                }));
            }
            match res.text() {
                Err(e) => Err(Box::new(FetchLinkErrorEnum::ResponseText {
                    source: e,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                })),
                Ok(text) => Ok(text),
            }
        }
    }
}

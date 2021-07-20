use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

// use std::time::Instant;
//async
// let res = reqwest::get(link).await?;
//NOT WORKING FOR SOME REASON (returning 404)
// let client = reqwest::blocking::Client::new();
// let res = reqwest::blocking::Client::head(&client, link).send()?;
// let mut result_tuplefff: (bool, HandledReachProviderStatusInfo) =
//     (false, HandledReachProviderStatusInfo::Initialized);
// if res.status() == reqwest::StatusCode::OK {
//     println!("fetch_link res.status() ok");
//     result_tuplefff = (true, HandledReachProviderStatusInfo::Success)
// } else {
//     println!("fetch_link res.status() not ok");
//     result_tuplefff.1 = HandledReachProviderStatusInfo::ResStatusError(res.status());
// }
pub fn rss_check_provider_status(
    link: &str,
) -> Result<(bool, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    let mut result_tuple: (bool, HandledFetchStatusInfo) =
        (false, HandledFetchStatusInfo::Initialized);
    if res.status() == reqwest::StatusCode::OK {
        result_tuple.0 = true;
    } else {
        result_tuple.1 = HandledFetchStatusInfo::ResStatusError(res.status());
        print_colorful_message(
            None,
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            format!("{} {}", link, res.status()),
        );
    }
    Ok(result_tuple)
}

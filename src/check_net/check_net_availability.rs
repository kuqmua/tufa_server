use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::check_net::check_is_status_code_successfull::check_is_status_code_successfull;
use crate::check_net::check_is_status_code_successfull::StatusCodeError;

#[derive(displaydoc::Display, Debug, BoxErrFromErrDerive)]
pub struct CheckNetAvailabilityError {
    pub source: Box<CheckNetAvailabilityErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum CheckNetAvailabilityErrorEnum {
    CheckLinkStatusCodeError(CheckLinkStatusCodeError),
    StatusCodeError(StatusCodeError),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), CheckNetAvailabilityError> {
    let status_code = check_link_status_code(link)?;
    check_is_status_code_successfull(status_code)?;
    Ok(())
}

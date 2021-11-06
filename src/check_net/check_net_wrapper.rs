use crate::check_net::check_link::check_link;

// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_wrapper() -> bool {
    //  if CONFIG.params.enable_prints {
    //         print_colorful_message(
    //             None,
    //             PrintType::Info,
    //             file!().to_string(),
    //             line!().to_string(),
    //             format!("server can reach {}", &CONFIG.params.starting_check_link),
    //         );
    //     }
    let starting_link_checked = check_link(&CONFIG.params.starting_check_link).0;
    let postgres_link_checked = check_link(&postgres_get_db_url()).0;
    let mongo_link_checked = check_link(&mongo_get_db_url()).0;
    starting_link_checked && postgres_link_checked && mongo_link_checked
}

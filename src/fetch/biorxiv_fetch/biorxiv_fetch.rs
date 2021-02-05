extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use std::collections::HashMap;
use std::str;

use super::biorxiv_fetch_and_parse_xml::biorxiv_fetch_and_parse_xml;
use crate::check_net::check_link::check_link;
use crate::config::BIORXIV_URL;
use crate::config::ENABLE_PRINTS_BIORXIV;
// use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;

pub fn biorxiv_part() -> bool {
    if reach_provider(BIORXIV_URL.to_string()) {
        if ENABLE_PRINTS_BIORXIV {
            println!(
                "{:#?} elements in Biorxiv HashMap",
                biorxiv_links_in_hash_map.len()
            )
        };

        biorxiv_fetch_and_parse_xml(vec_of_links, vec_of_keys); //тут есть возвращаемое значение let vec_of_vec_of_strings =
        return true;
    } else {
        return false;
    }
}

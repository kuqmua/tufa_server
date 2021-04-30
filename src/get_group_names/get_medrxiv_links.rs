use std::collections::HashMap;
pub fn get_medrxiv_links(
    medrxiv_names: HashMap<&'static str, &str>,
) -> HashMap<&'static str, String> {
    let first_part_of_link: &str = "http://connect.medrxiv.org/medrxiv_xml.php?subject=";
    let mut medrxiv_links: HashMap<&'static str, String> =
        HashMap::with_capacity(medrxiv_names.len());
    for (key, value) in medrxiv_names {
        medrxiv_links.insert(key, format!("{}{}", first_part_of_link, value));
    }
    medrxiv_links
}

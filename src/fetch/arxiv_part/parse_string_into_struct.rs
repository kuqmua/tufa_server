use super::arxiv_metainfo_structures::AreThereItems;
use super::arxiv_structures::ArxivPost;
use super::arxiv_structures::ArxivPostStruct;
use super::arxiv_structures::Creator;
use super::arxiv_structures::XmlArxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use serde_xml_rs::from_str;

pub fn parse_string_into_struct(
    fetch_tuple_result: String,
    key: &str,
    value: &str,
) -> (ArxivPostStruct, AreThereItems) {
    let mut arxiv_post_struct_handle: ArxivPostStruct = ArxivPostStruct::new();
    let are_there_items_handle: AreThereItems; // = AreThereItems::Initialized
    match fetch_tuple_result.find("</item>") {
        Some(_) => {
            let arxiv_struct_from_str_result: Result<XmlArxivParserStruct, serde_xml_rs::Error> =
                from_str(&fetch_tuple_result);
            match arxiv_struct_from_str_result {
                Ok(arxiv_struct) => {
                    let mut count = 0;
                    let mut arxiv_page_struct: ArxivPostStruct = ArxivPostStruct::new();
                    loop {
                        if count < arxiv_struct.items.len() {
                            let mut arxiv_post: ArxivPost = ArxivPost::new();
                            arxiv_post.title = arxiv_struct.items[count].title.clone();
                            arxiv_post.link = arxiv_struct.items[count].link.clone();
                            arxiv_post.description = arxiv_struct.items[count].description.clone();
                            let mut string_part_for_loop =
                                arxiv_struct.items[count].creator.clone();
                            while let Some(link_index_from_start) =
                                string_part_for_loop.find("<a href=\"")
                            {
                                if let Some(link_index_from_end) = string_part_for_loop.find("\">")
                                {
                                    if let Some(name_index_from_end) =
                                        string_part_for_loop.find("</a>")
                                    {
                                        let mut creator = Creator::new();
                                        creator.link = string_part_for_loop[link_index_from_start
                                            + "<a href=\"".len()
                                            ..link_index_from_end]
                                            .to_string();
                                        let name_index_from_start =
                                            link_index_from_end + "\">".len();
                                        creator.name = string_part_for_loop
                                            [name_index_from_start..name_index_from_end]
                                            .to_string();
                                        string_part_for_loop = string_part_for_loop
                                            [name_index_from_end + "\">".len()..]
                                            .to_string();
                                        arxiv_post.creators.push(creator);
                                    }
                                }
                            }

                            arxiv_page_struct.items.push(arxiv_post);
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if !arxiv_page_struct.items.is_empty() {
                        are_there_items_handle = AreThereItems::Yep;
                    } else {
                        are_there_items_handle =
                            AreThereItems::NopeButThereIsTag(fetch_tuple_result);
                    }
                    arxiv_post_struct_handle = arxiv_page_struct;
                }
                Err(e) => {
                    if ENABLE_ERROR_PRINTS_ARXIV {
                        println!(
                            "arxiv conversion from str for {}, error {}",
                            key,
                            e.to_string()
                        );
                    };
                    are_there_items_handle =
                        AreThereItems::ConversionFromStrError(fetch_tuple_result, e.to_string());
                }
            }
        }
        _ => {
            if ENABLE_PRINTS_ARXIV {
                println!("arxiv no items for key {} {}", key, value);
            };
            are_there_items_handle = AreThereItems::NopeNoTag(fetch_tuple_result);
        }
    }
    (arxiv_post_struct_handle, are_there_items_handle)
}

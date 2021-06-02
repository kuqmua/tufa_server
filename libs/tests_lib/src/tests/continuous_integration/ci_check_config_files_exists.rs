use crate::tests::tests_constants::CONFIG_FILE_EXTENSION;
use crate::tests::tests_constants::VECTOR_OF_MODES;
use get_config_lib::PATH_TO_CONFIG;
use std::fs::File;
#[test]
fn ci_check_config_files_exists() {
    for mode in VECTOR_OF_MODES {
        let file = File::open(format!(
            "{}{}{}",
            PATH_TO_CONFIG, mode, CONFIG_FILE_EXTENSION
        ));
        match file {
            Ok(_) => {}
            Err(e) => panic!("file: {} error: {}", mode, e),
        }
    }
}

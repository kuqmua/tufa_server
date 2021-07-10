use crate::async_tokio_wrapper::async_tokio_wrapper;
use std::time::Instant;

use crate::check_net::check_link::check_link;

use prints_lib::print_colorful_message;
use prints_lib::PrintType;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;

extern crate num_cpus;

pub fn entry() {
    let time = Instant::now();
    let cpus = num_cpus::get();

    if CONFIG.params.enable_prints_handle {
        if cpus > 1 {
            println!("We are on a multicore system with {} CPUs", cpus);
        } else {
            println!("We are on a single core system");
        }
    }
    println!("ENV: {}", CONFIG.env);
    let enable_common_time_measurement = CONFIG.params.enable_common_time_measurement; //need to be different variable cuz move happpens
    if check_link(
        &CONFIG.links.starting_check_link,
        CONFIG.params.enable_error_prints_handle,
    )
    .0
    {
        if CONFIG.params.enable_prints_handle {
            let its_all_good_message =
                "server can reach ".to_string() + &CONFIG.links.starting_check_link;
            println!("{}", its_all_good_message);
        }
        async_tokio_wrapper();
    } else if CONFIG.params.enable_error_prints_handle {
        let its_not_good_message =
            "server cannot reach ".to_string() + &CONFIG.links.starting_check_link;
        print_colorful_message(
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            its_not_good_message,
        );
    }

    if enable_common_time_measurement {
        println!("main done in {} seconds", time.elapsed().as_secs());
    }
}

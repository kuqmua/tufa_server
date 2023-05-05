// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]

mod config_mods;
pub mod configuration;
pub mod email_client;
pub mod entry;
pub mod global_variables;
// pub mod issue_delivery_worker;
mod preparation;
// mod routes;
// mod server_wrapper;
// mod session_state;
// pub mod startup;
#[cfg(test)]
mod tests;

pub mod dev;

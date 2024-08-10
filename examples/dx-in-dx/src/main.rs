#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use dx_in_dx::{register_link, App};

#[allow(clippy::expect_used)]
fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    info!("Register 'plop-link' web component");
    register_link();

    info!("starting app");
    launch(App);
}

#![doc = include_str!("../README.md")]
#![allow(clippy::multiple_crate_versions)]

use dioxus::logger::tracing::{info, Level};
use dioxus::{logger, prelude::*};

use dx_in_dx::{register_link, App};

#[allow(clippy::expect_used)]
fn main() {
    // Init logger
    let _ = logger::init(Level::INFO);

    info!("Register 'plop-link' web component");
    register_link();

    info!("starting the app");
    launch(App);
}

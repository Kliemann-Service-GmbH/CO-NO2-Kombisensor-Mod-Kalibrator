#![feature(stmt_expr_attributes)]
extern crate gdk;
extern crate gtk;
extern crate libmodbus_rs;
extern crate xmz_server;
extern crate byteorder;

#[macro_use] mod macros;
mod gui {
    pub mod gtk3;
}
mod calib_error;
mod configuration;
mod commands;
mod co_no2_kombisensor {
    pub use self::kombisensor::Kombisensor;

    pub mod kombisensor;
    pub mod sensor;
}

use std::sync::{Arc, Mutex};


fn main() {
    let configuration = Arc::new(Mutex::new(configuration::Configuration::new()));
    gui::gtk3::launch(&configuration);
}

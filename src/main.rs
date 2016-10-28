#![feature(stmt_expr_attributes)]
extern crate gdk;
extern crate gtk;
extern crate libmodbus_rs;
extern crate xmz_server;

#[macro_use] mod macros;
mod gui {
    pub mod gtk3;
}
mod commands;
mod co_no2_kombisensor {
    pub use self::kombisensor::Kombisensor;

    pub mod kombisensor;
    pub mod sensor;
}


fn main() {
    gui::gtk3::launch();
}

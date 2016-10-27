#![feature(stmt_expr_attributes)]
extern crate gtk;
extern crate gdk;
extern crate libmodbus_rs;
extern crate xmz_server;

#[macro_use] mod macros;
mod gui {
    pub mod gtk3;
}
mod commands;


fn main() {
    gui::gtk3::launch();
}

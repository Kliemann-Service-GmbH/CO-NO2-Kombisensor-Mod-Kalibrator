#![feature(stmt_expr_attributes)]

extern crate gtk;
extern crate gdk;

mod gui {
    pub mod gtk3;
}


fn main() {
    gui::gtk3::launch();
}

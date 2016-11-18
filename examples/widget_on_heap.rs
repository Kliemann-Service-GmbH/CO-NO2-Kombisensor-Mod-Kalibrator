extern crate gio;
extern crate gtk;

use gio::ApplicationExt;
use gtk::WidgetExt;
use std::borrow::Borrow;

use std::sync::Arc;

fn main() {
    let gtk_app = gtk::Application::new(Some("org.gtk.application"), gio::APPLICATION_FLAGS_NONE).unwrap();

    let main_window = Arc::new(gtk::ApplicationWindow::new(&gtk_app));
    Borrow::<gtk::ApplicationWindow>::borrow(&main_window).show_all();

    { 
        let mw = Arc::downgrade(&main_window);
        gtk_app.connect_startup(move |gapp| {
            gapp.add_window(Borrow::<gtk::ApplicationWindow>::borrow(&mw.upgrade().unwrap()));
        });
    }

    gtk_app.run(0, &[]);
}

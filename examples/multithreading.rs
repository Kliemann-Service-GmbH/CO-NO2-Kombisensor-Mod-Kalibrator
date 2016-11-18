extern crate gtk;
extern crate glib;

use gtk::prelude::*;
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Multithreading GTK+ Program");
    window.set_border_width(10);
    window.set_default_size(600, 400);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let text_view = gtk::TextView::new();
    let scroll = gtk::ScrolledWindow::new(None, None);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let (tx, rx) = channel();
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((text_view.get_buffer().unwrap(), rx))
    });

    thread::spawn(move|| {
        for i in 1..100 {
            //do long work
            thread::sleep(Duration::from_millis(50));
            //send result to gui
            tx.send(format!("#{} Text from another thread.", i)).unwrap();
            glib::idle_add(receive);
        }
    });

    window.add(&scroll);
    window.show_all();
    gtk::main();
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref buf, ref rx)) = *global.borrow() {
            if let Ok(text) = rx.try_recv() {
                buf.set_text(&text);
            }
        }
    });
    glib::Continue(false)
}

thread_local!(
    static GLOBAL: RefCell<Option<(gtk::TextBuffer, Receiver<String>)>> = RefCell::new(None)
);

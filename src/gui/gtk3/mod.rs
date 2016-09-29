use gtk;
use gtk::prelude::*;
use gdk::enums::key;


mod calibrator_view;


// Callback zum Speichern der Modbus Adresse
fn callback_button_save_modbus_address(builder: &gtk::Builder) {
    println!("Click click {:?}", builder);
}
// Callback Kalibrieren Button NO2 geklickt
fn callback_button_calib_no2(builder: &gtk::Builder) {
    calibrator_view::launch("NO2", &builder);
}
// Callback Kalibrieren Button CO geklickt
fn callback_button_calib_co(builder: &gtk::Builder) {
    calibrator_view::launch("CO", &builder);
}

// Basic Setup des Fensters
fn window_setup(window: &gtk::Window) {
    let window_title = format!("{} {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    window.set_default_size(1024, 600);
    // window.set_border_width(10);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);

    match ::std::env::var("XMZ_HARDWARE") {
        Ok(_) => {
            window.fullscreen();
        }
        Err(_) => {}
    }
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
        env!("CARGO_PKG_NAME")))
    });

    // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
    let builder = gtk::Builder::new_from_string(include_str!("interface.glade"));
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_setup(&window);

    window.show_all();

    // Callback 'button_save_modbus_address' geklickt
    let builder1 = builder.clone();
    button_save_modbus_address.connect_clicked(move |_| {
        callback_button_save_modbus_address(&builder1);
    });

    // Callback 'button_calib_no2' geklickt
    let builder1 = builder.clone();
    button_calib_no2.connect_clicked(move |_| {
        callback_button_calib_no2(&builder1);
    });

    // Callback 'button_calib_co' geklickt
    let builder1 = builder.clone();
    button_calib_co.connect_clicked(move |_| {
        callback_button_calib_co(&builder1);
    });

    // Beende Programm wenn das Fenster geschlossen wurde
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Registriert die Esc Taste mit main_quit() (schliesst also das Fenster mit der Esc Taste),
    // nur in DEBUG Builds. Wird das Programm mit `--release` übersetzt, funktioniert dies nicht.
    #[cfg(debug_assertions)]
    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    gtk::main();
}

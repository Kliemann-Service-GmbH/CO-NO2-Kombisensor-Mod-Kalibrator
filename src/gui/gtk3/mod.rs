use gtk;
use gtk::prelude::*;
use gdk::enums::key;

mod calibrator_view;
mod static_resource;

// Callback Sensor Erkennen, Discovery
fn callback_button_discover(spinner_discovery: &gtk::Spinner) {
    spinner_discovery.start();
}

// Callback zum Speichern der Modbus Adresse
fn callback_button_save_modbus_address(builder: &gtk::Builder) {
    println!("Modbus Adresse speichern {:?}", builder);
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

    static_resource::init();

    // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
    let builder = gtk::Builder::new_from_resource("/com/gaswarnanlagen/xmz-mod-touch/GUI/main.ui");
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_enable_no2: gtk::ToggleButton = builder.get_object("button_enable_no2").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_enable_co: gtk::ToggleButton = builder.get_object("button_enable_co").unwrap();
    let spinner_discovery: gtk::Spinner = builder.get_object("spinner_discovery").unwrap();
    let button_discover: gtk::Button = builder.get_object("button_discover").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_setup(&window);

    window.show_all();

    // Callback Senor erkennen, Discovery
    button_discover.connect_clicked(clone!(window => move |_| {
        callback_button_discover(&spinner_discovery);
    }));

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

    let button_enable_no2_clone = button_enable_no2.clone();
    button_enable_no2.connect_clicked(clone!(window => move |_| {
        if button_enable_no2_clone.get_active() {
            &label_no2.set_sensitive(false);
            &button_calib_no2.set_sensitive(false);
        } else {
            &label_no2.set_sensitive(true);
            &button_calib_no2.set_sensitive(true);
        }
    }));

    let button_enable_co_clone = button_enable_co.clone();
    button_enable_co.connect_clicked(clone!(window => move |_| {
        if button_enable_co_clone.get_active() {
            &label_co.set_sensitive(false);
            &button_calib_co.set_sensitive(false);
        } else {
            &label_co.set_sensitive(true);
            &button_calib_co.set_sensitive(true);
        }
    }));

    // // Callback 'button_calib_co' geklickt
    // let builder1 = builder.clone();
    // button_calib_co.connect_clicked(move |_| {
    //     callback_button_calib_co(&builder1);
    // });

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

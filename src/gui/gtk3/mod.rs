extern crate libc;
extern crate glib_sys as glib_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate glib;
use self::glib::translate::ToGlibPtr;
use self::glib_ffi::gpointer;
use commands;
use gdk::enums::key;
use gtk;
use gtk::prelude::*;
use std::path::Path;

mod calibrator_view;
mod static_resource;

// Callback Sensor Erkennen, Discovery
fn callback_button_discover(builder: &gtk::Builder) {
    let spinner_discovery: gtk::Spinner = builder.get_object("spinner_discovery").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();

    if Path::new("/dev/ttyUSB0").exists() {
        spinner_discovery.start();
    } else {
        label_info_bar_message.set_text("Serielle Schnittstelle nicht gefunden");
        info_bar.show();
    }
}

// Gemeinsamer Callback
fn callback_button_enable_sensor(button_enable: &gtk::ToggleButton, button_calib: &gtk::Button, label: &gtk::Label) {
    if button_enable.get_active() {
        match commands::enable_no2(false) {
            Ok(_) => {
                // Deaktiviere NO2 Sensor
                label.set_sensitive(false);
                button_calib.set_sensitive(false);
            }
            Err(_) => {
                button_enable.set_active(false);
            }
        }
    } else { // Knopf war gedrueckt, Sensor muss wieder aktiviert werden.
        match commands::enable_no2(true) {
            Ok(_) => {
                // Deaktiviere NO2 Sensor
                label.set_sensitive(true);
                button_calib.set_sensitive(true);
            }
            Err(_) => {
                button_enable.set_active(true);
            }
        }
    }
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

    // Deaktivier Animationen. Behebt den Bug das der InfoBar nur einmal angezeigt wird, oder
    // nur angezeigt wird, wenn das Fenster kein Fokus hat.
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    unsafe{
        self::gobject_ffi::g_object_set (gtk_ffi::gtk_settings_get_default () as gpointer,
        "gtk-enable-animations".to_glib_none().0, glib_ffi::GFALSE, ::std::ptr::null::<libc::c_void>());
    }

    // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
    let builder = gtk::Builder::new_from_resource("/com/gaswarnanlagen/xmz-mod-touch/GUI/main.ui");
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_discover: gtk::Button = builder.get_object("button_discover").unwrap();
    let button_enable_co: gtk::ToggleButton = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::ToggleButton = builder.get_object("button_enable_no2").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let window: gtk::Window = builder.get_object("main_window").unwrap();

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_setup(&window);

    window.show_all();

    info_bar.hide();
    // Close callback
    info_bar.connect_response(clone!(info_bar => move |info_bar, _| {
        info_bar.hide();
    }));

    // Callback Senor erkennen, Discovery
    button_discover.connect_clicked(clone!(builder => move |_| {
        callback_button_discover(&builder);
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

    button_enable_no2.connect_clicked(clone!(button_enable_no2, button_calib_no2 => move |_| {
        callback_button_enable_sensor(&button_enable_no2, &button_calib_no2, &label_no2);
    }));

    button_enable_co.connect_clicked(clone!(button_enable_co, button_calib_co => move |_| {
        callback_button_enable_sensor(&button_enable_co, &button_calib_co, &label_co);
    }));

    button_calib_co.connect_clicked(clone!(builder => move |_| {
        callback_button_calib_co(&builder);
    }));

    button_calib_no2.connect_clicked(clone!(builder => move |_| {
        callback_button_calib_no2(&builder);
    }));

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

extern crate libc;
extern crate glib_sys as glib_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate glib;

use std::error::Error;
use configuration::Configuration;
use self::glib::translate::ToGlibPtr;
use self::glib_ffi::gpointer;
use commands;
use gdk::enums::key;
use gtk;
use gtk::prelude::*;
use std::path::Path;
use co_no2_kombisensor::*;
use std::sync::{Arc, Mutex};

mod calibrator_view;
mod static_resource;
mod view_messpunkt;
mod view_liveview;


fn callback_button_sensor_connect(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    use std::mem::transmute;

    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_discover: gtk::Button = builder.get_object("button_discover").unwrap();
    let button_enable_co: gtk::ToggleButton = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::ToggleButton = builder.get_object("button_enable_no2").unwrap();
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    let label_kombisensor_version: gtk::Label = builder.get_object("label_kombisensor_version").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let spin_button_modbus_address: gtk::SpinButton = builder.get_object("spin_button_modbus_address").unwrap();
    let adjustment_modbus_address: gtk::Adjustment = builder.get_object("adjustment_modbus_address").unwrap();

    // Get modbus Adresse von dem Adjustment
    {
        let mut kombisensor = kombisensor.lock().unwrap();
        // Die Adjustment Werte sind leider f64 Datentypen. Die nachsten Schritte sind nötig um
        // daraus ein u8 Datentyp zu machen.
        let modbus_address = adjustment_modbus_address.get_value().round() as u64;
        let modbus_address: [u8; 8] = unsafe { ::std::mem::transmute(modbus_address)};
        kombisensor.set_modbus_address(modbus_address[0] as u8);
    }

    // Wird ein Sensor erkannt dann wird als nächstes die Kombisensor Datenstruktur
    // mit den Daten der echten Hardware gefüllt.
    match commands::kombisensor_from_modbus(&kombisensor) {
        Err(err) => {
            label_info_bar_message.set_text(err.description());
            info_bar.show();
        }
        Ok(_) => {
            // Label "Firmware Version" füllen
            label_kombisensor_version.set_text(&kombisensor.lock().unwrap().get_version());

            // Widget aktivieren
            // TODO: Funktion Widget Status -> Kombisensor Struct
            button_calib_co.set_sensitive(true);
            button_calib_no2.set_sensitive(true);
            button_enable_co.set_sensitive(true);
            button_enable_no2.set_sensitive(true);
            button_save_modbus_address.set_sensitive(false);
            button_save_modbus_address.set_sensitive(true);
            label_co.set_sensitive(true);
            label_no2.set_sensitive(true);


            spin_button_modbus_address.set_value(kombisensor.lock().unwrap().get_modbus_address() as f64);
        }
    }

    button_live_view.set_sensitive(true);
}

fn callback_button_live_view(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    view_liveview::launch(&builder, &kombisensor);
}

// Callback Sensor Erkennen, Discovery
fn callback_button_discover(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>, configuration: &Arc<Mutex<Configuration>>) {
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_discover: gtk::Button = builder.get_object("button_discover").unwrap();
    let button_enable_co: gtk::ToggleButton = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::ToggleButton = builder.get_object("button_enable_no2").unwrap();
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    let label_kombisensor_version: gtk::Label = builder.get_object("label_kombisensor_version").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let spin_button_modbus_address: gtk::SpinButton = builder.get_object("spin_button_modbus_address").unwrap();
    let window: gtk::Window = builder.get_object("main_window").unwrap();

    // Get config from Arc<Mutex<>>
    let configuration = configuration.lock().unwrap();
    // Wenn die Serielle Schnittstelle existiert dann mache weiter
    match configuration.is_valid() {
        Ok(_) => {
            match commands::kombisensor_discovery(kombisensor) {
                Ok(_) => {
                    // Wird ein Sensor erkannt dann wird als nächstes die Kombisensor Datenstruktur
                    // mit den Daten der echten Hardware gefüllt.
                    commands::kombisensor_from_modbus(&kombisensor);

                    // Label "Firmware Version" füllen
                    label_kombisensor_version.set_text(&kombisensor.lock().unwrap().get_version());

                    // Widget aktivieren
                    // TODO: Funktion Widget Status -> Kombisensor Struct
                    button_calib_co.set_sensitive(true);
                    button_calib_no2.set_sensitive(true);
                    button_enable_co.set_sensitive(true);
                    button_enable_no2.set_sensitive(true);
                    button_save_modbus_address.set_sensitive(false);
                    button_save_modbus_address.set_sensitive(true);
                    label_co.set_sensitive(true);
                    label_no2.set_sensitive(true);

                    spin_button_modbus_address.set_value(kombisensor.lock().unwrap().get_modbus_address() as f64);
                    println!("{:#?}", kombisensor);
                }
                Err(err) => {}
            };
        }
        Err(err) => {
            label_info_bar_message.set_text(err.description());
            info_bar.show();
        }
    }
}

// Gemeinsamer Callback
fn callback_button_enable_sensor(builder: &gtk::Builder, button: &gtk::ToggleButton, kombisensor: &Arc<Mutex<Kombisensor>>, configuration: &Arc<Mutex<Configuration>>) {
    let button_enable_co: gtk::ToggleButton = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::ToggleButton = builder.get_object("button_enable_no2").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    // Get config from Arc<Mutex<>>
    let configuration = configuration.lock().unwrap();

    let mut sensor_type: String = String::new();
    let sensor_status = !button.get_active();

    // Checke Serielles Interface
    match configuration.is_valid() {
        Ok(_) => {}
        Err(err) => {
            label_info_bar_message.set_text(err.description());
            info_bar.show();
            button.set_active(false);
            return;
        }
    }

    if button == &button_enable_co {
        sensor_type = "CO".to_string();
        match commands::enable_sensor(&kombisensor, &sensor_type, sensor_status) {
            Ok(_) => {
                button_calib_co.set_sensitive(sensor_status);
                label_co.set_sensitive(sensor_status);
            }
            Err(_) => {}
        }
    } else if button == &button_enable_no2 {
        sensor_type = "NO2".to_string();
        match commands::enable_sensor(&kombisensor, &sensor_type, sensor_status) {
            Ok(_) => {
                button_calib_no2.set_sensitive(sensor_status);
                label_no2.set_sensitive(sensor_status);
            }
            Err(_) => {}
        }
    }
}

// Callback zum Speichern der Modbus Adresse
fn callback_button_save_modbus_address(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let adjustment_modbus_address: gtk::Adjustment = builder.get_object("adjustment_modbus_address").unwrap();
    // Die Adjustment Werte sind leider f64 Datentypen. Die nachsten Schritte sind nötig um
    // daraus ein u8 Datentyp zu machen.
    let modbus_address = adjustment_modbus_address.get_value().round() as u64;
    let modbus_address: [u32; 2] = unsafe { ::std::mem::transmute(modbus_address)};
    let new_modbus_address: i32 = modbus_address[0] as i32;

    commands::kombisensor_new_modbus_address(kombisensor, new_modbus_address);
}

// Callback Kalibrieren Button NO2 geklickt
fn callback_button_calib_no2(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    calibrator_view::launch("NO2", &builder, &kombisensor);
}
// Callback Kalibrieren Button CO geklickt
fn callback_button_calib_co(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    calibrator_view::launch("CO", &builder, &kombisensor);
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


pub fn launch(configuration: &Arc<Mutex<Configuration>>) {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
        env!("CARGO_PKG_NAME")))
    });

    static_resource::init();

    // Deaktivier Animationen. Behebt den Bug das der InfoBar nur einmal angezeigt wird, oder
    // nur angezeigt wird, wenn das Fenster kein Fokus hat.
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        self::gobject_ffi::g_object_set (gtk_ffi::gtk_settings_get_default () as gpointer,
        "gtk-enable-animations".to_glib_none().0, glib_ffi::GFALSE, ::std::ptr::null::<libc::c_void>());
    }

    // Die Kombisensor Datenstruktur ist die Softwarebescheibung des Sensors an den der Kalibrator
    // angeschlossen ist. Mit dessen Funktionen und Attributen werden dann die verschiedenen
    // Widgets der GUI gefüllt.
    let kombisensor = Arc::new(Mutex::new(Kombisensor::new()));

    // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
    let builder = gtk::Builder::new_from_resource("/com/gaswarnanlagen/xmz-mod-touch/GUI/main.ui");
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_discover: gtk::Button = builder.get_object("button_discover").unwrap();
    let button_sensor_connect: gtk::Button = builder.get_object("button_sensor_connect").unwrap();
    let button_enable_co: gtk::ToggleButton = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::ToggleButton = builder.get_object("button_enable_no2").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let window: gtk::Window = builder.get_object("main_window").unwrap();

    // Bei Programstart werden alle Button und Label erstmal auf nicht sensitive gestellt.
    // Erst wenn eine Modbus Adresse gefunden wurde, discovery, werden die jeweiligen Widgets aktiv.
    button_calib_co.set_sensitive(false);
    button_calib_no2.set_sensitive(false);
    button_enable_co.set_sensitive(false);
    button_enable_no2.set_sensitive(false);
    button_live_view.set_sensitive(false);
    button_save_modbus_address.set_sensitive(false);
    label_co.set_sensitive(false);
    label_no2.set_sensitive(false);


    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_setup(&window);

    window.show_all();

    info_bar.hide();
    // Close callback
    info_bar.connect_response(clone!(info_bar => move |info_bar, _| {
        info_bar.hide();
    }));

    button_sensor_connect.connect_clicked(clone!(builder, kombisensor => move |_| {
        callback_button_sensor_connect(&builder, &kombisensor);
    }));

    button_live_view.connect_clicked(clone!(builder, kombisensor => move |_| {
        callback_button_live_view(&builder, &kombisensor);
    }));

    // Callback Senor erkennen, Discovery
    button_discover.connect_clicked(clone!(builder, kombisensor, configuration => move |_| {
        callback_button_discover(&builder, &kombisensor, &configuration);
    }));

    // Callback 'button_save_modbus_address' geklickt
    button_save_modbus_address.connect_clicked(clone!(builder, kombisensor => move |_| {
        callback_button_save_modbus_address(&builder, &kombisensor);
    }));

    button_enable_no2.connect_clicked(clone!(builder, button_enable_no2, kombisensor, configuration => move |_| {
        callback_button_enable_sensor(&builder, &button_enable_no2, &kombisensor, &configuration);
    }));

    button_enable_co.connect_clicked(clone!(builder, button_enable_co, kombisensor, configuration => move |_| {
        callback_button_enable_sensor(&builder, &button_enable_co, &kombisensor, &configuration);
    }));

    button_calib_co.connect_clicked(clone!(builder, kombisensor => move |_| {
        callback_button_calib_co(&builder, &kombisensor);
    }));

    button_calib_no2.connect_clicked(clone!(builder, kombisensor => move |_| {
        callback_button_calib_no2(&builder, &kombisensor);
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

    // Worker Threads

    // Update der sensor Daten via Modbus. Nur wenn das `live_view` flag gesetzt ist. Das geschieht
    // z.B. im callback_button_live_view
    use std::thread;
    use std::time::Duration;

    thread::spawn(clone!(kombisensor => move || {
        loop {
            {
                let mut kombisensor = kombisensor.lock().unwrap();
                if kombisensor.get_live_update() {
                    println!("{:?}", &kombisensor.get_modbus_address());
                }
            }
            thread::sleep(Duration::from_millis(500));
        } // End loop
    }));


    gtk::main();
}

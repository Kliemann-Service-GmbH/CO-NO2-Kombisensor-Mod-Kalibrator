extern crate glib_sys as glib_ffi;
extern crate glib;
extern crate gobject_sys as gobject_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate libc;

use co_no2_kombisensor::*;
use co_no2_kombisensor::sensor::SensorType;
use commands;
use configuration::Configuration;
use gdk::enums::key;
use gtk;
use gtk::prelude::*;
use self::glib_ffi::gpointer;
use self::glib::translate::ToGlibPtr;
use std::error::Error;
use std::sync::{Arc, Mutex};

mod static_resource;
mod view_calibrator;
mod view_liveview;
mod view_messpunkt;
mod view_show_values;


/// Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
fn update_widgets(builder: &gtk::Builder) {
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let button_enable_co: gtk::Button = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::Button = builder.get_object("button_enable_no2").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_show_values: gtk::Button = builder.get_object("button_show_values").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let label_kalibrator_version: gtk::Label = builder.get_object("label_kalibrator_version").unwrap();

    label_kalibrator_version.set_text(env!("CARGO_PKG_VERSION"));

    // Bei Programstart werden alle Button und Label erstmal auf nicht sensitive gestellt.
    // Erst wenn eine Modbus Adresse gefunden wurde, discovery, werden die jeweiligen Widgets aktiv.
    button_calib_co.set_sensitive(false);
    button_save_modbus_address.set_sensitive(false);
    button_calib_no2.set_sensitive(false);
    button_enable_co.set_sensitive(false);
    button_enable_no2.set_sensitive(false);
    button_live_view.set_sensitive(false);
    button_show_values.set_sensitive(false);
    label_co.set_sensitive(false);
    label_no2.set_sensitive(false);
}

// Callback "Sensor auslesen"
fn callback_button_sensor_connect(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let adjustment_modbus_address: gtk::Adjustment = builder.get_object("adjustment_modbus_address").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_enable_co: gtk::Button = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::Button = builder.get_object("button_enable_no2").unwrap();
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let button_show_values: gtk::Button = builder.get_object("button_show_values").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    let label_kombisensor_version: gtk::Label = builder.get_object("label_kombisensor_version").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let spin_button_modbus_address: gtk::SpinButton = builder.get_object("spin_button_modbus_address").unwrap();

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
            button_save_modbus_address.set_sensitive(true);
            button_live_view.set_sensitive(true);
            button_show_values.set_sensitive(true);
            button_enable_no2.set_sensitive(true);
            button_enable_co.set_sensitive(true);

            // Buttons zum aktivieren/ deaktivieren der
            //let kombisensor = &kombisensor.lock().unwrap();
            if kombisensor.lock().unwrap().sensors[0].is_enabled() {
                button_calib_no2.set_sensitive(true);
                label_no2.set_sensitive(true);
            } else {
                button_calib_no2.set_sensitive(false);
                label_no2.set_sensitive(false);
            }
            if kombisensor.lock().unwrap().sensors[1].is_enabled() {
                button_calib_co.set_sensitive(true);
                label_co.set_sensitive(true);
            } else {
                button_calib_co.set_sensitive(false);
                label_co.set_sensitive(false);
            }

            button_enable_no2.connect_clicked(clone!(builder, button_enable_no2, kombisensor => move |_| {
                let _ = callback_button_enable_sensor(&builder, &kombisensor, SensorType::RaGasNO2);
            }));

            button_enable_co.connect_clicked(clone!(builder, button_enable_co, kombisensor => move |_| {
                let _ = callback_button_enable_sensor(&builder, &kombisensor, SensorType::RaGasCO);
            }));


            info_bar.hide();

            spin_button_modbus_address.set_value(kombisensor.lock().unwrap().get_modbus_address() as f64);
        }
    }
}

// Callback Live Ansicht der Sensor Messzellen
fn callback_button_live_view(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    view_liveview::launch(&builder, &kombisensor);
}

// Callback Sensor Erkennen, Discovery
fn callback_button_discover(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>, configuration: &Arc<Mutex<Configuration>>) {
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_enable_co: gtk::Button = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::Button = builder.get_object("button_enable_no2").unwrap();
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let button_show_values: gtk::Button = builder.get_object("button_show_values").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    let label_kombisensor_version: gtk::Label = builder.get_object("label_kombisensor_version").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let spin_button_modbus_address: gtk::SpinButton = builder.get_object("spin_button_modbus_address").unwrap();

    // Get config from Arc<Mutex<>>
    let configuration = configuration.lock().unwrap();
    // Wenn die Serielle Schnittstelle existiert dann mache weiter
    match configuration.is_valid() {
        Ok(_) => {
            match commands::kombisensor_discovery(kombisensor) {
                Ok(_) => {
                    // Wird ein Sensor erkannt dann wird als nächstes die Kombisensor Datenstruktur
                    // mit den Daten der echten Hardware gefüllt.
                    let _ = commands::kombisensor_from_modbus(&kombisensor);

                    // Label "Firmware Version" füllen
                    label_kombisensor_version.set_text(&kombisensor.lock().unwrap().get_version());

                    // Widget aktivieren
                    // TODO: Funktion Widget Status -> Kombisensor Struct
                    button_calib_co.set_sensitive(true);
                    button_calib_no2.set_sensitive(true);
                    button_enable_co.set_sensitive(true);
                    button_enable_no2.set_sensitive(true);
                    button_save_modbus_address.set_sensitive(true);
                    button_show_values.set_sensitive(true);
                    label_co.set_sensitive(true);
                    label_no2.set_sensitive(true);
                    button_live_view.set_sensitive(true);

                    spin_button_modbus_address.set_value(kombisensor.lock().unwrap().get_modbus_address() as f64);
                    println!("Erkannter Sensor:\n{:#?}\n", kombisensor);
                }
                Err(err) => {
                    label_info_bar_message.set_text(err.description());
                    info_bar.show();
                }
            };
        }
        Err(err) => {
            label_info_bar_message.set_text(err.description());
            info_bar.show();
        }
    }
}

// Aktion für den Reset Knopf
fn callback_button_reset(builder: &gtk::Builder) {
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_enable_co: gtk::Button = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::Button = builder.get_object("button_enable_no2").unwrap();
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let button_show_values: gtk::Button = builder.get_object("button_show_values").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    let label_kombisensor_version: gtk::Label = builder.get_object("label_kombisensor_version").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let adjustment_modbus_address: gtk::Adjustment = builder.get_object("adjustment_modbus_address").unwrap();

    button_calib_co.set_sensitive(false);
    button_calib_no2.set_sensitive(false);
    button_enable_co.set_sensitive(false);
    button_enable_no2.set_sensitive(false);
    button_save_modbus_address.set_sensitive(false);
    button_live_view.set_sensitive(false);
    button_show_values.set_sensitive(false);
    label_co.set_sensitive(false);
    label_no2.set_sensitive(false);

    adjustment_modbus_address.set_value(247.0);

    info_bar.hide();
}

// Gemeinsamer Callback
fn callback_button_enable_sensor(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>, sensor_type: SensorType) {
    let button_enable_co: gtk::Button = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::Button = builder.get_object("button_enable_no2").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let label_co: gtk::Label = builder.get_object("label_co").unwrap();
    let label_no2: gtk::Label = builder.get_object("label_no2").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    //// Get config from Arc<Mutex<>>
    //let configuration = configuration.lock().unwrap();
    //
    //// Checke Serielles Interface
    //match configuration.is_valid() {
    //    Ok(_) => {}
    //    Err(err) => {
    //        label_info_bar_message.set_text(err.description());
    //        info_bar.show();
    //        return;
    //    }
    //}

    match sensor_type {
        SensorType::RaGasNO2 => {
            println!("NO2");
            if kombisensor.lock().unwrap().sensors[0].is_enabled() {
                println!("disable");
                match commands::enable_sensor(&kombisensor, SensorType::RaGasNO2, false) {
                    Ok(_) => {
                        info_bar.hide();
                        button_calib_no2.set_sensitive(false);
                        label_no2.set_sensitive(false);
                    }
                    Err(err) => {
                        label_info_bar_message.set_text(err.description());
                        info_bar.show();
                    }
                }
            } else {
                println!("enable");
                match commands::enable_sensor(&kombisensor, SensorType::RaGasNO2, true) {
                    Ok(_) => {
                        info_bar.hide();
                        button_calib_no2.set_sensitive(true);
                        label_no2.set_sensitive(true);
                    }
                    Err(err) => {
                        label_info_bar_message.set_text(err.description());
                        info_bar.show();
                    }
                }
            }
        }
        SensorType::RaGasCO => {
            println!("CO");
            if kombisensor.lock().unwrap().sensors[1].is_enabled() {
                println!("disable");
                match commands::enable_sensor(&kombisensor, SensorType::RaGasCO, false) {
                    Ok(_) => {
                        info_bar.hide();
                        button_calib_co.set_sensitive(false);
                        label_co.set_sensitive(false);
                    }
                    Err(err) => {
                        label_info_bar_message.set_text(err.description());
                        info_bar.show();
                    }
                }
            } else {
                println!("enable");
                match commands::enable_sensor(&kombisensor, SensorType::RaGasCO, true) {
                    Ok(_) => {
                        info_bar.hide();
                        button_calib_co.set_sensitive(true);
                        label_co.set_sensitive(true);
                    }
                    Err(err) => {
                        label_info_bar_message.set_text(err.description());
                        info_bar.show();
                    }
                }
            }
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

    let _ = commands::kombisensor_new_modbus_address(kombisensor, new_modbus_address);
}

// Callback Kalibrieren Button NO2 geklickt
fn callback_button_calib_no2(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let _ = view_calibrator::launch(SensorType::RaGasNO2, &builder, &kombisensor);
}
// Callback Kalibrieren Button CO geklickt
fn callback_button_calib_co(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<(), Box<::std::error::Error>>{
    try!(view_calibrator::launch(SensorType::RaGasCO, &builder, &kombisensor));

    Ok(())
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
            // Fullscreen geht unter Weston nicht richtig wenn die Fenstergrösse nicht 100% unter unserer Konstrolle sind.
            // Die Anwenung ist dann im Fehlerfall einfach nicht sichtbar, aber gestartet.
            window.fullscreen();

            //// Alternative wird halt ein richtiges Fenster maximiert. Wichtig ist hier das es nicht
            //// schliessbar und verschiebbar ist...
            //window.maximize();
            //window.set_deletable(false);
            //window.set_resizable(false);
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
        self::gobject_ffi::g_object_set (gtk_ffi::gtk_settings_get_default() as *mut self::gobject_ffi::GObject,
        "gtk-enable-animations".to_glib_none().0, glib_ffi::GFALSE);
    }

    // Die Kombisensor Datenstruktur ist die Softwarebescheibung des Sensors an den der Kalibrator
    // angeschlossen ist. Mit dessen Funktionen und Attributen werden dann die verschiedenen
    // Widgets der GUI gefüllt.
    let kombisensor = Arc::new(Mutex::new(Kombisensor::new()));

    let builder = gtk::Builder::new_from_resource("/com/gaswarnanlagen/xmz-mod-touch/GUI/main.ui");

    // Widget Instancen
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let button_calib_co: gtk::Button = builder.get_object("button_calib_co").unwrap();
    let button_calib_no2: gtk::Button = builder.get_object("button_calib_no2").unwrap();
    let button_discover: gtk::Button = builder.get_object("button_discover").unwrap();
    let button_enable_co: gtk::Button = builder.get_object("button_enable_co").unwrap();
    let button_enable_no2: gtk::Button = builder.get_object("button_enable_no2").unwrap();
    let button_live_view: gtk::Button = builder.get_object("button_live_view").unwrap();
    let button_save_modbus_address: gtk::Button = builder.get_object("button_save_modbus_address").unwrap();
    let button_sensor_connect: gtk::Button = builder.get_object("button_sensor_connect").unwrap();
    let button_show_values: gtk::Button = builder.get_object("button_show_values").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let button_reset: gtk::Button = builder.get_object("button_reset").unwrap();

    update_widgets(&builder);

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_setup(&window);

    window.show_all();

    info_bar.hide();

    // Close callback
    info_bar.connect_response(move |info_bar, _| {
        info_bar.hide();
    });

    button_show_values.connect_clicked(clone!(builder, kombisensor => move |_| {
        view_show_values::launch(&builder, &kombisensor);
    }));

    button_reset.connect_clicked(clone!(builder => move |_| {
        callback_button_reset(&builder);
    }));

    button_sensor_connect.connect_clicked(clone!(builder, kombisensor => move |_| {
        callback_button_sensor_connect(&builder, &kombisensor);
    }));

    button_live_view.connect_clicked(clone!(builder, kombisensor => move |_| {
        callback_button_live_view(&builder, &kombisensor);
    }));

    // Callback Senor erkennen, Discovery
    button_discover.connect_clicked(clone!(builder, kombisensor, configuration => move |_| {
        let _ = callback_button_discover(&builder, &kombisensor, &configuration);
    }));

    // Callback 'button_save_modbus_address' geklickt
    button_save_modbus_address.connect_clicked(clone!(builder, kombisensor => move |_| {
        let _ = callback_button_save_modbus_address(&builder, &kombisensor);
    }));

    button_calib_co.connect_clicked(clone!(builder, kombisensor => move |_| {
        let _ = callback_button_calib_co(&builder, &kombisensor);
    }));

    button_calib_no2.connect_clicked(clone!(builder, kombisensor => move |_| {
        let _ = callback_button_calib_no2(&builder, &kombisensor);
    }));

    // Beende Programm wenn das Fenster geschlossen wurde
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

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
            // Ein weiterer Klone des Kombisensors ist nötig, zur Abfrage des Live Update Flags
            let kombisensor1 = kombisensor.clone();
            if kombisensor1.lock().unwrap().get_live_update() {
                let _ = commands::kombisensor_from_modbus(&kombisensor);
            }

            thread::sleep(Duration::from_millis(500));
        } // End loop
    }));


    gtk::main();
}

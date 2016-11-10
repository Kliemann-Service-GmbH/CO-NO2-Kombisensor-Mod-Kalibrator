use gtk;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;
use co_no2_kombisensor::kombisensor::{Kombisensor};


fn save_adc_value(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let button_messpunkt_save: gtk::Button = builder.get_object("button_messpunkt_save").unwrap();
    button_messpunkt_save.connect_clicked(clone!(builder, kombisensor => move |_| {
        let check_button_adc_manuell: gtk::CheckButton = builder.get_object("check_button_adc_manuell").unwrap();

        let mut adc_value: i32 = 0;

        if check_button_adc_manuell.get_active() {
            let adjustment_sensor_adc_value_at: gtk::Adjustment = builder.get_object("adjustment_sensor_adc_value_at").unwrap();
            let mut kombisensor = kombisensor.lock().unwrap();
            kombisensor.set_live_update(false);
            adc_value = adjustment_sensor_adc_value_at.get_value() as i32;
        } else {
            // Live Update beenden und aktuellen ADC Wert aus der Sensor Struktur entnehmen.
            // Die Kombisensor.-/ Sensor Structuren werden im Worker Task über Modbus abgeglichen
            let mut kombisensor = kombisensor.lock().unwrap();
            kombisensor.set_live_update(false);
            adc_value = kombisensor.sensors[0].get_adc_value() as i32;
        }

        ::commands::sensor_new_adc_at_nullgas(&kombisensor, adc_value);
    }));
}

fn update_widgets(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    gtk::timeout_add(100, clone!(kombisensor, builder => move || {
        let label_messpunkt_sensor_type: gtk::Label = builder.get_object("label_messpunkt_sensor_type").unwrap();
        let label_messpunkt_adc: gtk::Label = builder.get_object("label_messpunkt_adc").unwrap();
        let label_messpunkt_mV: gtk::Label = builder.get_object("label_messpunkt_mV").unwrap();

        // Wurde der Checkbutton getrueckt dann gibts kein linve update.
        // Dadurch das der CheckButton das Live Update beendet, wird auch dieser Thread beendet, siehe weiter unten.
        let check_button_adc_manuell: gtk::CheckButton = builder.get_object("check_button_adc_manuell").unwrap();
        if check_button_adc_manuell.get_active() {
            let mut kombisensor = kombisensor.lock().unwrap();
            kombisensor.set_live_update(false);
        }

        let adjustment_sensor_adc_value_at: gtk::Adjustment = builder.get_object("adjustment_sensor_adc_value_at").unwrap();
        {
            let kombisensor = kombisensor.lock().unwrap();;
            let adc_value = kombisensor.sensors[0].get_adc_value() as f64;
            adjustment_sensor_adc_value_at.set_value(adc_value);
        }

        label_messpunkt_sensor_type.set_text("NO2 Messzelle");

        let mut adc_value = String::new();
        let mut mv_value = String::new();
        {
            let kombisensor = kombisensor.lock().unwrap();
            adc_value = kombisensor.sensors[0].get_adc_value().to_string();
            mv_value = kombisensor.sensors[0].get_mv().to_string();
        }
        label_messpunkt_adc.set_text(&adc_value);
        label_messpunkt_mV.set_text(&mv_value);

        let kombisensor = kombisensor.lock().unwrap();
        if kombisensor.get_live_update() {
            Continue(true)
        } else {
            Continue(false)
        }
    }));
}

pub fn launch<T: AsRef<str>>(sensor_type: T, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let button_messpunkt_cancel: gtk::Button = builder.get_object("button_messpunkt_cancel").unwrap();
    let box_calibrator_view: gtk::Box = builder.get_object("box_calibrator_view").unwrap();
    let box_messpunkt_view: gtk::Box = builder.get_object("box_messpunkt_view").unwrap();

    stack_main.set_visible_child(&box_messpunkt_view);

    let mut kombisensor_liveupdate = kombisensor.lock().unwrap();
    kombisensor_liveupdate.set_live_update(true);

    match sensor_type.as_ref() {
        "NO2" => {
            update_widgets(&builder, &kombisensor);

            save_adc_value(&builder, &kombisensor);
        }
        "CO" => {
            update_widgets(&builder, &kombisensor);

            save_adc_value(&builder, &kombisensor);
        }
        _ => { }
    }

    // Weg zurück
    button_messpunkt_cancel.connect_clicked(clone!(kombisensor => move |_| {
        let mut kombisensor = kombisensor.lock().unwrap();

        kombisensor.set_live_update(false);
        stack_main.set_visible_child(&box_calibrator_view);
    }));
}

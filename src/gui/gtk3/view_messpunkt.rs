use gtk;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;
use co_no2_kombisensor::kombisensor::{Kombisensor};


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
            gtk::timeout_add(100, clone!(kombisensor, builder => move || {
                let label_messpunkt_sensor_type: gtk::Label = builder.get_object("label_messpunkt_sensor_type").unwrap();
                let label_messpunkt_adc: gtk::Label = builder.get_object("label_messpunkt_adc").unwrap();
                let label_messpunkt_mV: gtk::Label = builder.get_object("label_messpunkt_mV").unwrap();

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

            let button_messpunkt_save: gtk::Button = builder.get_object("button_messpunkt_save").unwrap();
            button_messpunkt_save.connect_clicked(clone!(kombisensor => move |_| {
                let mut adc_value: i32 = 0;
                {
                    let mut kombisensor = kombisensor.lock().unwrap();
                    kombisensor.set_live_update(false);
                    adc_value = kombisensor.sensors[0].get_adc_value() as i32;
                }

                ::commands::sensor_new_adc_at_nullgas(&kombisensor, adc_value);
            }));
        }
        "CO" => {}
        _ => { }
    }

    // Weg zurück
    button_messpunkt_cancel.connect_clicked(clone!(kombisensor => move |_| {
        let mut kombisensor = kombisensor.lock().unwrap();

        kombisensor.set_live_update(false);
        stack_main.set_visible_child(&box_calibrator_view);
    }));
}

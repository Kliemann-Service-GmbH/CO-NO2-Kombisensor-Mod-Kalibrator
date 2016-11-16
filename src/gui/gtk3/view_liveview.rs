use gtk;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;
use co_no2_kombisensor::kombisensor::{Kombisensor};


pub fn launch(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let box_liveview: gtk::Box = builder.get_object("box_liveview").unwrap();
    let box_index_view: gtk::Box = builder.get_object("box_index_view").unwrap();
    let button_liveview_cancel: gtk::Button = builder.get_object("button_liveview_cancel").unwrap();
    let label_liveview_sensor_type: gtk::Label = builder.get_object("label_liveview_sensor_type").unwrap();

    label_liveview_sensor_type.set_text("CO/NO2 Kombisensoren mit Modbus Interface");

    {
        let mut kombisensor = kombisensor.lock().unwrap();
        kombisensor.set_live_update(true);
    }

    stack_main.set_visible_child(&box_liveview);

    gtk::timeout_add(100, clone!(kombisensor, builder => move || {
        let label_liveview_no2_value: gtk::Label = builder.get_object("label_liveview_no2_value").unwrap();
        let label_liveview_co_value: gtk::Label = builder.get_object("label_liveview_co_value").unwrap();
        let label_liveview_no2_adc_value: gtk::Label = builder.get_object("label_liveview_no2_adc_value").unwrap();
        let label_liveview_co_adc_value: gtk::Label = builder.get_object("label_liveview_co_adc_value").unwrap();
        let label_liveview_no2_mv_value: gtk::Label = builder.get_object("label_liveview_no2_mv_value").unwrap();
        let label_liveview_co_mv_value: gtk::Label = builder.get_object("label_liveview_co_mv_value").unwrap();
        let label_liveview_no2_value: gtk::Label = builder.get_object("label_liveview_no2_value").unwrap();
        let label_liveview_co_value: gtk::Label = builder.get_object("label_liveview_co_value").unwrap();

        let mut no2_adc_value = String::new();
        let mut co_adc_value = String::new();
        let mut no2_mv_value = String::new();
        let mut co_mv_value = String::new();
        let mut no2_value = String::new();
        let mut co_value = String::new();
        {
            let kombisensor = kombisensor.lock().unwrap();
            no2_adc_value = kombisensor.sensors[0].get_adc_value().to_string();
            co_adc_value = kombisensor.sensors[1].get_adc_value().to_string();
            no2_mv_value = kombisensor.sensors[0].get_mv().to_string();
            co_mv_value = kombisensor.sensors[1].get_mv().to_string();
            no2_value = (kombisensor.sensors[0].get_concentration() as u64).to_string();
            co_value = (kombisensor.sensors[1].get_concentration() as u64).to_string();
        }

        label_liveview_no2_adc_value.set_text(&no2_adc_value);
        label_liveview_co_adc_value.set_text(&co_adc_value);
        label_liveview_no2_mv_value.set_text(&no2_mv_value);
        label_liveview_co_mv_value.set_text(&co_mv_value);
        label_liveview_no2_value.set_text(&no2_value);
        label_liveview_co_value.set_text(&co_value);

        let kombisensor = kombisensor.lock().unwrap();
        if kombisensor.get_live_update() {
            Continue(true)
        } else {
            Continue(false)
        }
    }));

    // Weg zurück
    button_liveview_cancel.connect_clicked(clone!(kombisensor => move |_| {
        let mut kombisensor = kombisensor.lock().unwrap();

        kombisensor.set_live_update(false);
        stack_main.set_visible_child(&box_index_view);
    }));
}

use calib_error::CalibError;
use co_no2_kombisensor::sensor::{Sensor, SensorType};
use co_no2_kombisensor::kombisensor::{Kombisensor};
use std::sync::{Arc, Mutex};
use gtk;
use gtk::prelude::*;


fn fill_widgets(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>, sensor_num: usize) {
    let mut sensor_type: &str = "";
    let label_sensor_type: gtk::Label = builder.get_object("label_sensor_type").unwrap();
    if sensor_num == 0 {
        label_sensor_type.set_text("Nemoto™ EC NAP-550 - NO2");
        sensor_type = "NO2";
    } else {
        label_sensor_type.set_text("Nemoto™ EC NAP-505 - CO");
        sensor_type = "CO";
    }

    {
        let kombisensor = kombisensor.lock().unwrap();
        let ref sensor = kombisensor.sensors[sensor_num];
        let adjustment_sensor_minimal: gtk::Adjustment = builder.get_object("adjustment_sensor_minimal").unwrap();
        let adjustment_sensor_maximal: gtk::Adjustment = builder.get_object("adjustment_sensor_maximal").unwrap();
        let adjustment_sensor_concentration_at_nullgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_nullgas").unwrap();
        let adjustment_sensor_concentration_at_messgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_messgas").unwrap();
        let min_value = sensor.get_min_value() as f64;
        let max_value = sensor.get_max_value() as f64 + 1.0;

        adjustment_sensor_minimal.configure(min_value, min_value, max_value, 1.0, 1.0, 1.0);
        adjustment_sensor_maximal.configure(max_value, min_value, max_value, 1.0, 1.0, 1.0);
        adjustment_sensor_concentration_at_nullgas.configure(0.0, min_value, max_value, 1.0, 1.0, 1.0);
        adjustment_sensor_concentration_at_messgas.configure(sensor.get_concentration_at_messgas() as f64,
            min_value, max_value, 1.0, 1.0, 1.0);
    }
}

fn button_messpuntk_nullgas(sensor_type: SensorType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let button_messpunkt_nullgas: gtk::Button = builder.get_object("button_messpunkt_nullgas").unwrap();

    match sensor_type {
        SensorType::RaGasNO2 => {
            button_messpunkt_nullgas.connect_clicked(clone!(builder, kombisensor => move |_| {
                callback_button_messpunkt_nullgas(SensorType::RaGasNO2, &builder, &kombisensor);
            }));
        }
        SensorType::RaGasCO => {}
    }
}

fn callback_button_messpunkt_nullgas(sensor_type: SensorType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    ::gui::gtk3::view_messpunkt::launch(sensor_type, "Nullgas", &builder, &kombisensor);
}

pub fn launch<T: AsRef<str>>(sensor_type: T, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<(), CalibError> {
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let box_index_view: gtk::Box = builder.get_object("box_index_view").unwrap();
    let box_calibrator_view: gtk::Box = builder.get_object("box_calibrator_view").unwrap();
    let button_calibrator_cancel: gtk::Button = builder.get_object("button_calibrator_cancel").unwrap();
    let button_messpunkt_nullgas: gtk::Button = builder.get_object("button_messpunkt_nullgas").unwrap();

    match sensor_type.as_ref() {
        "NO2" => {
            fill_widgets(&builder, &kombisensor, 0);

            button_messpuntk_nullgas(SensorType::RaGasNO2, &builder, &kombisensor);
        },
        "CO" => {
            fill_widgets(&builder, &kombisensor, 1);

            button_messpuntk_nullgas(SensorType::RaGasCO, &builder, &kombisensor);
        },
        _ => {}
    }

    // Stack Layer anzeigen
    stack_main.set_visible_child(&box_calibrator_view);

    // Zurueck Action
    button_calibrator_cancel.connect_clicked(move |_| {
        stack_main.set_visible_child(&box_index_view);
    });

    Ok(())
}

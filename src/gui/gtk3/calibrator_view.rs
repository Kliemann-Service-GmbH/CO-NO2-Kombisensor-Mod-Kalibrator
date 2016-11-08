use calib_error::CalibError;
use co_no2_kombisensor::sensor::{Sensor, SensorType};
use co_no2_kombisensor::kombisensor::{Kombisensor};
use std::sync::{Arc, Mutex};
use gtk;
use gtk::prelude::*;


fn callback_button_messpunkt_nullgas<T: AsRef<str>>(sensor_type: T, builder: &gtk::Builder, kombisensor: Kombisensor) {
    ::gui::gtk3::view_messpunkt::launch(&builder);
}

pub fn launch<T: AsRef<str>>(sensor_type: T, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<(), CalibError> {
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let box_index_view: gtk::Box = builder.get_object("box_index_view").unwrap();
    let box_calibrator_view: gtk::Box = builder.get_object("box_calibrator_view").unwrap();
    let button_calibrator_cancel: gtk::Button = builder.get_object("button_calibrator_cancel").unwrap();
    let button_messpunkt_nullgas: gtk::Button = builder.get_object("button_messpunkt_nullgas").unwrap();
    let kombisensor = kombisensor.lock().unwrap();

    match sensor_type.as_ref() {
        "NO2" => {
            let ref sensor = kombisensor.sensors[0];
            let label_sensor_type: gtk::Label = builder.get_object("label_sensor_type").unwrap();
            let adjustment_sensor_minimal: gtk::Adjustment = builder.get_object("adjustment_sensor_minimal").unwrap();
            let adjustment_sensor_maximal: gtk::Adjustment = builder.get_object("adjustment_sensor_maximal").unwrap();
            let adjustment_sensor_concentration_at_nullgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_nullgas").unwrap();
            let adjustment_sensor_concentration_at_messgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_messgas").unwrap();
            label_sensor_type.set_text("Nemoto™ EC NAP-550 - NO2");
            adjustment_sensor_minimal.configure(sensor.get_min_value() as f64,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);
            adjustment_sensor_maximal.configure(sensor.get_max_value() as f64 + 1.0,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);
            adjustment_sensor_concentration_at_nullgas.configure(0.0,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);
            adjustment_sensor_concentration_at_messgas.configure(sensor.get_concentration_messgas() as f64,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);

            // button_messpunkt_nullgas.connect_clicked(clone!(builder => move |_| {
            //     callback_button_messpunkt_nullgas("NO2", &builder, &kombisensor.downgrade());
            // }));
        },
        "CO" => {
            let ref sensor = kombisensor.sensors[1];
            let label_sensor_type: gtk::Label = builder.get_object("label_sensor_type").unwrap();
            let adjustment_sensor_minimal: gtk::Adjustment = builder.get_object("adjustment_sensor_minimal").unwrap();
            let adjustment_sensor_maximal: gtk::Adjustment = builder.get_object("adjustment_sensor_maximal").unwrap();
            let adjustment_sensor_concentration_at_nullgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_nullgas").unwrap();
            let adjustment_sensor_concentration_at_messgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_messgas").unwrap();
            label_sensor_type.set_text("Nemoto™ EC NAP-505 - CO");
            adjustment_sensor_minimal.configure(sensor.get_min_value() as f64,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);
            adjustment_sensor_maximal.configure(sensor.get_max_value() as f64 + 1.0,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);
            adjustment_sensor_concentration_at_nullgas.configure(0.0,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);
            adjustment_sensor_concentration_at_messgas.configure(sensor.get_concentration_messgas() as f64,
                sensor.get_min_value() as f64, sensor.get_max_value() as f64 + 1.0, 1.0, 1.0, 1.0);

        },
        _ => {}
    }

    stack_main.set_visible_child(&box_calibrator_view);


    button_calibrator_cancel.connect_clicked(move |_| {
        stack_main.set_visible_child(&box_index_view);
    });

    Ok(())
}

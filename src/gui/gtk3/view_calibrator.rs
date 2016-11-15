use calib_error::CalibError;
use co_no2_kombisensor::kombisensor::{Kombisensor};
use co_no2_kombisensor::sensor::{Sensor, SensorType};
use gas::GasType;
use gtk;
use gtk::prelude::*;
use gui::gtk3::glib::translate::ToGlibPtr;
use gui::gtk3::gobject_ffi;
use std::error::Error;
use std::sync::{Arc, Mutex};


fn fill_widgets(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>, sensor_num: usize) {
    let label_sensor_type: gtk::Label = builder.get_object("label_sensor_type").unwrap();
    if sensor_num == 0 {
        label_sensor_type.set_text("Nemoto™ EC NAP-550 - NO2");
    } else {
        label_sensor_type.set_text("Nemoto™ EC NAP-505 - CO");
    }

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

pub fn launch(sensor_type: SensorType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<(), CalibError> {
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let box_index_view: gtk::Box = builder.get_object("box_index_view").unwrap();
    let box_calibrator_view: gtk::Box = builder.get_object("box_calibrator_view").unwrap();
    let button_calibrator_cancel: gtk::Button = builder.get_object("button_calibrator_cancel").unwrap();
    let button_messpunkt_nullgas: gtk::Button = builder.get_object("button_messpunkt_nullgas").unwrap();
    let button_messpunkt_messgas: gtk::Button = builder.get_object("button_messpunkt_messgas").unwrap();
    let button_calibrator_save: gtk::Button = builder.get_object("button_calibrator_save").unwrap();

    match sensor_type {
        SensorType::RaGasNO2 => {
            fill_widgets(&builder, &kombisensor, 0);
        },
        SensorType::RaGasCO => {
            fill_widgets(&builder, &kombisensor, 1);
        },
    }

    // Stack Layer anzeigen
    stack_main.set_visible_child(&box_calibrator_view);

    let id_button_messpunkt_nullgas = button_messpunkt_nullgas.connect_clicked(clone!(builder, kombisensor, sensor_type => move |_| {
        // println!("Button Nullgas, {:?}", sensor_type)
        ::gui::gtk3::view_messpunkt::launch(GasType::Nullgas, &sensor_type, &builder, &kombisensor);
    }));

    let id_button_messpunkt_messgas = button_messpunkt_messgas.connect_clicked(clone!(builder, kombisensor, sensor_type => move |_| {
        // println!("Button Messgas, {:?}", sensor_type)
        ::gui::gtk3::view_messpunkt::launch(GasType::Messgas, &sensor_type, &builder, &kombisensor);
    }));

    let id_button_calibrator_save = button_calibrator_save.connect_clicked(clone!(builder, kombisensor => move |_| {
        let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
        let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
        let mut values: Vec<u16> = vec![0; 30];

        {
            let kombisensor = kombisensor.lock().unwrap();
            values = kombisensor.to_modbus_registers();
        }
        match ::commands::kombisensor_to_modbus(&kombisensor, &values) {
            Ok(()) => {}
            Err(err) => {
                label_info_bar_message.set_text(err.description());
                info_bar.show();
            }
        }
    }));

    // Zurueck Action
    button_calibrator_cancel.connect_clicked(move |_| {
        unsafe {
            if gobject_ffi::g_signal_handler_is_connected(button_messpunkt_nullgas.to_glib_none().0, id_button_messpunkt_nullgas) == 1 {
                gobject_ffi::g_signal_handler_disconnect(button_messpunkt_nullgas.to_glib_none().0, id_button_messpunkt_nullgas);
            }
        }
        unsafe {
            if gobject_ffi::g_signal_handler_is_connected(button_messpunkt_messgas.to_glib_none().0, id_button_messpunkt_messgas) == 1 {
                gobject_ffi::g_signal_handler_disconnect(button_messpunkt_messgas.to_glib_none().0, id_button_messpunkt_messgas);
            }
        }
        unsafe {
            if gobject_ffi::g_signal_handler_is_connected(button_calibrator_save.to_glib_none().0, id_button_calibrator_save) == 1 {
                gobject_ffi::g_signal_handler_disconnect(button_calibrator_save.to_glib_none().0, id_button_calibrator_save);
            }
        }
        stack_main.set_visible_child(&box_index_view);
    });

    Ok(())
}

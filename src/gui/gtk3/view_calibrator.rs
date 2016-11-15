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


fn callback_button_calibrator_save(sensor_type: &SensorType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let adjustment_sensor_concentration_at_messgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_messgas").unwrap();
    let adjustment_sensor_concentration_at_nullgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_nullgas").unwrap();
    let adjustment_sensor_maximal: gtk::Adjustment = builder.get_object("adjustment_sensor_maximal").unwrap();
    let adjustment_sensor_minimal: gtk::Adjustment = builder.get_object("adjustment_sensor_minimal").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let label_info_bar_message: gtk::Label = builder.get_object("label_info_bar_message").unwrap();
    let label_sensor_type: gtk::Label = builder.get_object("label_sensor_type").unwrap();

    let mut values: Vec<u16> = vec![0; 30];
    let mut sensor_num: usize = 0;

    match *sensor_type {
        SensorType::RaGasNO2 => {
            sensor_num = 0;
        }
        SensorType::RaGasCO => {
            sensor_num = 1;
        }
    }

    {
        let mut kombisensor = kombisensor.lock().unwrap();
        kombisensor.sensors[sensor_num].set_min_value(adjustment_sensor_minimal.get_value() as u16);
        kombisensor.sensors[sensor_num].set_max_value(adjustment_sensor_maximal.get_value() as u16);
        kombisensor.sensors[sensor_num].set_concentration_at_nullgas(adjustment_sensor_concentration_at_nullgas.get_value() as u16);
        kombisensor.sensors[sensor_num].set_concentration_at_messgas(adjustment_sensor_concentration_at_messgas.get_value() as u16);

        values = kombisensor.to_modbus_registers();
    }

    match ::commands::kombisensor_to_modbus(&kombisensor, &values) {
        Ok(()) => {}
        Err(err) => {
            label_info_bar_message.set_text(err.description());
            info_bar.show();
        }
    }
}

fn fill_widgets(sensor_type: &SensorType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let adjustment_sensor_concentration_at_messgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_messgas").unwrap();
    let adjustment_sensor_concentration_at_nullgas: gtk::Adjustment = builder.get_object("adjustment_sensor_concentration_at_nullgas").unwrap();
    let adjustment_sensor_maximal: gtk::Adjustment = builder.get_object("adjustment_sensor_maximal").unwrap();
    let adjustment_sensor_minimal: gtk::Adjustment = builder.get_object("adjustment_sensor_minimal").unwrap();
    let label_sensor_type: gtk::Label = builder.get_object("label_sensor_type").unwrap();

    let mut sensor_num: usize = 0;

    match *sensor_type {
        SensorType::RaGasNO2 => {
            label_sensor_type.set_text("Nemoto™ EC NAP-550 - NO2");
            sensor_num = 0;
        }
        SensorType::RaGasCO => {
            label_sensor_type.set_text("Nemoto™ EC NAP-505 - CO");
            sensor_num = 1;
        }
    }

    let kombisensor = kombisensor.lock().unwrap();
    let ref sensor = kombisensor.sensors[sensor_num];
    let min_value = sensor.get_min_value() as f64;
    let max_value = sensor.get_max_value() as f64 + 1.0;

    adjustment_sensor_minimal.configure(min_value, 0.0, max_value + max_value * 0.5, 1.0, 1.0, 1.0);
    adjustment_sensor_maximal.configure(max_value, 0.0, max_value + max_value * 0.5, 1.0, 1.0, 1.0);
    adjustment_sensor_concentration_at_nullgas.configure(sensor.get_concentration_at_nullgas() as f64,
        0.0, max_value + max_value * 0.5, 1.0, 1.0, 1.0);
    adjustment_sensor_concentration_at_messgas.configure(sensor.get_concentration_at_messgas() as f64,
        0.0, max_value + max_value * 0.5, 1.0, 1.0, 1.0);
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
            fill_widgets(&sensor_type, &builder, &kombisensor);
        },
        SensorType::RaGasCO => {
            fill_widgets(&sensor_type, &builder, &kombisensor);
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

    let id_button_calibrator_save = button_calibrator_save.connect_clicked(clone!(builder, kombisensor, sensor_type => move |_| {
        callback_button_calibrator_save(&sensor_type, &builder, &kombisensor);
    }));

    // Zurueck Action
    button_calibrator_cancel.connect_clicked(move |_| {
        use gui::gtk3::libc::c_ulong;

        unsafe {
            if gobject_ffi::g_signal_handler_is_connected(button_messpunkt_nullgas.to_glib_none().0, id_button_messpunkt_nullgas) == 1 {
                gobject_ffi::g_signal_handler_disconnect(button_messpunkt_nullgas.to_glib_none().0, id_button_messpunkt_nullgas as c_ulong);
            }
        }
        unsafe {
            if gobject_ffi::g_signal_handler_is_connected(button_messpunkt_messgas.to_glib_none().0, id_button_messpunkt_messgas) == 1 {
                gobject_ffi::g_signal_handler_disconnect(button_messpunkt_messgas.to_glib_none().0, id_button_messpunkt_messgas as c_ulong);
            }
        }
        unsafe {
            if gobject_ffi::g_signal_handler_is_connected(button_calibrator_save.to_glib_none().0, id_button_calibrator_save) == 1 {
                gobject_ffi::g_signal_handler_disconnect(button_calibrator_save.to_glib_none().0, id_button_calibrator_save as c_ulong);
            }
        }
        stack_main.set_visible_child(&box_index_view);
    });

    Ok(())
}

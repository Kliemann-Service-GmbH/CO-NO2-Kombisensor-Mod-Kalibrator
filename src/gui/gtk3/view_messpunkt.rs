use co_no2_kombisensor::kombisensor::{Kombisensor};
use co_no2_kombisensor::sensor::{SensorType};
use gas::GasType;
use gtk;
use gtk::prelude::*;
use gui::gtk3::glib::translate::ToGlibPtr;
use gui::gtk3::gobject_ffi;
use std::sync::{Arc, Mutex};


#[allow(unused_assignments)]
fn get_adc_value(sensor_type: &SensorType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) -> i32 {
    let mut adc_value: i32 = 0;
    let check_button_adc_manuell: gtk::CheckButton = builder.get_object("check_button_adc_manuell").unwrap();

    let sensor_num: usize = match *sensor_type {
        SensorType::RaGasNO2 => 0,
        SensorType::RaGasCO => 1,
    };

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
        adc_value = kombisensor.sensors[sensor_num].get_adc_value() as i32;
    }

    adc_value
}

#[allow(unused_assignments)]
fn update_widgets(sensor_type: SensorType, gas_type: GasType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    // 100ms Sekunden Timer
    gtk::timeout_add(100, clone!(builder, kombisensor, sensor_type => move || {
        let label_messpunkt_sensor_type: gtk::Label = builder.get_object("label_messpunkt_sensor_type").unwrap();
        let label_messpunkt_gas_type: gtk::Label = builder.get_object("label_messpunkt_gas_type").unwrap();
        let label_messpunkt_adc: gtk::Label = builder.get_object("label_messpunkt_adc").unwrap();
        let label_messpunkt_mv: gtk::Label = builder.get_object("label_messpunkt_mv").unwrap();
        let label_messpunkt_current_adc_value: gtk::Label = builder.get_object("label_messpunkt_current_adc_value").unwrap();

        let mut sensor_num: usize = 0;

        let mut current_adc_value = String::new(); // Wenn der Text dem Widget im Match Arm zugewiesen wird freezed die App
        match sensor_type {
            SensorType::RaGasNO2 => {
                sensor_num = 0;
                label_messpunkt_sensor_type.set_text("NO2 Messzelle");

                match gas_type {
                    GasType::Nullgas => {
                        label_messpunkt_gas_type.set_text("Nullgas");
                        let kombisensor = kombisensor.lock().unwrap();
                        let current_adc_value = kombisensor.sensors[0].get_adc_at_nullgas().to_string();
                        label_messpunkt_current_adc_value.set_text(&current_adc_value);
                    }
                    GasType::Messgas => {
                        label_messpunkt_gas_type.set_text("Messgas");
                        let kombisensor = kombisensor.lock().unwrap();
                        let current_adc_value = kombisensor.sensors[0].get_adc_at_messgas().to_string();
                        label_messpunkt_current_adc_value.set_text(&current_adc_value);
                    }
                }
            }
            SensorType::RaGasCO => {
                sensor_num = 1;
                label_messpunkt_sensor_type.set_text("CO Messzelle");

                match gas_type {
                    GasType::Nullgas => {
                        label_messpunkt_gas_type.set_text("Nullgas");
                        let kombisensor = kombisensor.lock().unwrap();
                        let current_adc_value = kombisensor.sensors[1].get_adc_at_nullgas().to_string();
                        label_messpunkt_current_adc_value.set_text(&current_adc_value);
                    }
                    GasType::Messgas => {
                        label_messpunkt_gas_type.set_text("Messgas");
                        let kombisensor = kombisensor.lock().unwrap();
                        let current_adc_value = kombisensor.sensors[1].get_adc_at_messgas().to_string();
                        label_messpunkt_current_adc_value.set_text(&current_adc_value);
                    }
                }
            }
        }


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
            let adc_value = kombisensor.sensors[sensor_num].get_adc_value() as f64;
            adjustment_sensor_adc_value_at.set_value(adc_value);
        }


        let mut adc_value = String::new();
        let mut mv_value = String::new();
        {
            let kombisensor = kombisensor.lock().unwrap();
            adc_value = kombisensor.sensors[sensor_num].get_adc_value().to_string();
            mv_value = kombisensor.sensors[sensor_num].get_mv().to_string();
        }
        label_messpunkt_adc.set_text(&adc_value);
        label_messpunkt_mv.set_text(&mv_value);

        let kombisensor = kombisensor.lock().unwrap();
        if kombisensor.get_live_update() {
            Continue(true)
        } else {
            Continue(false)
        }
    }));
}

pub fn launch(gas_type: GasType, sensor_type: &SensorType, builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let box_calibrator_view: gtk::Box = builder.get_object("box_calibrator_view").unwrap();
    let box_messpunkt_view: gtk::Box = builder.get_object("box_messpunkt_view").unwrap();
    let button_messpunkt_cancel: gtk::Button = builder.get_object("button_messpunkt_cancel").unwrap();
    let button_messpunkt_save: gtk::Button = builder.get_object("button_messpunkt_save").unwrap();
    let check_button_adc_manuell: gtk::CheckButton = builder.get_object("check_button_adc_manuell").unwrap();
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();

    stack_main.set_visible_child(&box_messpunkt_view);

    // Default deaktiviere dem Manuel ADC Wert
    check_button_adc_manuell.set_active(false);

    let mut kombisensor_liveupdate = kombisensor.lock().unwrap();
    kombisensor_liveupdate.set_live_update(true);

    match *sensor_type {
        SensorType::RaGasNO2 => {
            match gas_type {
                GasType::Nullgas => {
                    update_widgets(SensorType::RaGasNO2, GasType::Nullgas, &builder, &kombisensor);
                }
                GasType::Messgas => {
                    update_widgets(SensorType::RaGasNO2, GasType::Messgas, &builder, &kombisensor);
                }
            }
        }
        SensorType::RaGasCO => {
            match gas_type {
                GasType::Nullgas => {
                    update_widgets(SensorType::RaGasCO, GasType::Nullgas, &builder, &kombisensor);
                }
                GasType::Messgas => {
                    update_widgets(SensorType::RaGasCO, GasType::Messgas, &builder, &kombisensor);
                }
            }
        }
    }

    let id_button_messpunkt_save = button_messpunkt_save.connect_clicked(clone!(gas_type, sensor_type, builder, kombisensor => move |_| {
        let adc_value = get_adc_value(&sensor_type, &builder, &kombisensor);
        println!("ADC: {}, Sensor: {:?}, GasType: {:?}", &adc_value, sensor_type, gas_type);
        let _ = ::commands::sensor_new_adc_at(&gas_type, &sensor_type, &kombisensor, adc_value);
    }));

    // Weg zurück
    button_messpunkt_cancel.connect_clicked(clone!(kombisensor => move |_| {
        use gui::gtk3::libc::c_ulong;

        let mut kombisensor = kombisensor.lock().unwrap();
        // Beende Live Update
        kombisensor.set_live_update(false);

        unsafe {
            if gobject_ffi::g_signal_handler_is_connected(button_messpunkt_save.to_glib_none().0, id_button_messpunkt_save as c_ulong) == 1 {
                gobject_ffi::g_signal_handler_disconnect(button_messpunkt_save.to_glib_none().0, id_button_messpunkt_save as c_ulong);
            }
        }

        stack_main.set_visible_child(&box_calibrator_view);
    }));
}

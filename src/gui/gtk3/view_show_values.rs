use co_no2_kombisensor::kombisensor::{Kombisensor};
use co_no2_kombisensor::sensor::{SensorType};
use gtk;
use gtk::prelude::*;
use gui::gtk3::glib::translate::ToGlibPtr;
use gui::gtk3::gobject_ffi;
use std::sync::{Arc, Mutex};


fn update_widgets(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
}


pub fn launch(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let box_index_view: gtk::Box = builder.get_object("box_index_view").unwrap();
    let box_show_values: gtk::Box = builder.get_object("box_show_values").unwrap();
    let button_show_values_cancel: gtk::Button = builder.get_object("button_show_values_cancel").unwrap();
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let label_show_values_version_major_value: gtk::Label = builder.get_object("label_show_values_version_major_value").unwrap();
    let label_show_values_version_minor_value: gtk::Label = builder.get_object("label_show_values_version_minor_value").unwrap();
    let label_show_values_version_patch_value: gtk::Label = builder.get_object("label_show_values_version_patch_value").unwrap();
    let label_show_values_modbus_address_value: gtk::Label = builder.get_object("label_show_values_modbus_address_value").unwrap();
    let label_show_values_sensor_num_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_num_sensor1").unwrap();
    let label_show_values_sensor_num_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_num_sensor2").unwrap();
    let label_show_values_sensor_min_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_min_sensor1").unwrap();
    let label_show_values_sensor_min_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_min_sensor2").unwrap();
    let label_show_values_sensor_max_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_max_sensor1").unwrap();
    let label_show_values_sensor_max_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_max_sensor2").unwrap();
    let label_show_values_sensor_calib_nullgas_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_calib_nullgas_sensor1").unwrap();
    let label_show_values_sensor_calib_nullgas_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_calib_nullgas_sensor2").unwrap();
    let label_show_values_sensor_calib_messgas_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_calib_messgas_sensor1").unwrap();
    let label_show_values_sensor_calib_messgas_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_calib_messgas_sensor2").unwrap();
    let label_show_values_sensor_conz_nullgas_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_conz_nullgas_sensor1").unwrap();
    let label_show_values_sensor_conz_nullgas_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_conz_nullgas_sensor2").unwrap();
    let label_show_values_sensor_conz_messgas_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_conz_messgas_sensor1").unwrap();
    let label_show_values_sensor_conz_messgas_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_conz_messgas_sensor2").unwrap();
    let label_show_values_sensor_config_sensor1: gtk::Label = builder.get_object("label_show_values_sensor_config_sensor1").unwrap();
    let label_show_values_sensor_config_sensor2: gtk::Label = builder.get_object("label_show_values_sensor_config_sensor2").unwrap();

    stack_main.set_visible_child(&box_show_values);

    {
        let kombisensor = kombisensor.lock().unwrap();
        label_show_values_version_major_value.set_text(kombisensor.get_version().split(".").collect::<Vec<_>>()[0]);
        label_show_values_version_minor_value.set_text(kombisensor.get_version().split(".").collect::<Vec<_>>()[1]);
        label_show_values_version_patch_value.set_text(kombisensor.get_version().split(".").collect::<Vec<_>>()[2]);
        label_show_values_modbus_address_value.set_text(&kombisensor.get_modbus_address().to_string());
        label_show_values_sensor_num_sensor1.set_text(&kombisensor.sensors[0].get_number().to_string());
        label_show_values_sensor_num_sensor2.set_text(&kombisensor.sensors[1].get_number().to_string());
        label_show_values_sensor_min_sensor1.set_text(&kombisensor.sensors[0].get_min_value().to_string());
        label_show_values_sensor_min_sensor2.set_text(&kombisensor.sensors[1].get_min_value().to_string());
        label_show_values_sensor_max_sensor1.set_text(&kombisensor.sensors[0].get_max_value().to_string());
        label_show_values_sensor_max_sensor2.set_text(&kombisensor.sensors[1].get_max_value().to_string());
        label_show_values_sensor_calib_nullgas_sensor1.set_text(&kombisensor.sensors[0].get_adc_at_nullgas().to_string());
        label_show_values_sensor_calib_nullgas_sensor2.set_text(&kombisensor.sensors[1].get_adc_at_nullgas().to_string());
        label_show_values_sensor_calib_messgas_sensor1.set_text(&kombisensor.sensors[0].get_adc_at_messgas().to_string());
        label_show_values_sensor_calib_messgas_sensor2.set_text(&kombisensor.sensors[1].get_adc_at_messgas().to_string());
        label_show_values_sensor_conz_nullgas_sensor1.set_text(&kombisensor.sensors[0].get_concentration_at_nullgas().to_string());
        label_show_values_sensor_conz_nullgas_sensor2.set_text(&kombisensor.sensors[1].get_concentration_at_nullgas().to_string());
        label_show_values_sensor_conz_messgas_sensor1.set_text(&kombisensor.sensors[0].get_concentration_at_messgas().to_string());
        label_show_values_sensor_conz_messgas_sensor2.set_text(&kombisensor.sensors[1].get_concentration_at_messgas().to_string());
        label_show_values_sensor_config_sensor1.set_text(&kombisensor.sensors[0].get_config().to_string());
        label_show_values_sensor_config_sensor2.set_text(&kombisensor.sensors[1].get_config().to_string());
    }

    // Weg zurück
    button_show_values_cancel.connect_clicked(clone!(kombisensor => move |_| {
        //use gui::gtk3::libc::c_ulong;
        //
        //let mut kombisensor = kombisensor.lock().unwrap();
        //// Beende Live Update
        //kombisensor.set_live_update(false);
        //
        //unsafe {
        //    if gobject_ffi::g_signal_handler_is_connected(button_messpunkt_save.to_glib_none().0, id_button_messpunkt_save as c_ulong) == 1 {
        //        gobject_ffi::g_signal_handler_disconnect(button_messpunkt_save.to_glib_none().0, id_button_messpunkt_save as c_ulong);
        //    }
        //}
        //
        stack_main.set_visible_child(&box_index_view);
    }));
}

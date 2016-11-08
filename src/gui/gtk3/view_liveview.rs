use gtk;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;
use std::thread;
use std::time::Duration;
use co_no2_kombisensor::kombisensor::{Kombisensor};

pub fn launch(builder: &gtk::Builder, kombisensor: &Arc<Mutex<Kombisensor>>) {
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let box_liveview: gtk::Box = builder.get_object("box_liveview").unwrap();
    let box_index_view: gtk::Box = builder.get_object("box_index_view").unwrap();
    let button_liveview_cancel: gtk::Button = builder.get_object("button_liveview_cancel").unwrap();

    stack_main.set_visible_child(&box_liveview);

    button_liveview_cancel.connect_clicked(move |_| {
        stack_main.set_visible_child(&box_index_view);
    });

    thread::spawn(clone!(kombisensor => move || {
        loop {
            {
                let mut kombisensor = kombisensor.lock().unwrap();

                println!("{:?}", &kombisensor.get_modbus_address());
            }
            thread::sleep(Duration::from_millis(500));
        }
    }));
}

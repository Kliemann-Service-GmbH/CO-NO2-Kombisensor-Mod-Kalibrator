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

    {
        let mut kombisensor = kombisensor.lock().unwrap();

        kombisensor.set_live_update(true);
    }

    stack_main.set_visible_child(&box_liveview);

    button_liveview_cancel.connect_clicked(clone!(kombisensor => move |_| {
        let mut kombisensor = kombisensor.lock().unwrap();

        kombisensor.set_live_update(false);
        stack_main.set_visible_child(&box_index_view);
    }));
}

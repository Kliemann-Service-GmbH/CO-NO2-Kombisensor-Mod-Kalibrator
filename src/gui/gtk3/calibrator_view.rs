use gtk;
use gtk::prelude::*;


pub fn launch<T: AsRef<str>>(sensor_type: T, builder: &gtk::Builder) {
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();
    let box_index_view: gtk::Box = builder.get_object("box_index_view").unwrap();
    let box_calibrator_view: gtk::Box = builder.get_object("box_calibrator_view").unwrap();
    let button_calibrator_cancel: gtk::Button = builder.get_object("button_calibrator_cancel").unwrap();
    let label_sensor_type: gtk::Label = builder.get_object("label_sensor_type").unwrap();

    match sensor_type.as_ref() {
        "NO2" => {
            label_sensor_type.set_text("Nemoto™ EC NAP-550 - NO2");
        },
        "CO" => {
            label_sensor_type.set_text("Nemoto™ EC NAP-505 - CO");
        },
        _ => {}
    }
    stack_main.set_visible_child(&box_calibrator_view);

    button_calibrator_cancel.connect_clicked(move |_| {
        stack_main.set_visible_child(&box_index_view);
    });

}

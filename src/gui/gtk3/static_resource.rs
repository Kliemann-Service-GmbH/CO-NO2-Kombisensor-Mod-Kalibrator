extern crate gio;
extern crate glib;
extern crate libc;


pub fn init() {
    let resource_data: &[u8] = &include_bytes!("../../resources/resources.gresource")[..];
    let bytes = glib::Bytes::from(&resource_data);
    let resource = gio::Resource::new_from_data(&bytes).unwrap();
    gio::resources_register(&resource);
}

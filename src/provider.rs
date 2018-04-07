extern crate dbus;

use self::dbus::{
    Connection,
    BusType,
    Message
};

pub struct dbus_provider {
    con: Connection,
}

pub struct interface {
    ip: u32,
    dns: u32,

}

impl dbus_provider {
    fn get_interfaces() {
        //TODO: return array or map of interfaces?
    }
    fn modify_interface() {
        //TODO: take an interface struct and set nm interface with provided information.
    }
}


pub fn init_provider() -> dbus_provider{
    let c = Connection::get_private(BusType::Session).unwrap();
    let provider = dbus_provider{
        con: c,
    };
    return provider;
}
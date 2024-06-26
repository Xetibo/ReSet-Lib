use std::time::Duration;

use dbus::{
    arg::{Append, AppendAll, Arg, Get, ReadAll},
    blocking::Connection,
    Path,
};

pub const DBUS_PATH: &str = "/org/Xetibo/ReSet/Daemon";
pub const WIRELESS: &str = "org.Xetibo.ReSet.Wireless";
pub const BLUETOOTH: &str = "org.Xetibo.ReSet.Bluetooth";
pub const AUDIO: &str = "org.Xetibo.ReSet.Audio";
pub const BASE: &str = "org.Xetibo.ReSet.Daemon";

pub fn call_system_dbus_method<I: AppendAll + 'static, O: ReadAll + 'static>(
    name: &str,
    object: Path<'static>,
    function: &str,
    proxy_name: &str,
    params: I,
    time: u64,
) -> Result<O, dbus::Error> {
    let conn = Connection::new_system().unwrap();
    let proxy = conn.with_proxy(name, object, Duration::from_millis(time));
    let result: Result<O, dbus::Error> = proxy.method_call(proxy_name, function, params);
    result
}

pub fn get_system_dbus_property<I: AppendAll, O: for<'a> Get<'a> + 'static>(
    name: &str,
    object: Path<'static>,
    interface: &str,
    property: &str,
) -> Result<O, dbus::Error> {
    let conn = Connection::new_system().unwrap();
    let proxy = conn.with_proxy(name, object, Duration::from_millis(1000));
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

    let result: Result<O, dbus::Error> = proxy.get(interface, property);
    result
}

pub fn set_system_dbus_property<I: Arg + Append>(
    name: &str,
    object: Path<'static>,
    interface: &str,
    property: &str,
    value: I,
) -> Result<(), dbus::Error> {
    let conn = Connection::new_system().unwrap();
    let proxy = conn.with_proxy(name, object, Duration::from_millis(1000));
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

    let result: Result<(), dbus::Error> = proxy.set(interface, property, value);
    result
}

pub fn call_session_dbus_method<
    I: AppendAll + Sync + Send + 'static,
    O: ReadAll + Sync + Send + 'static,
>(
    name: &str,
    object: Path<'static>,
    function: &str,
    proxy_name: &str,
    params: I,
) -> Result<O, dbus::Error> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(name, object, Duration::from_millis(1000));
    let result: Result<O, dbus::Error> = proxy.method_call(proxy_name, function, params);
    result
}

pub fn call_reset_dbus_method<
    I: AppendAll + Sync + Send + 'static,
    O: ReadAll + Sync + Send + 'static,
>(
    interface: &str,
    function: &str,
    params: I,
) -> Result<O, dbus::Error> {
    let conn = Connection::new_session().unwrap();
    let proxy = conn.with_proxy(BASE, DBUS_PATH, Duration::from_millis(1000));
    let result: Result<O, dbus::Error> = proxy.method_call(
        "org.Xetibo.ReSet".to_string() + "." + interface,
        function,
        params,
    );
    result
}

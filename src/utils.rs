use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread,
    time::Duration,
};

use dbus::{
    arg::{Append, AppendAll, Arg, Get, ReadAll},
    blocking::Connection,
    Path,
};

use crate::signals::GetVal;

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
    let proxy = conn.with_proxy(
        "org.Xetibo.ReSetDaemon",
        "/org/Xetibo/ResetDaemon",
        Duration::from_millis(1000),
    );
    let result: Result<O, dbus::Error> =
        proxy.method_call("org.Xetibo.ReSet".to_string() + interface, function, params);
    result
}

pub enum Events<AddedType: ReadAll + AppendAll, RemovedType: ReadAll + AppendAll> {
    AddedEvent(AddedType),
    RemovedEvent(RemovedType),
}

pub fn start_event_listener<
    AddedType: ReadAll + AppendAll + Send + Sync + 'static,
    RemovedType: ReadAll + AppendAll + Send + Sync + 'static,
    AddedEvent: ReadAll + AppendAll + dbus::message::SignalArgs + GetVal<AddedType>,
    RemovedEvent: ReadAll + AppendAll + dbus::message::SignalArgs + GetVal<RemovedType>,
>(
    interface: String,
    active_listener: Arc<AtomicBool>,
    sender: Arc<Sender<Events<AddedType, RemovedType>>>,
) -> Result<(), dbus::Error> {
    thread::spawn(move || {
        let added_sender = sender.clone();
        let removed_sender = sender.clone();
        let conn = Connection::new_system().unwrap();
        let mr = AddedEvent::match_rule(
            Some(&("org.Xetibo.ReSet".to_string() + &interface).into()),
            Some(&Path::from("/org/Xetibo/ReSet")),
        )
        .static_clone();
        let mrb = RemovedEvent::match_rule(
            Some(&("org.Xetibo.ReSet".to_string() + &interface).into()),
            Some(&Path::from("/org/Xetibo/ReSet")),
        )
        .static_clone();
        let res = conn.add_match(mr, move |ir: AddedEvent, _, _| {
            let res = added_sender.send(Events::AddedEvent(ir.get_value()));
            if res.is_err() {
                return false;
            }
            true
        });
        if res.is_err() {
            return Err(dbus::Error::new_custom(
                "SignalMatchFailed",
                "Failed to match signal on ReSet.",
            ));
        }
        let res = conn.add_match(mrb, move |ir: RemovedEvent, _, _| {
            let res = removed_sender.send(Events::RemovedEvent(ir.get_value()));
            if res.is_err() {
                return false;
            }
            true
        });
        if res.is_err() {
            return Err(dbus::Error::new_custom(
                "SignalMatchFailed",
                "Failed to match signal on ReSet.",
            ));
        }
        active_listener.store(true, Ordering::SeqCst);
        loop {
            let _ = conn.process(Duration::from_millis(1000))?;
            if !active_listener.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_millis(1000));
        }
        Ok(())
    });
    Ok(())
}

pub fn stop_listener(active_listener: Arc<AtomicBool>) {
    active_listener.store(false, Ordering::SeqCst);
}

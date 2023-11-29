// This code was autogenerated with `dbus-codegen-rust -s -d org.bluez -c blocking`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;
use dbus::{self};

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
{
    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .map(|r: (String,)| r.0)
    }
}

pub trait OrgFreedesktopDBusObjectManager {
    fn get_managed_objects(
        &self,
    ) -> Result<
        ::std::collections::HashMap<
            dbus::Path<'static>,
            ::std::collections::HashMap<String, arg::PropMap>,
        >,
        dbus::Error,
    >;
}

#[derive(Debug)]
pub struct BluetoothDeviceAdded {
    pub object: dbus::Path<'static>,
    pub interfaces: ::std::collections::HashMap<String, arg::PropMap>,
}

impl arg::AppendAll for BluetoothDeviceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.object, i);
        arg::RefArg::append(&self.interfaces, i);
    }
}

impl arg::ReadAll for BluetoothDeviceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(BluetoothDeviceAdded {
            object: i.read()?,
            interfaces: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for BluetoothDeviceAdded {
    const NAME: &'static str = "InterfacesAdded";
    const INTERFACE: &'static str = "org.freedesktop.DBus.ObjectManager";
}

#[derive(Debug)]
pub struct BluetoothDeviceRemoved {
    pub object: dbus::Path<'static>,
    pub interfaces: Vec<String>,
}

impl arg::AppendAll for BluetoothDeviceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.object, i);
        arg::RefArg::append(&self.interfaces, i);
    }
}

impl arg::ReadAll for BluetoothDeviceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(BluetoothDeviceRemoved {
            object: i.read()?,
            interfaces: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for BluetoothDeviceRemoved {
    const NAME: &'static str = "InterfacesRemoved";
    const INTERFACE: &'static str = "org.freedesktop.DBus.ObjectManager";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopDBusObjectManager for blocking::Proxy<'a, C>
{
    fn get_managed_objects(
        &self,
    ) -> Result<
        ::std::collections::HashMap<
            dbus::Path<'static>,
            ::std::collections::HashMap<String, arg::PropMap>,
        >,
        dbus::Error,
    > {
        self.method_call(
            "org.freedesktop.DBus.ObjectManager",
            "GetManagedObjects",
            (),
        )
        .map(
            |r: (
                ::std::collections::HashMap<
                    dbus::Path<'static>,
                    ::std::collections::HashMap<String, arg::PropMap>,
                >,
            )| r.0,
        )
    }
}

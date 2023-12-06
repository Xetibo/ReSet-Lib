use dbus::{
    arg::{self, PropMap},
    Path,
};

use crate::{
    bluetooth::bluetooth_structures::BluetoothDevice,
    network::network_structures::{AccessPoint, WifiDevice},
};

pub trait GetVal<T> {
    fn get_value(&self) -> T;
}

#[derive(Debug)]
pub struct BluetoothDeviceAdded {
    pub bluetooth_device: BluetoothDevice,
}

impl arg::AppendAll for BluetoothDeviceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.bluetooth_device, i);
    }
}

impl arg::ReadAll for BluetoothDeviceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(BluetoothDeviceAdded {
            bluetooth_device: i.read()?,
        })
    }
}

impl GetVal<(BluetoothDevice,)> for BluetoothDeviceAdded {
    fn get_value(&self) -> (BluetoothDevice,) {
        (self.bluetooth_device.clone(),)
    }
}

impl dbus::message::SignalArgs for BluetoothDeviceAdded {
    const NAME: &'static str = "BluetoothDeviceAdded";
    const INTERFACE: &'static str = "org.Xetibo.ReSetBluetooth";
}

#[derive(Debug)]
pub struct BluetoothDeviceChanged {
    pub bluetooth_device: BluetoothDevice,
}

impl arg::AppendAll for BluetoothDeviceChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.bluetooth_device, i);
    }
}

impl arg::ReadAll for BluetoothDeviceChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(BluetoothDeviceChanged {
            bluetooth_device: i.read()?,
        })
    }
}

impl GetVal<(BluetoothDevice,)> for BluetoothDeviceChanged {
    fn get_value(&self) -> (BluetoothDevice,) {
        (self.bluetooth_device.clone(),)
    }
}

impl dbus::message::SignalArgs for BluetoothDeviceChanged {
    const NAME: &'static str = "BluetoothDeviceChanged";
    const INTERFACE: &'static str = "org.Xetibo.ReSetBluetooth";
}

#[derive(Debug)]
pub struct BluetoothDeviceRemoved {
    pub bluetooth_device: Path<'static>,
}

impl arg::AppendAll for BluetoothDeviceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.bluetooth_device, i);
    }
}

impl arg::ReadAll for BluetoothDeviceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(BluetoothDeviceRemoved {
            bluetooth_device: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for BluetoothDeviceRemoved {
    const NAME: &'static str = "BluetoothDeviceRemoved";
    const INTERFACE: &'static str = "org.Xetibo.ReSetBluetooth";
}

impl GetVal<(Path<'static>,)> for BluetoothDeviceRemoved {
    fn get_value(&self) -> (Path<'static>,) {
        (self.bluetooth_device.clone(),)
    }
}

#[derive(Debug)]
pub struct AccessPointAdded {
    pub access_point: AccessPoint,
}

impl arg::AppendAll for AccessPointAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.access_point, i);
    }
}

impl arg::ReadAll for AccessPointAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(AccessPointAdded {
            access_point: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for AccessPointAdded {
    const NAME: &'static str = "AccessPointAdded";
    const INTERFACE: &'static str = "org.Xetibo.ReSetWireless";
}

impl GetVal<(AccessPoint,)> for AccessPointAdded {
    fn get_value(&self) -> (AccessPoint,) {
        (self.access_point.clone(),)
    }
}

#[derive(Debug)]
pub struct AccessPointChanged {
    pub access_point: AccessPoint,
}

impl arg::AppendAll for AccessPointChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.access_point, i);
    }
}

impl arg::ReadAll for AccessPointChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(AccessPointChanged {
            access_point: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for AccessPointChanged {
    const NAME: &'static str = "AccessPointChanged";
    const INTERFACE: &'static str = "org.Xetibo.ReSetWireless";
}

impl GetVal<(AccessPoint,)> for AccessPointChanged {
    fn get_value(&self) -> (AccessPoint,) {
        (self.access_point.clone(),)
    }
}

#[derive(Debug)]
pub struct AccessPointRemoved {
    pub access_point: Path<'static>,
}

impl arg::AppendAll for AccessPointRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.access_point, i);
    }
}

impl arg::ReadAll for AccessPointRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(AccessPointRemoved {
            access_point: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for AccessPointRemoved {
    const NAME: &'static str = "AccessPointRemoved";
    const INTERFACE: &'static str = "org.Xetibo.ReSetWireless";
}

impl GetVal<(Path<'static>,)> for AccessPointRemoved {
    fn get_value(&self) -> (Path<'static>,) {
        (self.access_point.clone(),)
    }
}

#[derive(Debug)]
pub struct WifiDeviceChanged {
    pub wifi_device: WifiDevice,
}

impl arg::AppendAll for WifiDeviceChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.wifi_device, i);
    }
}

impl arg::ReadAll for WifiDeviceChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(WifiDeviceChanged {
            wifi_device: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for WifiDeviceChanged {
    const NAME: &'static str = "WifiDeviceChanged";
    const INTERFACE: &'static str = "org.Xetibo.ReSetWireless";
}

impl GetVal<(WifiDevice,)> for WifiDeviceChanged {
    fn get_value(&self) -> (WifiDevice,) {
        (self.wifi_device.clone(),)
    }
}

#[derive(Debug)]
pub struct PropertiesChanged {
    pub interface: String,
    pub map: PropMap,
    pub invalid: Vec<String>,
}

impl arg::AppendAll for PropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface, i);
        arg::RefArg::append(&self.map, i);
        arg::RefArg::append(&self.invalid, i);
    }
}

impl arg::ReadAll for PropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(PropertiesChanged {
            interface: i.read()?,
            map: i.read()?,
            invalid: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for PropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

use dbus::{arg, Path};

use crate::{bluetooth::bluetooth::BluetoothDevice, network::network::AccessPoint};

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

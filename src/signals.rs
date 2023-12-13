use dbus::{
    arg::{self, Append, PropMap},
    Path,
};

use crate::{
    audio::audio_structures::{InputStream, OutputStream, Sink, Source},
    bluetooth::bluetooth_structures::BluetoothDevice,
    network::network_structures::{AccessPoint, WifiDevice},
    utils::{AUDIO, BLUETOOTH, WIRELESS},
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
    const INTERFACE: &'static str = BLUETOOTH;
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
    const INTERFACE: &'static str = BLUETOOTH;
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
    const INTERFACE: &'static str = BLUETOOTH;
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
    const INTERFACE: &'static str = WIRELESS;
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
    const INTERFACE: &'static str = WIRELESS;
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
    const INTERFACE: &'static str = WIRELESS;
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
    const INTERFACE: &'static str = WIRELESS;
}

impl GetVal<(WifiDevice,)> for WifiDeviceChanged {
    fn get_value(&self) -> (WifiDevice,) {
        (self.wifi_device.clone(),)
    }
}

#[derive(Debug)]
pub struct WifiDeviceRemoved {
    pub path: dbus::Path<'static>,
}

impl arg::AppendAll for WifiDeviceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.path, i);
    }
}

impl arg::ReadAll for WifiDeviceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(WifiDeviceRemoved { path: i.read()? })
    }
}

impl dbus::message::SignalArgs for WifiDeviceRemoved {
    const NAME: &'static str = "WifiDeviceRemoved";
    const INTERFACE: &'static str = WIRELESS;
}

#[derive(Debug)]
pub struct WifiDeviceAdded {
    pub device: WifiDevice,
}

impl arg::AppendAll for WifiDeviceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.device, i);
    }
}

impl arg::ReadAll for WifiDeviceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(WifiDeviceAdded { device: i.read()? })
    }
}

impl dbus::message::SignalArgs for WifiDeviceAdded {
    const NAME: &'static str = "WifiDeviceAdded";
    const INTERFACE: &'static str = WIRELESS;
}

#[derive(Debug)]
pub struct WifiDeviceReset {
    pub devices: Vec<WifiDevice>,
}

impl arg::AppendAll for WifiDeviceReset {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.devices, i);
    }
}

impl arg::ReadAll for WifiDeviceReset {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(WifiDeviceReset { devices: i.read()? })
    }
}

impl dbus::message::SignalArgs for WifiDeviceReset {
    const NAME: &'static str = "WifiDeviceReset";
    const INTERFACE: &'static str = WIRELESS;
}

#[derive(Debug)]
pub struct SinkAdded {
    pub sink: Sink,
}

impl arg::AppendAll for SinkAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.sink.append_by_ref(i);
    }
}

impl arg::ReadAll for SinkAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SinkAdded { sink: i.read()? })
    }
}

impl dbus::message::SignalArgs for SinkAdded {
    const NAME: &'static str = "SinkAdded";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(Sink,)> for SinkAdded {
    fn get_value(&self) -> (Sink,) {
        (self.sink.clone(),)
    }
}

#[derive(Debug)]
pub struct SinkChanged {
    pub sink: Sink,
}

impl arg::AppendAll for SinkChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.sink.append_by_ref(i);
    }
}

impl arg::ReadAll for SinkChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SinkChanged { sink: i.read()? })
    }
}

impl dbus::message::SignalArgs for SinkChanged {
    const NAME: &'static str = "SinkChanged";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(Sink,)> for SinkChanged {
    fn get_value(&self) -> (Sink,) {
        (self.sink.clone(),)
    }
}

#[derive(Debug)]
pub struct SinkRemoved {
    pub index: u32,
}

impl arg::AppendAll for SinkRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for SinkRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SinkRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for SinkRemoved {
    const NAME: &'static str = "SinkRemoved";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(u32,)> for SinkRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index,)
    }
}

#[derive(Debug)]
pub struct InputStreamAdded {
    pub stream: InputStream,
}

impl arg::AppendAll for InputStreamAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for InputStreamAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(InputStreamAdded { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for InputStreamAdded {
    const NAME: &'static str = "InputStreamAdded";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(InputStream,)> for InputStreamAdded {
    fn get_value(&self) -> (InputStream,) {
        (self.stream.clone(),)
    }
}

#[derive(Debug)]
pub struct InputStreamChanged {
    pub stream: InputStream,
}

impl arg::AppendAll for InputStreamChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for InputStreamChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(InputStreamChanged { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for InputStreamChanged {
    const NAME: &'static str = "InputStreamChanged";
    const INTERFACE: &'static str = AUDIO;
}

#[derive(Debug)]
pub struct InputStreamRemoved {
    pub index: u32,
}

impl arg::AppendAll for InputStreamRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for InputStreamRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(InputStreamRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for InputStreamRemoved {
    const NAME: &'static str = "InputStreamRemoved";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(u32,)> for InputStreamRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index,)
    }
}

#[derive(Debug)]
pub struct SourceAdded {
    pub source: Source,
}

impl arg::AppendAll for SourceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.source.append_by_ref(i);
    }
}

impl arg::ReadAll for SourceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SourceAdded { source: i.read()? })
    }
}

impl dbus::message::SignalArgs for SourceAdded {
    const NAME: &'static str = "SourceAdded";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(Source,)> for SourceAdded {
    fn get_value(&self) -> (Source,) {
        (self.source.clone(),)
    }
}

#[derive(Debug)]
pub struct SourceChanged {
    pub source: Source,
}

impl arg::AppendAll for SourceChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.source.append_by_ref(i);
    }
}

impl arg::ReadAll for SourceChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SourceChanged { source: i.read()? })
    }
}

impl dbus::message::SignalArgs for SourceChanged {
    const NAME: &'static str = "SourceChanged";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(Source,)> for SourceChanged {
    fn get_value(&self) -> (Source,) {
        (self.source.clone(),)
    }
}

#[derive(Debug)]
pub struct SourceRemoved {
    pub index: u32,
}

impl arg::AppendAll for SourceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for SourceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(SourceRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for SourceRemoved {
    const NAME: &'static str = "SourceRemoved";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(u32,)> for SourceRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index,)
    }
}

#[derive(Debug)]
pub struct OutputStreamAdded {
    pub stream: OutputStream,
}

impl arg::AppendAll for OutputStreamAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for OutputStreamAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OutputStreamAdded { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for OutputStreamAdded {
    const NAME: &'static str = "OutputStreamAdded";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(OutputStream,)> for OutputStreamAdded {
    fn get_value(&self) -> (OutputStream,) {
        (self.stream.clone(),)
    }
}

#[derive(Debug)]
pub struct OutputStreamChanged {
    pub stream: OutputStream,
}

impl arg::AppendAll for OutputStreamChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        self.stream.append_by_ref(i);
    }
}

impl arg::ReadAll for OutputStreamChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OutputStreamChanged { stream: i.read()? })
    }
}

impl dbus::message::SignalArgs for OutputStreamChanged {
    const NAME: &'static str = "OutputStreamChanged";
    const INTERFACE: &'static str = AUDIO;
}

#[derive(Debug)]
pub struct OutputStreamRemoved {
    pub index: u32,
}

impl arg::AppendAll for OutputStreamRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        self.index.append_by_ref(i);
    }
}

impl arg::ReadAll for OutputStreamRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OutputStreamRemoved { index: i.read()? })
    }
}

impl dbus::message::SignalArgs for OutputStreamRemoved {
    const NAME: &'static str = "OutputStreamRemoved";
    const INTERFACE: &'static str = AUDIO;
}

impl GetVal<(u32,)> for OutputStreamRemoved {
    fn get_value(&self) -> (u32,) {
        (self.index,)
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

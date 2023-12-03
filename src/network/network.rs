use core::fmt;
use std::any;

use dbus::{
    arg::{self, Append, Arg, ArgType, Get, IterAppend, RefArg},
    Path, Signature,
};

#[derive(Debug, Clone)]
pub struct Error {
    pub message: &'static str,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionError {
    pub method: &'static str,
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not {} Access Point.", self.method)
    }
}

#[derive(PartialEq, Eq)]
pub enum DeviceType {
    UNKNOWN,
    GENERIC = 1,
    WIFI = 2,
    BT = 5,
    DUMMY = 22,
    OTHER,
}

#[derive(Default, Copy, Clone)]
pub enum WifiStrength {
    Excellent,
    Ok,
    Weak,
    #[default]
    None,
}

impl WifiStrength {
    pub fn from_u8(num: u8) -> Self {
        match num {
            0..=42 => WifiStrength::Weak,
            43..=84 => WifiStrength::Ok,
            85..=128 => WifiStrength::Excellent,
            _ => WifiStrength::None,
        }
    }
}

#[allow(dead_code)]
impl DeviceType {
    pub fn from_u32(num: u32) -> Self {
        match num {
            0 => DeviceType::UNKNOWN,
            1 => DeviceType::GENERIC,
            2 => DeviceType::WIFI,
            5 => DeviceType::BT,
            22 => DeviceType::DUMMY,
            _ => DeviceType::OTHER,
        }
    }
    pub fn _to_u32(&self) -> u32 {
        match self {
            DeviceType::UNKNOWN => 0,
            DeviceType::GENERIC => 1,
            DeviceType::WIFI => 2,
            DeviceType::BT => 5,
            DeviceType::DUMMY => 22,
            DeviceType::OTHER => 90,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AccessPoint {
    pub ssid: Vec<u8>,
    pub strength: u8,
    pub associated_connection: Path<'static>,
    pub dbus_path: Path<'static>,
    pub stored: bool,
}

unsafe impl Send for AccessPoint {}
unsafe impl Sync for AccessPoint {}

impl Append for AccessPoint {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            let sig = unsafe { Signature::from_slice_unchecked("y\0") };
            i.append_array(&sig, |i| {
                for byte in self.ssid.iter() {
                    i.append(byte);
                }
            });
            i.append(self.strength);
            i.append(&self.associated_connection);
            i.append(&self.dbus_path);
            i.append(self.stored);
        });
    }
}

impl<'a> Get<'a> for AccessPoint {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (ssid, strength, associated_connection, dbus_path, stored) =
            <(Vec<u8>, u8, Path<'static>, Path<'static>, bool)>::get(i)?;
        Some(AccessPoint {
            ssid,
            strength,
            associated_connection,
            dbus_path,
            stored,
        })
    }
}

impl Arg for AccessPoint {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ayyoob)\0") }
    }
}

impl RefArg for AccessPoint {
    fn arg_type(&self) -> ArgType {
        ArgType::Struct
    }
    fn signature(&self) -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ayyoob)\0") }
    }
    fn append(&self, i: &mut IterAppend) {
        self.append_by_ref(i);
    }
    #[inline]
    fn as_any(&self) -> &dyn any::Any
    where
        Self: 'static,
    {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn any::Any
    where
        Self: 'static,
    {
        self
    }

    fn box_clone(&self) -> Box<dyn RefArg + 'static> {
        Box::new(self.clone())
    }
}

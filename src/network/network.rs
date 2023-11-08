use core::fmt;

use dbus::{
    arg::{self, Append, Arg, ArgType, Get},
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
    method: &'static str,
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

#[allow(dead_code)]
impl DeviceType {
    fn from_u32(num: u32) -> Self {
        match num {
            0 => DeviceType::UNKNOWN,
            1 => DeviceType::GENERIC,
            2 => DeviceType::WIFI,
            5 => DeviceType::BT,
            22 => DeviceType::DUMMY,
            _ => DeviceType::OTHER,
        }
    }
    fn _to_u32(&self) -> u32 {
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

#[derive(Debug, Clone)]
pub struct AccessPoint {
    pub ssid: Vec<u8>,
    pub strength: u8,
    pub associated_connection: Path<'static>,
    pub dbus_path: Path<'static>,
}

impl Append for AccessPoint {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            let sig = unsafe { Signature::from_slice_unchecked("y\0") };
            i.append_array(&sig, |i| {
                for byte in self.ssid.iter() {
                    i.append(byte);
                }
            });
            i.append(&self.strength);
            i.append(&self.associated_connection);
            i.append(&self.dbus_path);
        });
    }
}

impl<'a> Get<'a> for AccessPoint {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (ssid, strength, associated_connection, dbus_path) =
            <(Vec<u8>, u8, Path<'static>, Path<'static>)>::get(i)?;
        Some(AccessPoint {
            ssid,
            strength,
            associated_connection,
            dbus_path,
        })
    }
}

impl Arg for AccessPoint {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ayyoo)\0") }
    }
}

use dbus::{
    arg::{self, Append, Arg, ArgType, Get},
    Signature,
};
use pulse::context::introspect::{
    CardInfo, CardProfileInfo, SinkInfo, SinkInputInfo, SourceInfo, SourceOutputInfo,
};

use crate::network::connection::Enum;

#[derive(Debug)]
pub struct PulseError(pub &'static str);

pub enum Status {
    RUNNING,
    SUSPENDED,
    IDLE,
}

pub enum Channel {
    MONO,
    STEREO,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Volume {
    value: u32,
}

impl Volume {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            i if i < 0 => None,
            i if i > 100 => None,
            _ => Some(Volume {
                value: value as u32,
            }),
        }
    }

    pub fn add(&mut self, value: i32) {
        let temporary = self.value as i32 + value;
        match temporary {
            i if i > 100 => self.value = 100,
            i if i < 0 => self.value = 0,
            _ => self.value = temporary as u32,
        }
    }
}

#[test]
fn volume_test() {
    let failvolume = Volume::from_i32(101);
    assert!(failvolume.is_none());
    let failvolume = Volume::from_i32(-1);
    assert!(failvolume.is_none());
    let volume = Volume::from_i32(78);
    assert!(volume.is_some());
    let mut volume = volume.unwrap();
    volume.add(200);
    assert_eq!(Volume::from_i32(100).unwrap(), volume);
}

#[derive(Debug, Clone, Default)]
pub enum DeviceState {
    #[default]
    Idle,
    Invalid,
    Suspended,
    Running,
}

impl Enum for DeviceState {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => DeviceState::Idle,
            1 => DeviceState::Invalid,
            2 => DeviceState::Suspended,
            _ => DeviceState::Running,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            DeviceState::Idle => 0,
            DeviceState::Invalid => 1,
            DeviceState::Suspended => 2,
            DeviceState::Running => 3,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Source {
    pub index: u32,
    pub name: String,
    pub alias: String,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
    pub active: i32,
}

unsafe impl Send for Source {}
unsafe impl Sync for Source {}

impl Append for Source {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(self.index);
            i.append(&self.name);
            i.append(&self.alias);
            i.append(self.channels);
            i.append(&self.volume);
            i.append(self.muted);
            i.append(self.active);
        });
    }
}

impl<'a> Get<'a> for Source {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, alias, channels, volume, muted, active) =
            <(u32, String, String, u16, Vec<u32>, bool, i32)>::get(i)?;
        Some(Self {
            index,
            name,
            alias,
            channels,
            volume,
            muted,
            active,
        })
    }
}

impl Arg for Source {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussqaubi)\0") }
    }
}

impl From<&SourceInfo<'_>> for Source {
    fn from(value: &SourceInfo<'_>) -> Self {
        let name = if let Some(name_opt) = &value.name {
            String::from(name_opt.clone())
        } else {
            String::from("")
        };
        let alias = if let Some(alias_opt) = &value.description {
            String::from(alias_opt.clone())
        } else {
            String::from("")
        };
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        Self {
            index: value.index,
            name,
            alias,
            channels,
            volume,
            muted: value.mute,
            active: value.state as i32,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Sink {
    pub index: u32,
    pub name: String,
    pub alias: String,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
    pub active: i32,
}

unsafe impl Send for Sink {}
unsafe impl Sync for Sink {}

impl Append for Sink {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(self.index);
            i.append(&self.name);
            i.append(&self.alias);
            i.append(self.channels);
            i.append(&self.volume);
            i.append(self.muted);
            i.append(self.active);
        });
    }
}

impl<'a> Get<'a> for Sink {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, alias, channels, volume, muted, active) =
            <(u32, String, String, u16, Vec<u32>, bool, i32)>::get(i)?;
        Some(Self {
            index,
            name,
            alias,
            channels,
            volume,
            muted,
            active,
        })
    }
}

impl Arg for Sink {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussqaubi)\0") }
    }
}

impl From<&SinkInfo<'_>> for Sink {
    fn from(value: &SinkInfo<'_>) -> Self {
        let name = if let Some(name_opt) = &value.name {
            String::from(name_opt.clone())
        } else {
            String::from("")
        };
        let alias = if let Some(alias_opt) = &value.description {
            String::from(alias_opt.clone())
        } else {
            String::from("")
        };
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        Self {
            index: value.index,
            name,
            alias,
            channels,
            volume,
            muted: value.mute,
            active: value.state as i32,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InputStream {
    pub index: u32,
    pub name: String,
    pub application_name: String,
    pub sink_index: u32,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
    pub corked: bool,
}

impl Append for InputStream {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(self.index);
            i.append(&self.name);
            i.append(&self.application_name);
            i.append(self.sink_index);
            i.append(self.channels);
            i.append(&self.volume);
            i.append(self.muted);
            i.append(self.corked);
        });
    }
}

impl<'a> Get<'a> for InputStream {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, application_name, sink_index, channels, volume, muted, corked) =
            <(u32, String, String, u32, u16, Vec<u32>, bool, bool)>::get(i)?;
        Some(Self {
            index,
            name,
            application_name,
            sink_index,
            channels,
            volume,
            muted,
            corked,
        })
    }
}

impl Arg for InputStream {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussuqaubb)\0") }
    }
}

impl From<&SinkInputInfo<'_>> for InputStream {
    fn from(value: &SinkInputInfo<'_>) -> Self {
        let name = if let Some(name_opt) = &value.name {
            String::from(name_opt.clone())
        } else {
            String::from("")
        };
        let application_name = value
            .proplist
            .get_str("application.name")
            .unwrap_or_default();
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        Self {
            index: value.index,
            name,
            application_name,
            sink_index: value.sink,
            channels,
            volume,
            muted: value.mute,
            corked: value.corked,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OutputStream {
    pub index: u32,
    pub name: String,
    pub application_name: String,
    pub source_index: u32,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
    pub corked: bool,
}

impl Append for OutputStream {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(self.index);
            i.append(&self.name);
            i.append(&self.application_name);
            i.append(self.source_index);
            i.append(self.channels);
            i.append(&self.volume);
            i.append(self.muted);
            i.append(self.corked);
        });
    }
}

impl<'a> Get<'a> for OutputStream {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, application_name, source_index, channels, volume, muted, corked) =
            <(u32, String, String, u32, u16, Vec<u32>, bool, bool)>::get(i)?;
        Some(Self {
            index,
            name,
            application_name,
            source_index,
            channels,
            volume,
            muted,
            corked,
        })
    }
}

impl Arg for OutputStream {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussuqaubb)\0") }
    }
}

impl From<&SourceOutputInfo<'_>> for OutputStream {
    fn from(value: &SourceOutputInfo<'_>) -> Self {
        let name = if let Some(name_opt) = &value.name {
            String::from(name_opt.clone())
        } else {
            String::from("")
        };
        let application_name = value
            .proplist
            .get_str("application.name")
            .unwrap_or_default();
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        Self {
            index: value.index,
            name,
            application_name,
            source_index: value.source,
            channels,
            volume,
            muted: value.mute,
            corked: value.corked,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Card {
    pub index: u32,
    pub name: String,
    pub profiles: Vec<CardProfile>,
    pub active_profile: String,
}

impl Append for Card {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(self.index);
            i.append(&self.name);
            i.append(&self.profiles);
            i.append(&self.active_profile);
        });
    }
}

impl<'a> Get<'a> for Card {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, profiles, active_profile) =
            <(u32, String, Vec<CardProfile>, String)>::get(i)?;
        Some(Self {
            index,
            name,
            profiles,
            active_profile,
        })
    }
}

impl Arg for Card {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(usa(ssb)s)\0") }
    }
}

impl From<&CardInfo<'_>> for Card {
    fn from(value: &CardInfo<'_>) -> Self {
        let name = if let Some(name_opt) = &value.proplist.get_str("alsa.card_name") {
            name_opt.clone()
        } else {
            String::from("Unnamed")
        };
        let index = value.index;
        let mut profiles = Vec::new();
        for profile in value.profiles.iter() {
            profiles.push(CardProfile::from(profile));
        }
        let active_profile = if let Some(profile_opt) = value.active_profile.as_ref() {
            profile_opt.name.clone().unwrap().to_string()
        } else {
            String::from("Off")
        };
        Self {
            index,
            name,
            profiles,
            active_profile,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CardProfile {
    pub name: String,
    pub description: String,
    pub available: bool,
}

impl Append for CardProfile {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(&self.name);
            i.append(&self.description);
            i.append(self.available);
        });
    }
}

impl<'a> Get<'a> for CardProfile {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (name, description, available) = <(String, String, bool)>::get(i)?;
        Some(Self {
            name,
            description,
            available,
        })
    }
}

impl Arg for CardProfile {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ssb)\0") }
    }
}

impl From<&CardProfileInfo<'_>> for CardProfile {
    fn from(value: &CardProfileInfo<'_>) -> Self {
        let name = if let Some(name_opt) = &value.name {
            String::from(name_opt.clone())
        } else {
            String::from("")
        };
        let description = if let Some(description_opt) = &value.description {
            String::from(description_opt.clone())
        } else {
            String::from("")
        };
        Self {
            name,
            description,
            available: value.available,
        }
    }
}

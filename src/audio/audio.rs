use dbus::{
    arg::{self, Append, Arg, ArgType, Get},
    Signature,
};
use pulse::context::introspect::{
    CardInfo, CardProfileInfo, SinkInfo, SinkInputInfo, SourceInfo, SourceOutputInfo,
};

#[derive(Debug)]
pub struct PulseError(pub &'static str);

#[derive(Debug, Clone, Default)]
pub struct Source {
    pub index: u32,
    pub name: String,
    pub alias: String,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
}

unsafe impl Send for Source {}
unsafe impl Sync for Source {}

impl Append for Source {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(&self.index);
            i.append(&self.name);
            i.append(&self.alias);
            i.append(&self.channels);
            i.append(&self.volume);
            i.append(&self.muted);
        });
    }
}

impl<'a> Get<'a> for Source {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, alias, channels, volume, muted) =
            <(u32, String, String, u16, Vec<u32>, bool)>::get(i)?;
        Some(Self {
            index,
            name,
            alias,
            channels,
            volume,
            muted,
        })
    }
}

impl Arg for Source {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussqaub)\0") }
    }
}

impl From<&SourceInfo<'_>> for Source {
    fn from(value: &SourceInfo<'_>) -> Self {
        let name_opt = &value.name;
        let alias_opt = &value.description;
        let name: String;
        let alias: String;
        if name_opt.is_none() {
            name = String::from("");
        } else {
            name = String::from(name_opt.clone().unwrap());
        }
        if alias_opt.is_none() {
            alias = String::from("");
        } else {
            alias = String::from(alias_opt.clone().unwrap());
        }
        let index = value.index;
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        let muted = value.mute;
        Self {
            index,
            name,
            alias,
            channels,
            volume,
            muted,
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
}

unsafe impl Send for Sink {}
unsafe impl Sync for Sink {}

impl Append for Sink {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(&self.index);
            i.append(&self.name);
            i.append(&self.alias);
            i.append(&self.channels);
            i.append(&self.volume);
            i.append(&self.muted);
        });
    }
}

impl<'a> Get<'a> for Sink {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, alias, channels, volume, muted) =
            <(u32, String, String, u16, Vec<u32>, bool)>::get(i)?;
        Some(Self {
            index,
            name,
            alias,
            channels,
            volume,
            muted,
        })
    }
}

impl Arg for Sink {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussqaub)\0") }
    }
}

impl From<&SinkInfo<'_>> for Sink {
    fn from(value: &SinkInfo<'_>) -> Self {
        let name_opt = &value.name;
        let alias_opt = &value.description;
        let name: String;
        let alias: String;
        if name_opt.is_none() {
            name = String::from("");
        } else {
            name = String::from(name_opt.clone().unwrap());
        }
        if alias_opt.is_none() {
            alias = String::from("");
        } else {
            alias = String::from(alias_opt.clone().unwrap());
        }
        let index = value.index;
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        let muted = value.mute;
        Self {
            index,
            name,
            alias,
            channels,
            volume,
            muted,
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
            i.append(&self.index);
            i.append(&self.name);
            i.append(&self.application_name);
            i.append(&self.sink_index);
            i.append(&self.channels);
            i.append(&self.volume);
            i.append(&self.muted);
            i.append(&self.corked);
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
        let name_opt = &value.name;
        let name: String;
        if name_opt.is_none() {
            name = String::from("");
        } else {
            name = String::from(name_opt.clone().unwrap());
        }
        let application_name = value
            .proplist
            .get_str("application.name")
            .unwrap_or_default();
        let sink_index = value.sink;
        let index = value.index;
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        let muted = value.mute;
        let corked = value.corked;
        Self {
            index,
            name,
            application_name,
            sink_index,
            channels,
            volume,
            muted,
            corked,
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
            i.append(&self.index);
            i.append(&self.name);
            i.append(&self.application_name);
            i.append(&self.source_index);
            i.append(&self.channels);
            i.append(&self.volume);
            i.append(&self.muted);
            i.append(&self.corked);
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
        let name_opt = &value.name;
        let name: String;
        if name_opt.is_none() {
            name = String::from("");
        } else {
            name = String::from(name_opt.clone().unwrap());
        }
        let application_name = value
            .proplist
            .get_str("application.name")
            .unwrap_or_default();
        let source_index = value.source;
        let index = value.index;
        let channels = value.channel_map.len() as u16;
        let mut volume = vec![0; channels as usize];
        for i in 0..channels as usize {
            unsafe { *volume.get_unchecked_mut(i) = value.volume.get()[i].0 };
        }
        let muted = value.mute;
        let corked = value.corked;
        Self {
            index,
            name,
            application_name,
            source_index,
            channels,
            volume,
            muted,
            corked,
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
            i.append(&self.index);
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

impl From<CardInfo<'_>> for Card {
    fn from(value: CardInfo<'_>) -> Self {
        let name_opt = &value.name;
        let name: String;
        if name_opt.is_none() {
            name = String::from("");
        } else {
            name = String::from(name_opt.clone().unwrap());
        }
        let index = value.index;
        let mut profiles = Vec::new();
        for profile in value.profiles.iter() {
            profiles.push(CardProfile::from(profile));
        }
        let active_profile: String;
        if value.active_profile.is_some() {
            active_profile = value
                .active_profile
                .unwrap()
                .name
                .unwrap_or_default()
                .to_string();
        } else {
            active_profile = "".into();
        }
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
            i.append(&self.available);
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
        let name_opt = &value.name;
        let name: String;
        if name_opt.is_none() {
            name = String::from("");
        } else {
            name = String::from(name_opt.clone().unwrap());
        }
        let description_opt = &value.description;
        let description: String;
        if description_opt.is_none() {
            description = String::from("");
        } else {
            description = String::from(description_opt.clone().unwrap());
        }
        let available = value.available;
        Self {
            name,
            description,
            available,
        }
    }
}

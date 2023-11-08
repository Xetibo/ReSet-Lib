use dbus::{
    arg::{self, Append, Arg, ArgType, Get},
    Signature,
};
use pulse::context::introspect::{SinkInfo, SinkInputInfo, SourceInfo, SourceOutputInfo};

#[derive(Debug)]
pub struct PulseError(pub &'static str);

pub struct Source {
    pub index: u32,
    pub name: String,
    pub alias: String,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
}

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

#[derive(Debug)]
pub struct Sink {
    pub index: u32,
    pub name: String,
    pub alias: String,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
}

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

pub struct InputStream {
    pub index: u32,
    pub name: String,
    pub application_name: String,
    pub sink_index: u32,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
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
        });
    }
}

impl<'a> Get<'a> for InputStream {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, application_name, sink_index, channels, volume, muted) =
            <(u32, String, String, u32, u16, Vec<u32>, bool)>::get(i)?;
        Some(Self {
            index,
            name,
            application_name,
            sink_index,
            channels,
            volume,
            muted,
        })
    }
}

impl Arg for InputStream {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussuqaub)\0") }
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
        Self {
            index,
            name,
            application_name,
            sink_index,
            channels,
            volume,
            muted,
        }
    }
}

pub struct OutputStream {
    pub index: u32,
    pub name: String,
    pub application_name: String,
    pub source_index: u32,
    pub channels: u16,
    pub volume: Vec<u32>,
    pub muted: bool,
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
        });
    }
}

impl<'a> Get<'a> for OutputStream {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (index, name, application_name, source_index, channels, volume, muted) =
            <(u32, String, String, u32, u16, Vec<u32>, bool)>::get(i)?;
        Some(Self {
            index,
            name,
            application_name,
            source_index,
            channels,
            volume,
            muted,
        })
    }
}

impl Arg for OutputStream {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(ussuqaub)\0") }
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
        let sink_index = value.source;
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
            application_name,
            source_index: sink_index,
            channels,
            volume,
            muted,
        }
    }
}

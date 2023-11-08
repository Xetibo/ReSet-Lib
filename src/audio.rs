use dbus::{
    arg::{self, Append, Arg, ArgType, Get},
    Signature,
};

#[derive(Debug)]
pub struct PulseError(&'static str);

pub struct Source {
    index: u32,
    name: String,
    alias: String,
    channels: u16,
    volume: Vec<u32>,
    muted: bool,
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

#[derive(Debug)]
pub struct Sink {
    index: u32,
    name: String,
    alias: String,
    channels: u16,
    volume: Vec<u32>,
    muted: bool,
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

pub struct InputStream {
    index: u32,
    name: String,
    application_name: String,
    sink_index: u32,
    channels: u16,
    volume: Vec<u32>,
    muted: bool,
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

pub struct OutputStream {
    index: u32,
    name: String,
    application_name: String,
    source_index: u32,
    channels: u16,
    volume: Vec<u32>,
    muted: bool,
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

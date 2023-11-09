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

pub struct Source {
    id: u32,
    name: String,
    volume: Volume,
    status: Status,
    channel: Channel,
}

pub struct Sink {
    id: u32,
    name: String,
    volume: Volume,
    status: Status,
    channel: Channel,
}

pub struct Audio {
    default_sink: Sink,
    default_source: Source,
    sinks: Vec<Sink>,
    sources: Vec<Source>,
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

use dbus::{
    arg::{self, Append, Arg, ArgType, Get},
    Path, Signature,
};

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    path: Path<'static>,
    rssi: i16,
    name: String,
    adapter: Path<'static>,
    trusted: bool,
    bonded: bool,
    paired: bool,
    blocked: bool,
    address: String,
}

unsafe impl Send for BluetoothDevice {}
unsafe impl Sync for BluetoothDevice {}

impl<'a> Get<'a> for BluetoothDevice {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let path = <Path<'static>>::get(i)?;
        let rssi = <i16>::get(i)?;
        let name = <String>::get(i)?;
        let adapter = <Path<'static>>::get(i)?;
        let trusted = <bool>::get(i)?;
        let bonded = <bool>::get(i)?;
        let paired = <bool>::get(i)?;
        let blocked = <bool>::get(i)?;
        let address = <String>::get(i)?;
        Some(BluetoothDevice {
            path,
            rssi,
            name,
            adapter,
            trusted,
            bonded,
            paired,
            blocked,
            address,
        })
    }
}

impl Append for BluetoothDevice {
    fn append_by_ref(&self, iter: &mut arg::IterAppend) {
        iter.append_struct(|i| {
            i.append(&self.path);
            i.append(&self.rssi);
            i.append(&self.name);
            i.append(&self.adapter);
            i.append(&self.trusted);
            i.append(&self.bonded);
            i.append(&self.paired);
            i.append(&self.blocked);
            i.append(&self.address);
        });
    }
}

impl Arg for BluetoothDevice {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(nsobbbbs)\0") }
    }
}

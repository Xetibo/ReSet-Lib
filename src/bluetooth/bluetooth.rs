use std::any;

use dbus::{
    arg::{self, Append, Arg, ArgType, Get, IterAppend, RefArg},
    Path, Signature,
};

#[derive(Debug, Clone, Default)]
pub struct BluetoothDevice {
    pub path: Path<'static>,
    pub rssi: i16,
    pub alias: String,
    pub name: String,
    pub adapter: Path<'static>,
    pub trusted: bool,
    pub bonded: bool,
    pub paired: bool,
    pub blocked: bool,
    pub connected: bool,
    pub icon: String,
    pub address: String,
}

unsafe impl Send for BluetoothDevice {}
unsafe impl Sync for BluetoothDevice {}

impl<'a> Get<'a> for BluetoothDevice {
    fn get(i: &mut arg::Iter<'a>) -> Option<Self> {
        let (
            path,
            rssi,
            name,
            alias,
            adapter,
            trusted,
            bonded,
            paired,
            blocked,
            connected,
            icon,
            address,
        ) = <(
            Path<'static>,
            i16,
            String,
            String,
            Path<'static>,
            bool,
            bool,
            bool,
            bool,
            bool,
            String,
            String,
        )>::get(i)?;
        Some(BluetoothDevice {
            path,
            rssi,
            name,
            alias,
            adapter,
            trusted,
            bonded,
            paired,
            blocked,
            connected,
            icon,
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
            i.append(&self.alias);
            i.append(&self.adapter);
            i.append(&self.trusted);
            i.append(&self.bonded);
            i.append(&self.paired);
            i.append(&self.blocked);
            i.append(&self.connected);
            i.append(&self.icon);
            i.append(&self.address);
        });
    }
}

impl Arg for BluetoothDevice {
    const ARG_TYPE: arg::ArgType = ArgType::Struct;
    fn signature() -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(nssobbbbbss)\0") }
    }
}

impl RefArg for BluetoothDevice {
    fn arg_type(&self) -> ArgType {
        ArgType::Struct
    }
    fn signature(&self) -> Signature<'static> {
        unsafe { Signature::from_slice_unchecked("(nssobbbbbss)\0") }
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

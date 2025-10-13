use super::any::ReSetAny;

pub enum EntryButtonLevel {
    TopLevel,
    SubLevel,
}

pub struct EntryButton {
    pub title: &'static str,
    pub icon: Option<String>,
    pub msg: Box<&'static dyn ReSetAny>,
    pub level: EntryButtonLevel,
}

impl EntryButton {
    pub fn top_level(
        title: &'static str,
        icon: Option<impl Into<String>>,
        msg: impl Into<Box<&'static dyn ReSetAny>>,
    ) -> Self {
        Self {
            title,
            icon: icon.map(|icon| icon.into()),
            msg: msg.into(),
            level: EntryButtonLevel::TopLevel,
        }
    }

    pub fn sub_level(
        title: &'static str,
        icon: Option<impl Into<String>>,
        msg: impl Into<Box<&'static dyn ReSetAny>>,
    ) -> Self {
        Self {
            title,
            icon: icon.map(|icon| icon.into()),
            msg: msg.into(),
            level: EntryButtonLevel::SubLevel,
        }
    }
}

pub struct EntryCategory {
    pub main_entry: EntryButton,
    pub sub_entries: Vec<EntryButton>,
}

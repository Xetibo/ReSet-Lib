use gtk::{Align, Frame, Label, Orientation};

pub fn create_title(name: &'static str) -> Label {
    Label::builder()
        .label(name)
        .css_classes(vec!["resetSettingLabel"])
        .halign(Align::Start)
        .margin_start(5)
        .margin_bottom(10)
        .build()
}

pub fn create_frame(orientation: Orientation) -> gtk::Frame {
    Frame::builder()
        .css_classes(vec!["resetSettingFrame"])
        .build()
}

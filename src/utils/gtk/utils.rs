use gtk::{Align, Label};

pub fn create_title(name: &'static str) -> Label {
    Label::builder()
        .label(name)
        .css_classes(vec!["resetSettingLabel"])
        .halign(Align::Start)
        .margin_start(5)
        .margin_bottom(10)
        .build()
}

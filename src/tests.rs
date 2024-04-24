#[cfg(test)]
use serial_test::serial;

#[cfg(test)]
use crate::utils::plugin::plugin_tests;
#[cfg(test)]
use crate::{utils::plugin::PluginTestError, utils::plugin::PluginTestFunc};

#[test]
fn test_config_dir() {
    use crate::create_config;
    let organization_name = "xetibo";
    let project_name = "globiTM";
    assert!(create_config(organization_name, project_name).is_some());
}

#[test]
#[serial]
fn test_custom_config() {
    use crate::parse_flags;
    use crate::utils::flags::{Flag, Flags};
    use std::fs;

    fs::File::create_new("test.txt").expect("Could not create test file");
    let command_flags = vec![
        String::from("binary name"),
        String::from("--config"),
        String::from("test.txt"),
    ];
    let flags = parse_flags(command_flags);
    let string = "test.txt".to_string();
    let copy = Flags(vec![Flag::ConfigDir(string)]);
    assert!(!flags.0.is_empty());
    assert_eq!(flags.0.len(), copy.0.len());
    let impossible = String::from("impossible");
    let match_orig = match flags.0.first().unwrap() {
        Flag::ConfigDir(content) => content,
        _ => &impossible,
    };
    let match_copy = match copy.0.first().unwrap() {
        Flag::ConfigDir(content) => content,
        _ => &impossible,
    };
    assert_eq!(match_orig, match_copy);
    fs::remove_file("test.txt").expect("Could not delete test file");
}

#[test]
#[serial]
fn test_custom_config_non_existing() {
    use crate::parse_flags;

    let command_flags = vec![
        String::from("binary name"),
        String::from("--config"),
        String::from("test.txt"),
    ];
    let flags = parse_flags(command_flags);
    assert!(flags.0.is_empty());
}

#[test]
fn test_custom_flag() {
    use crate::parse_flags;
    use crate::utils::flags::Flag;
    use crate::utils::variant::Variant;
    use crate::utils::variant::{Empty, TVariant};

    let command_flags = vec![
        String::from("binary name"),
        String::from("--something"),
        String::from("test.txt"),
        String::from("test2.txt"),
        String::from("--other"),
        String::from("othervalue"),
    ];
    let flags = parse_flags(command_flags);
    let matched_name: &String;
    let matched_value: &Variant;
    let failed_string = String::from("failed");
    let failed_variant = Empty {}.into_variant();
    match flags.0.first().unwrap() {
        Flag::Other((name, value)) => {
            matched_name = name;
            matched_value = value;
        }
        _ => {
            matched_name = &failed_string;
            matched_value = &failed_variant;
        }
    }
    assert_eq!(flags.0.len(), 2);
    assert_eq!(matched_name, &String::from("--something"));
    assert_eq!(
        matched_value.to_value_cloned::<Vec<String>>().unwrap(),
        vec![String::from("test.txt"), String::from("test2.txt")]
            .into_variant()
            .to_value_cloned::<Vec<String>>()
            .unwrap()
    );
}

#[test]
fn test_custom_error_flag() {
    use crate::parse_flags;

    let command_flags = vec![String::from("binary name"), String::from("notaflag")];
    let flags = parse_flags(command_flags);
    assert!(flags.0.is_empty());
}

#[test]
fn test_custom_tests() {
    let func1 = || -> Result<(), PluginTestError> { Ok(()) };
    let func2 = || -> Result<(), PluginTestError> { Err(PluginTestError::new("fail")) };
    let func1 = PluginTestFunc::new(func1, "henlo");
    let func2 = PluginTestFunc::new(func2, "fail");
    let funcs = vec![func1, func2];
    plugin_tests(String::from("libtest"), funcs);
}

#[test]
fn test_plug_assert_macros() {
    use crate::{plug_assert, plug_assert_eq};
    let test = || plug_assert!(true);
    assert!(test().is_ok());
    let test = || plug_assert!(false);
    assert!(test().is_err());
    let test = || plug_assert_eq!(1, 1);
    assert!(test().is_ok());
    let test = || plug_assert_eq!(1, 10);
    assert!(test().is_err());
}

#[test]
fn test_config_dir() {
    use crate::create_config;
    let organization_name = "xetibo";
    let project_name = "globiTM";
    assert!(create_config(organization_name, project_name).is_some());
}

#[test]
fn test_parameters() {
    use crate::parse_flags;
    use crate::utils::flags::{Flag, Flags};
    use std::fs;

    fs::File::create_new("test.txt").expect("Could not create test file");
    let command_flags = vec![String::from("--config"), String::from("test.txt")];
    let flags = parse_flags(&command_flags);
    assert!(!flags.0.is_empty());
    assert_eq!(
        flags,
        Flags(vec![Flag::ConfigDir(&"test.txt".to_string())])
    );
    fs::remove_file("test.txt").expect("Could not delete test file");
}

#[test]
fn test_config_dir() {
    use crate::create_config;
    let organization_name = "xetibo";
    let project_name = "globiTM";
    assert!(create_config(organization_name, project_name).is_some());
}

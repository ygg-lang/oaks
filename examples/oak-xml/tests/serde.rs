use oak_xml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    #[serde(rename = "↯id")]
    id: u32,
    name: String,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Project {
    name: String,
    #[serde(rename = "user")]
    users: Vec<User>,
}

#[test]
fn test_serde_basic() {
    let project = Project { name: "Oak".to_string(), users: vec![User { id: 1, name: "Alice".to_string(), active: true }, User { id: 2, name: "Bob".to_string(), active: false }] };

    let xml = oak_xml::to_string(&project).unwrap();
    println!("Serialized XML: {}", xml);

    let decoded: Project = oak_xml::from_str(&xml).unwrap();
    assert_eq!(project, decoded);
}

#[test]
fn test_from_str() {
    let xml = r#"
<Project>
    <name>Oak</name>
    <user id="1">
        <name>Alice</name>
        <active>true</active>
    </user>
</Project>
"#;

    let project: Project = oak_xml::from_str(xml).unwrap();
    assert_eq!(project.name, "Oak");
    assert_eq!(project.users.len(), 1);
    assert_eq!(project.users[0].id, 1);
    assert_eq!(project.users[0].name, "Alice");
    assert_eq!(project.users[0].active, true);
}

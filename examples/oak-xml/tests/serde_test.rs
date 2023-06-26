#![cfg(feature = "serde")]
use oak_xml::{from_str, to_string};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct Project {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "dependencies")]
    dependencies: Vec<Dependency>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct Dependency {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "version")]
    version: String,
}

#[test]
fn test_serde_xml_nested() {
    let project = Project { name: "oak-xml".to_string(), version: "0.1.0".to_string(), dependencies: vec![Dependency { name: "oak-core".to_string(), version: "0.1.0".to_string() }, Dependency { name: "serde".to_string(), version: "1.0".to_string() }] };

    let xml = to_string(&project).unwrap();
    println!("Serialized XML:\n{}", xml);

    let deserialized: Project = from_str(&xml).unwrap();
    assert_eq!(project, deserialized);
}

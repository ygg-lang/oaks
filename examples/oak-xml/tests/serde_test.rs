use oak_xml::{from_str, to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Project {
    #[serde(rename = "@name")]
    name: String,
    version: String,
    dependencies: Dependencies,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Dependencies {
    #[serde(rename = "Dependency")]
    items: Vec<Dependency>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Dependency {
    name: String,
    version: String,
}

#[test]
fn test_serde_basic() {
    let project = Project {
        name: "oak".to_string(),
        version: "0.1.0".to_string(),
        dependencies: Dependencies { items: vec![Dependency { name: "serde".to_string(), version: "1.0".to_string() }, Dependency { name: "oak-core".to_string(), version: "0.1.0".to_string() }] },
    };

    let xml = to_string(&project).unwrap();
    println!("Serialized XML:\n{}", xml);

    // Expected XML structure (order might vary slightly depending on map implementation)
    // <Project name="oak">
    //   <version>0.1.0</version>
    //   <Dependency><name>serde</name><version>1.0</version></Dependency>
    //   <Dependency><name>oak-core</name><version>0.1.0</version></Dependency>
    // </Project>

    // Note: Our Serializer for seq of objects (Dependencies.items) wrapped in Dependencies
    // will produce <Dependencies><Dependency>...</Dependency><Dependency>...</Dependency></Dependencies>
    // because $value in Dependencies maps items directly to children of Dependencies.

    let deserialized: Project = from_str(&xml).unwrap();
    assert_eq!(project, deserialized);
}

#[test]
fn test_from_xml_string() {
    let xml = r#"<Project name="oak">
        <version>0.1.0</version>
        <Dependency>
            <name>serde</name>
            <version>1.0</version>
        </Dependency>
    </Project>"#;

    let project: Project = from_str(xml).unwrap();
    assert_eq!(project.name, "oak");
    assert_eq!(project.version, "0.1.0");
    assert_eq!(project.dependencies.items.len(), 1);
    assert_eq!(project.dependencies.items[0].name, "serde");
}

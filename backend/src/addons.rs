use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Version = [u32; 3];

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    format_version: u32,
    header: ManifestHeader,
    modules: Vec<ManifestModule>,
    dependencies: Vec<ManifestDependency>,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestHeader {
    name: String,
    description: String,
    uuid: Uuid,
    version: Version,
    min_engine_version: Version,
}
#[derive(Serialize, Deserialize)]
pub enum ModuleType {
    data,
    resource,
    script,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestModule {
    #[serde(rename = "type")]
    kind: ModuleType,
    uuid: Uuid,
    version: Version,
    language: Option<String>,
    entry: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyVersion {
    String(String),
    Vec(Vec<i32>),
}

#[derive(Serialize, Deserialize)]
pub struct ManifestDependency {
    module_name: Option<String>,
    version: DependencyVersion,
    uuid: Option<String>,
}
impl Manifest {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
    pub fn new(a: &mut Manifest, b: &mut Manifest) -> Self {
        let modules = {
            let mut modules = vec![];
            a.modules.append(&mut b.modules);
            modules.append(&mut a.modules);
            modules
        };
        let dependencies = {
            let mut dependencies = vec![];
            a.dependencies.append(&mut b.dependencies);
            dependencies.append(&mut a.dependencies);
            dependencies
        };
        Self {
            format_version: 2,
            header: ManifestHeader {
                name: "Addon made with rancor's addon merger".to_string(),
                description: "Define your description here".to_string(),
                uuid: Uuid::new_v4(),
                version: [1, 0, 0],
                min_engine_version: [1, 20, 80],
            },
            modules,
            dependencies,
        }
    }
    pub fn create(
        header: ManifestHeader,
        modules: Vec<ManifestModule>,
        dependencies: Option<Vec<ManifestDependency>>,
    ) -> Self {
        Self {
            format_version: 2,
            header,
            modules,
            dependencies: dependencies.unwrap_or(vec![]),
        }
    }
    pub fn from_string(str: String) -> Result<Manifest, serde_json::Error> {
        serde_json::from_str(&str)
    }
}
/*{
    "format_version": 2,
    "header": {
        "name": "BEHAVIOR MERGE MADE BY RANCOR-TOOLS WEBSITE(CODE CREATED BY CYCRO)",
        "description": "PLEASE PUT THE NAME OF THE MERGED ADDON AND DESCRIPTION",
        "uuid": "FIRST UUID",
        "version": [ 1, 0, 0 ],
        "min_engine_version": [ 1, 20, 80 ]
    },
    "modules": [{
        "type": "data | resources; set as 'data' or 'resources' depending on the type of the pack",
        "uuid": "UUID 2",
        "version": [ 1, 0, 0 ]
    },{
        "type": "script",
        "language": "javascript",
        "uuid": "UUID 3",
        "entry": "scripts/main.js",
        "version": [ 1, 0, 0 ],
        "remove this": "if the pack is resourcepack or behavior pack that does not use scripts, remove this, object"
    }],
    "dependencies": [{
        "module_name": "@minecraft/server",
        "version": "1.12.0-beta"
    },{
        "module_name": "@minecraft/server-ui",
        "version": "1.2.0-beta"
    }]
}*/

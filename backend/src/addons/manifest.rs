use serde::{Deserialize, Serialize};
use uuid::Uuid;

type McVersion = [u8; 3];

#[derive(Serialize, Deserialize)]
pub struct ManifestHeader {
    name: String,
    description: String,
    version: McVersion,
    uuid: Uuid,
    min_engine_version: McVersion,
}

impl ManifestHeader {
    pub fn new(name: &str, description: &str, version: &[u8; 3]) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: *version,
            uuid: Uuid::new_v4(),
            min_engine_version: [1, 21, 0],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ManifestModuleType {
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "resources")]
    Resource,
    #[serde(rename = "script")]
    Script,
}

#[derive(Serialize, Deserialize)]
pub struct ManifestModule {
    #[serde(rename = "type")]
    kind: ManifestModuleType,
    uuid: Uuid,
    version: McVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<String>,
}

impl ManifestModuleType {
    pub fn to_string(&self) -> String {
        (match self {
            ManifestModuleType::Data => "data",
            ManifestModuleType::Resource => "resource",
            ManifestModuleType::Script => "script",
        })
        .to_string()
    }
}
impl ManifestModule {
    pub fn new(kind: ManifestModuleType) -> Self {
        Self {
            kind,
            uuid: Uuid::new_v4(),
            version: [1, 0, 0],
            language: None,
            entry: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyVersion {
    String(String),
    Vec(McVersion),
}

#[derive(Serialize, Deserialize)]
pub struct ManifestDependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<Uuid>,
    version: DependencyVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    module_name: Option<String>,
}

impl ManifestDependency {
    pub fn new(module: String, version: DependencyVersion) -> Self {
        Self {
            version,
            module_name: Some(module),
            uuid: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    format_version: u8,
    header: ManifestHeader,
    modules: Vec<ManifestModule>,
    dependencies: Vec<ManifestDependency>,
}

impl Manifest {
    pub fn from_text(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
    pub fn create_from(a: &mut Manifest, b: &mut Manifest) -> Self {
        let merge_txt = format!("Merge of {} and {}", a.header.name, b.header.name);
        let mut modules: Vec<ManifestModule> = Vec::new();
        modules.append(&mut a.modules);
        modules.append(&mut b.modules);
        let mut dependencies: Vec<ManifestDependency> = Vec::new();
        dependencies.append(&mut a.dependencies);
        dependencies.append(&mut b.dependencies);
        Self {
            format_version: 2,
            header: ManifestHeader::new(&*merge_txt, &*merge_txt, &[1, 0, 0]),
            modules,
            dependencies,
        }
    }
    pub fn default() -> Self {
        Self {
            format_version: 2,
            header: ManifestHeader::new(
                "Default made manifest name",
                "Default made manifest description",
                &[1, 0, 0],
            ),
            modules: Vec::new(),
            dependencies: Vec::new(),
        }
    }
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    pub fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        let str = &*match self.to_string() {
            Ok(str) => str,
            Err(e) => return Err(e),
        };
        Ok(str.as_bytes().to_vec())
    }
}

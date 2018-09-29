#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesystemChange {
    pub path: String,
    pub kind: u8
}

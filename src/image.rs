use serde::de::{Deserializer, DeserializeOwned};
use serde::Deserialize;

fn null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
    where D: Deserializer<'de>,
          T: DeserializeOwned + Default
{
    let actual : Option<T> = Option::deserialize(de)?;
    Ok(actual.unwrap_or_default())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub created: u64,
    pub id: String,
    pub parent_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub repo_tags: Vec<String>,
    pub size: u64,
    pub virtual_size: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageStatus {
    pub status: Option<String>,
    pub error: Option<String>
}

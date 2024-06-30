use std::io::Write;
use super::errors::ModifyOpError;

pub fn construct_from_file<T>(path: &str) -> Result<T, ModifyOpError> where T: serde::de::DeserializeOwned{
    let file_contents = std::fs::read_to_string(path)
        .map_err(ModifyOpError::CouldNotFindFile)?;
    let deserialized = toml::from_str(&file_contents)
        .map_err(ModifyOpError::Deserialization)?;
    Ok(deserialized)
}

pub fn flush_to_file<T>(to_serialize: T, path: &str) -> Result<(), ModifyOpError> where T: serde::Serialize{
    let serialized = toml::to_string_pretty(&to_serialize).map_err(ModifyOpError::Serialization)?;
    let mut file = std::fs::File::create(path).map_err(ModifyOpError::CouldNotCreateFile)?;
    file.write_all(serialized.as_bytes()).map_err(ModifyOpError::CouldNotWriteFile)?;
    Ok(())
}
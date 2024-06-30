#[derive(Debug)]
pub enum ModifyOpError {
    CouldNotFindFile(std::io::Error),
    CouldNotCreateFile(std::io::Error),
    CouldNotWriteFile(std::io::Error),
    Deserialization(toml::de::Error),
    Serialization(toml::ser::Error),
}
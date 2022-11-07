use thiserror::Error;
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("AnkiServer error: {0}")]
    AnkiServer(#[from] ankisyncd::ApplicationError),
    #[error("Error while serializing data: {0}")]
    SerdeTomlSerializingError(#[from] toml::ser::Error),
    #[error("Error while deserializing data: {0}")]
    SerdeTomlDeserializingError(#[from] toml::de::Error),
    #[error("Error while updating pc Anki address: {0}")]
    UpdateAddr(#[from] updateaddr::UpdateAddrError),
    #[error("Error while Send Windows shortcut: {0}")]
    Send(#[from] send::SendError),
}

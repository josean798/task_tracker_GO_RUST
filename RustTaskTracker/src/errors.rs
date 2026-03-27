use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("La descripción no puede estar vacía")]
    EmptyDescription,

    #[error("No se encontró la tarea con id {0}")]
    TaskNotFound(u32),

    #[error("Error de Entrada o Salida: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Argumento faltante: {0}")]
    MissingArgument(String),

    #[error("Argumento inválido: {0}")]
    InvalidArgument(String),
}

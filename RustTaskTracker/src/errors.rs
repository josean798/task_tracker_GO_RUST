use thiserror::Error;

/// Errores de la aplicación.
///
/// Cada variante representa una condición de fallo distinta.
/// Los errores de I/O y JSON se convierten automáticamente desde
/// sus tipos estándar gracias al atributo `#[from]`.
#[derive(Error, Debug)]
pub enum AppError {
    /// La descripción de una tarea está vacía.
    #[error("La descripción no puede estar vacía")]
    EmptyDescription,

    /// No existe ninguna tarea con el id indicado.
    #[error("No se encontró la tarea con id {0}")]
    TaskNotFound(u32),

    /// Error al leer o escribir en el sistema de archivos.
    #[error("Error de Entrada o Salida: {0}")]
    IoError(#[from] std::io::Error),

    /// Error al serializar o deserializar JSON.
    #[error("Error JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Falta un argumento requerido por un comando.
    #[error("Argumento faltante: {0}")]
    MissingArgument(String),

    /// Un argumento no tiene el formato o valor esperado.
    #[error("Argumento inválido: {0}")]
    InvalidArgument(String),

    /// No existe ningún usuario con el id o nombre buscado.
    #[error("Usuario no encontrado")]
    UserNotFound,

    /// El usuario o la contraseña son incorrectos.
    #[error("Credenciales inválidas")]
    InvalidCredentials,

    /// Se intentó una operación que requiere sesión activa sin tenerla.
    #[error("No hay una sesión activa\nInicie sesión o registrese para continuar")]
    NoActiveSession,

    /// Ya existe un usuario registrado con ese nombre.
    #[error("El nombre de usuario ya existe")]
    UserAlreadyExists,

    /// El token de sesión almacenado no coincide con el usuario.
    #[error("Sesión inválida")]
    InvalidSession,
}
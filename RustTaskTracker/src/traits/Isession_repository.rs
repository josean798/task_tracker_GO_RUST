use crate::entities::session::Session;
use crate::errors::AppError;

/// Contrato de persistencia para la sesión activa.
///
/// Solo existe una sesión a la vez; las operaciones operan
/// sobre un único archivo (o registro) sin necesidad de un id.
pub trait ISessionRepository {
    /// Persiste una nueva sesión para el usuario indicado.
    ///
    /// # Arguments
    /// * `user_id` - Id del usuario que inicia sesión.
    /// * `token` - Token de verificación generado por el servicio.
    fn create_session(&self, user_id: u32, token: String) -> Result<(), AppError>;

    /// Devuelve la sesión actualmente almacenada.
    ///
    /// # Errors
    /// Retorna `AppError::NoActiveSession` si no hay ninguna sesión guardada.
    fn get_session(&self) -> Result<Session, AppError>;

    /// Elimina la sesión activa (cierre de sesión).
    fn delete_session(&self) -> Result<(), AppError>;
}
use crate::errors::AppError;
use crate::entities::user::User;

/// Contrato de persistencia para usuarios.
///
/// Cualquier implementación (JSON, SQL, en memoria, etc.) debe
/// satisfacer estos métodos para ser utilizable por `UserService`.
pub trait IUserRepository {
    /// Crea y persiste un nuevo usuario con el hash de contraseña dado.
    ///
    /// # Errors
    /// Retorna `AppError::UserAlreadyExists` si el nombre de usuario ya está registrado.
    fn create_user(&mut self, username: String, password_hash: String) -> Result<User, AppError>;

    /// Busca y devuelve el usuario con el `id` indicado.
    ///
    /// # Errors
    /// Retorna `AppError::UserNotFound` si no existe ningún usuario con ese id.
    fn get_user(&mut self, id: u32) -> Result<User, AppError>;

    /// Busca y devuelve el usuario con el nombre de usuario indicado.
    ///
    /// # Errors
    /// Retorna `AppError::UserNotFound` si no existe ningún usuario con ese nombre.
    fn get_user_by_username(&mut self, username: String) -> Result<User, AppError>;
}
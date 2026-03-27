use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;

use crate::traits::Iuser_repository::IUserRepository;
use crate::traits::Isession_repository::ISessionRepository;
use crate::errors::AppError;
use crate::entities::user::User;

/// Lógica de negocio para autenticación y gestión de sesión.
///
/// Usa Argon2 para el hash y verificación de contraseñas.
/// El token de sesión también es un hash Argon2 del `user_id`.
pub struct UserService {
    user_repository: Box<dyn IUserRepository>,
    session_repository: Box<dyn ISessionRepository>,
}

impl UserService {
    /// Crea una nueva instancia con los repositorios indicados.
    ///
    /// # Arguments
    /// * `user_repository` - Implementación de `IUserRepository` a utilizar.
    /// * `session_repository` - Implementación de `ISessionRepository` a utilizar.
    pub fn new(user_repository: Box<dyn IUserRepository>, session_repository: Box<dyn ISessionRepository>) -> Self {
        Self { user_repository, session_repository }
    }

    /// Registra un nuevo usuario y abre una sesión para él.
    ///
    /// # Arguments
    /// * `username` - Nombre de usuario deseado (mínimo 3 caracteres, único).
    /// * `password` - Contraseña en texto plano (mínimo 6 caracteres).
    pub fn register_user(&mut self, username: String, password: String) -> Result<User, AppError> {
        self.validate_username(&username)?;
        self.validate_password(&password)?;

        let password_hash = self.hash_password(&password)?;
        let user: User = self.user_repository.create_user(username, password_hash)?;
        self.create_session(user.id)?;
        Ok(user)
    }

    /// Verifica que el nombre de usuario sea único y cumpla con la longitud mínima.
    ///
    /// # Errors
    /// - `AppError::UserAlreadyExists` si el nombre ya está registrado.
    /// - `AppError::InvalidArgument` si el nombre está vacío o tiene menos de 3 caracteres.
    fn validate_username(&mut self, username: &str) -> Result<(), AppError> {
        let user_in_db = self.user_repository.get_user_by_username(username.to_string());
        if user_in_db.is_ok() {
            return Err(AppError::UserAlreadyExists);
        }
        if username.trim().is_empty() {
            return Err(AppError::InvalidArgument(
                "El nombre de usuario no puede estar vacío".to_string(),
            ));
        }

        if username.len() < 3 {
            return Err(AppError::InvalidArgument(
                "El nombre de usuario debe tener al menos 3 caracteres".to_string(),
            ));
        }

        Ok(())
    }

    /// Verifica que la contraseña cumpla con la longitud mínima.
    ///
    /// # Errors
    /// - `AppError::InvalidArgument` si la contraseña tiene menos de 6 caracteres.
    fn validate_password(&self, password: &str) -> Result<(), AppError> {
        if password.len() < 6 {
            return Err(AppError::InvalidArgument(
                "La contraseña debe tener al menos 6 caracteres".to_string(),
            ));
        }

        Ok(())
    }

    /// Genera un hash Argon2 con sal aleatoria para el texto dado.
    ///
    /// # Errors
    /// - `AppError::InvalidArgument` si Argon2 falla al hashear.
    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        match Argon2::default().hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(e) => Err(AppError::InvalidArgument(e.to_string())),
        }
    }

    /// Valida las credenciales y abre una sesión si son correctas.
    ///
    /// Para evitar enumerar usuarios, ante credenciales inválidas
    /// siempre se devuelve `AppError::InvalidCredentials`.
    ///
    /// # Arguments
    /// * `username` - Nombre de usuario.
    /// * `password` - Contraseña en texto plano.
    pub fn login_user(&mut self, username: String, password: String) -> Result<User, AppError> {
        let user = match self.user_repository.get_user_by_username(username.clone()) {
            Ok(user) => user,
            Err(_) => return Err(AppError::InvalidCredentials),
        };

        let password_is_valid = self.verify_password(&password, &user.password_hash)?;
        if !password_is_valid {
            return Err(AppError::InvalidCredentials);
        }
        self.create_session(user.id)?;
        Ok(user)
    }

    /// Comprueba si `password` coincide con el `hash` almacenado.
    ///
    /// # Returns
    /// * `Ok(true)` si la contraseña es correcta, `Ok(false)` si no.
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Genera un token y persiste la sesión para el usuario indicado.
    ///
    /// # Arguments
    /// * `user_id` - Id del usuario que inicia sesión.
    fn create_session(&self, user_id: u32) -> Result<(), AppError> {
        let token = self.hash_password(&user_id.to_string())?;
        self.session_repository.create_session(user_id, token)
    }

    /// Devuelve el usuario con sesión activa tras verificar el token.
    ///
    /// Si el token es inválido, elimina la sesión corrupta y retorna error.
    ///
    /// # Errors
    /// - `AppError::NoActiveSession` si no existe archivo de sesión.
    /// - `AppError::InvalidSession` si el token no corresponde al usuario.
    pub fn get_active_user(&mut self) -> Result<User, AppError> {
        let session = self.session_repository.get_session()?;
        let token_is_valid = self.verify_password(&session.user_id.to_string(), &session.token)?;
        if !token_is_valid {
            self.session_repository.delete_session()?;
            return Err(AppError::InvalidSession);
        }
        self.user_repository.get_user(session.user_id)
    }

    /// Cierra la sesión activa eliminando el archivo de sesión.
    pub fn logout_user(&mut self) -> Result<(), AppError> {
        self.session_repository.delete_session()
    }
}

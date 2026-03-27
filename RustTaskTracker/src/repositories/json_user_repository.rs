use std::fs;
use std::path::PathBuf;

use crate::errors::AppError;
use crate::traits::Iuser_repository::IUserRepository;
use crate::entities::user::User;

/// Implementación de `IUserRepository` que persiste los usuarios en un archivo JSON compartido.
///
/// Mantiene una copia en memoria (`users`) que se recarga desde disco
/// antes de cada operación de escritura para minimizar inconsistencias.
pub struct JsonUserRepository {
    file_path: PathBuf,
    users: Vec<User>,
}

impl JsonUserRepository {
    /// Crea una nueva instancia y carga los usuarios desde el archivo indicado.
    ///
    /// Si el archivo no existe todavía, la instancia parte con un listado vacío.
    ///
    /// # Arguments
    /// * `file_path` - Ruta al archivo JSON de usuarios.
    pub fn new(file_path: PathBuf) -> Self {
        let mut instance = Self {
            file_path,
            users: Vec::new(),
        };
        let _ = instance.load();
        instance
    }

    /// Recarga el listado de usuarios desde el archivo JSON.
    /// Si el archivo no existe, deja el listado vacío.
    fn load(&mut self) -> Result<(), AppError> {
        if !self.file_path.exists() {
            self.users = Vec::new();
            return Ok(());
        }
        let contents = fs::read_to_string(&self.file_path)?;
        let users: Vec<User> = serde_json::from_str(&contents)?;
        self.users = users;
        Ok(())
    }

    /// Serializa el listado en memoria y lo escribe en el archivo JSON.
    fn save(&self) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(&self.users)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    /// Devuelve el id más alto del listado, o `0` si está vacío.
    /// Se usa para generar el próximo id de forma incremental.
    fn find_max_id(&self) -> u32 {
        self.users.iter().map(|user| user.id).max().unwrap_or(0)
    }
}

impl IUserRepository for JsonUserRepository {
    fn create_user(&mut self, username: String, password_hash: String) -> Result<User, AppError> {
        self.load()?;
        let new_user = User {
            id: self.find_max_id() + 1,
            username,
            password_hash,
        };
        self.users.push(new_user.clone());
        self.save()?;
        Ok(new_user)
    }

    fn get_user(&mut self, id: u32) -> Result<User, AppError> {
        self.load()?;
        if let Some(user) = self.users.iter().find(|user| user.id == id) {
            return Ok(User {
                id: user.id,
                username: user.username.clone(),
                password_hash: user.password_hash.clone(),
            });
        }

        Err(AppError::UserNotFound)
    }

    fn get_user_by_username(&mut self, username: String) -> Result<User, AppError> {
        self.load()?;
        if let Some(user) = self.users.iter().find(|user| user.username == username) {
            return Ok(User {
                id: user.id,
                username: user.username.clone(),
                password_hash: user.password_hash.clone(),
            });
        }

        Err(AppError::UserNotFound)
    }
}
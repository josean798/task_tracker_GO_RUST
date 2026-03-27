use std::fs;
use std::path::PathBuf;

use crate::entities::session::Session;
use crate::traits::Isession_repository::ISessionRepository;
use crate::errors::AppError;

/// Implementación de `ISessionRepository` que persiste la sesión activa en un archivo JSON.
///
/// Solo existe un archivo de sesión (`session.json`). Cuando el usuario
/// cierra sesión, el archivo se elimina del disco.
pub struct JsonSessionRepository {
    file_path: PathBuf,
}

impl JsonSessionRepository {
    /// Crea una nueva instancia apuntando al archivo de sesión indicado.
    ///
    /// # Arguments
    /// * `file_path` - Ruta al archivo JSON de sesión.
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Serializa la sesión y la escribe en el archivo JSON.
    ///
    /// # Arguments
    /// * `session` - Sesión a persistir.
    fn save(&self, session: &Session) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(&session)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }
}

impl ISessionRepository for JsonSessionRepository {
    fn create_session(&self, user_id: u32, token: String) -> Result<(), AppError> {
        let session = Session { user_id, token };
        self.save(&session)
    }

    fn get_session(&self) -> Result<Session, AppError> {
        if !self.file_path.exists() {
            return Err(AppError::NoActiveSession);
        }
        let contents = fs::read_to_string(&self.file_path)?;
        let session: Session = serde_json::from_str(&contents)?;
        Ok(session)
    }
    
    fn delete_session(&self) -> Result<(), AppError> {
        if self.file_path.exists() {
            fs::remove_file(&self.file_path)?;
        }
        Ok(())
    }
}
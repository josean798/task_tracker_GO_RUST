use serde::{Deserialize, Serialize};

/// Representa la sesión activa de un usuario.
///
/// El token es el hash Argon2 del `user_id` y se usa para
/// verificar que la sesión almacenada en disco no fue alterada.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub user_id: u32,
    pub token: String,
}
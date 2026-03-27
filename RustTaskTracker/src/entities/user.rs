use serde::{Deserialize, Serialize};

/// Representa un usuario registrado en el sistema.
///
/// La contraseña nunca se almacena en texto plano;
/// solo se guarda el hash generado por Argon2.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
}
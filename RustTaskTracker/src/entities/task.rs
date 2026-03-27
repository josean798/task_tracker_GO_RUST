use chrono::{Local, prelude::DateTime};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Estados posibles de una tarea.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TaskStatus {
    /// Pendiente: aún no se ha iniciado.
    Todo,
    /// En progreso: actualmente en ejecución.
    InProgress,
    /// Completada: finalizada exitosamente.
    Done,
}

/// Representa una tarea del usuario.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Task {
    /// Crea una nueva tarea con estado `Todo` y marca de tiempo actual.
    ///
    /// # Arguments
    /// * `id` - Identificador único de la tarea.
    /// * `description` - Descripción de la tarea.
    pub fn new(id: u32, description: String) -> Self {
        let now = Local::now();

        Self {
            id,
            description,
            status: TaskStatus::Todo,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reemplaza la descripción y actualiza `updated_at`.
    ///
    /// # Arguments
    /// * `new_description` - Nueva descripción a asignar.
    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
        self.updated_at = Local::now();
    }

    /// Cambia el estado de la tarea y actualiza `updated_at`.
    ///
    /// # Arguments
    /// * `new_status` - Nuevo estado a asignar.
    pub fn update_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
        self.updated_at = Local::now();
    }

}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self.status {
            TaskStatus::Todo => "Pendiente",
            TaskStatus::InProgress => "En Progreso",
            TaskStatus::Done => "Completada",
        };
        write!(
            f,
            "  [{}] {}\n      Estado:{}\n      Fecha de Creación: {}\n      Fecha de Actualización: {}",
            self.id,
            self.description,
            status_str,
            self.created_at.format("%Y-%m-%d %H:%M"),
            self.updated_at.format("%Y-%m-%d %H:%M"),
        )
    }
}

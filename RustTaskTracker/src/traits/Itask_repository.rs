use crate::errors::AppError;
use crate::entities::task::Task;

/// Contrato de persistencia para tareas.
///
/// Cualquier implementación (JSON, SQL, en memoria, etc.) debe
/// satisfacer estos métodos para ser utilizable por `TaskService`.
pub trait ITaskRepository {
    /// Crea y persiste una nueva tarea con la descripción dada.
    fn create_task(&self, description: String) -> Result<Task, AppError>;

    /// Devuelve todas las tareas almacenadas.
    fn get_all_tasks(&self) -> Result<Vec<Task>, AppError>;

    /// Busca y devuelve la tarea con el `id` indicado.
    ///
    /// # Errors
    /// Retorna `AppError::TaskNotFound` si no existe ninguna tarea con ese id.
    fn get_task(&self, id: u32) -> Result<Task, AppError>;

    /// Reemplaza la tarea con el `id` indicado por `task`.
    ///
    /// # Errors
    /// Retorna `AppError::TaskNotFound` si no existe ninguna tarea con ese id.
    fn update_task(&self, id: u32, task: Task) -> Result<Task, AppError>;

    /// Elimina la tarea con el `id` indicado.
    ///
    /// # Errors
    /// Retorna `AppError::TaskNotFound` si no existe ninguna tarea con ese id.
    fn delete_task(&self, id: u32) -> Result<(), AppError>;
}

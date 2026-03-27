use crate::traits::Itask_repository::ITaskRepository;
use crate::errors::AppError;
use crate::entities::task::{Task, TaskStatus};

/// Lógica de negocio para la gestión de tareas.
///
/// Recibe los argumentos crudos del CLI, los valida, y delega la
/// persistencia al repositorio inyectado.
pub struct TaskService {
    task_repository: Box<dyn ITaskRepository>,
}

impl TaskService {
    /// Crea una nueva instancia con el repositorio indicado.
    ///
    /// # Arguments
    /// * `task_repository` - Implementación de `ITaskRepository` a utilizar.
    pub fn new(task_repository: Box<dyn ITaskRepository>) -> Self {
        Self { task_repository }
    }

    /// Crea una nueva tarea a partir de los argumentos del comando `add`.
    ///
    /// # Arguments
    /// * `args` - Se espera `args[0]` como descripción de la tarea.
    pub fn add_task(&self, args: Vec<String>) -> Result<Task, AppError> {
        if args.is_empty() {
            return Err(AppError::MissingArgument("description".to_string()));
        }
        let description = args[0].clone();
        self.task_repository.create_task(description)
    }

    /// Actualiza la descripción de una tarea existente.
    ///
    /// # Arguments
    /// * `args` - Se esperan `args[0]` como id y `args[1]` como nueva descripción.
    pub fn update_task(&self, args: Vec<String>) -> Result<Task, AppError> {
        if args.len() < 2 {
            return Err(AppError::MissingArgument("id and description".to_string()));
        }
        let id = self.parse_id(&args[0])?;
        let new_description = args[1].clone();
        let mut task = self.task_repository.get_task(id)?;
        task.update_description(new_description);
        self.task_repository.update_task(id, task)
    }

    /// Elimina una tarea por su id y devuelve el id eliminado.
    ///
    /// # Arguments
    /// * `args` - Se espera `args[0]` como id de la tarea a eliminar.
    pub fn delete_task(&self, args: Vec<String>) -> Result<u32, AppError> {
        if args.is_empty() {
            return Err(AppError::MissingArgument("id".to_string()));
        }
        let id = self.parse_id(&args[0])?;
        self.task_repository.delete_task(id)?;
        Ok(id)
    }

    /// Cambia el estado de una tarea al valor indicado.
    ///
    /// # Arguments
    /// * `args` - Se espera `args[0]` como id de la tarea.
    /// * `status` - Nuevo estado a asignar.
    pub fn mark_task_status(&self, args: Vec<String>, status: TaskStatus) -> Result<Task, AppError> {
        if args.is_empty() {
            return Err(AppError::MissingArgument("id".to_string()));
        }
        let id = self.parse_id(&args[0])?;
        let mut task = self.task_repository.get_task(id)?;
        task.update_status(status);
        self.task_repository.update_task(id, task)
    }

    /// Devuelve todas las tareas del usuario.
    pub fn list_tasks(&self) -> Result<Vec<Task>, AppError> {
        self.task_repository.get_all_tasks()
    }

    /// Devuelve las tareas filtradas por el estado indicado.
    ///
    /// # Arguments
    /// * `status` - Estado por el cual se filtran las tareas.
    pub fn list_tasks_by_status(&self, status: &TaskStatus) -> Result<Vec<Task>, AppError> {
        let tasks = self.task_repository.get_all_tasks()?;
        Ok(tasks.into_iter().filter(|t| &t.status == status).collect())
    }

    /// Convierte una cadena a un id numérico (`u32`).
    ///
    /// # Errors
    /// Retorna `AppError::InvalidArgument` si la cadena no es un número válido.
    fn parse_id(&self, raw_id: &str) -> Result<u32, AppError> {
        raw_id.parse::<u32>().map_err(|_| {
            AppError::InvalidArgument(format!("'{}' No es una id válida", raw_id))
        })
    }
}

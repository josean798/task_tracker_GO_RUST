use std::fs;
use std::path::PathBuf;

use crate::errors::AppError;
use crate::traits::Itask_repository::ITaskRepository;
use crate::entities::task::Task;

/// Implementación de `ITaskRepository` que persiste las tareas en un archivo JSON.
///
/// Cada usuario tiene su propio archivo (`tasks_<id>.json`) dentro del
/// directorio `src/storage`, por lo que el repositorio no comparte estado
/// entre usuarios.
pub struct JsonTaskRepository {
    file_path: PathBuf,
}

impl JsonTaskRepository {
    /// Crea una nueva instancia apuntando al archivo indicado.
    ///
    /// # Arguments
    /// * `file_path` - Ruta al archivo JSON de tareas del usuario.
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Lee el archivo JSON y devuelve el listado de tareas.
    /// Si el archivo no existe, devuelve un vector vacío.
    fn load(&self) -> Result<Vec<Task>, AppError> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }
        let contents = fs::read_to_string(&self.file_path)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    /// Serializa el listado de tareas y lo escribe en el archivo JSON.
    ///
    /// # Arguments
    /// * `tasks` - Listado de tareas a persistir.
    fn save(&self, tasks: &[Task]) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }
}

impl ITaskRepository for JsonTaskRepository {
    fn create_task(&self, description: String) -> Result<Task, AppError> {
        if description.is_empty() {
            return Err(AppError::EmptyDescription);
        }
        let mut tasks = self.load()?;
        let mut max_id: u32 = 0;
        for task in &tasks {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        let new_task = Task::new(max_id + 1, description);
        tasks.push(new_task.clone());
        self.save(&tasks)?;
        Ok(new_task)
    }

    fn get_all_tasks(&self) -> Result<Vec<Task>, AppError> {
        self.load()
    }

    fn get_task(&self, id: u32) -> Result<Task, AppError> {
        let tasks = self.load()?;
        for task in tasks {
            if task.id == id {
                return Ok(task);
            }
        }
        Err(AppError::TaskNotFound(id))
    }

    fn update_task(&self, id: u32, task: Task) -> Result<Task, AppError> {
        let mut tasks = self.load()?;
        let mut found = false;
        for i in 0..tasks.len() {
            if tasks[i].id == id {
                tasks[i] = task.clone();
                found = true;
                break;
            }
        }
        if !found {
            return Err(AppError::TaskNotFound(id));
        }
        self.save(&tasks)?;
        Ok(task)
    }

    fn delete_task(&self, id: u32) -> Result<(), AppError> {
        let mut tasks = self.load()?;
        let mut found_index: Option<usize> = None;
        for i in 0..tasks.len() {
            if tasks[i].id == id {
                found_index = Some(i);
                break;
            }
        }
        match found_index {
            Some(index) => {
                tasks.remove(index);
                self.save(&tasks)?;
                Ok(())
            }
            None => Err(AppError::TaskNotFound(id)),
        }
    }
}

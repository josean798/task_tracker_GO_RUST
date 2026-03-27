use crate::services::task_service::TaskService;
use crate::services::user_service::UserService;
use crate::entities::task::{Task, TaskStatus};
use crate::errors::AppError;

/// Comandos soportados por el programa
pub enum Command {
    Register,
    Login,
    Logout,
    Add,
    Update,
    Delete,
    MarkInProgress,
    MarkDone,
    List,
    ListDone,
    ListTodo,
    ListInProgress,
    Help,
    Unknown,
}

/// Se encarga de parsear los comandos y decidir qué acciones deben realizar los servicios.
pub struct Cli {
    task_service: Option<TaskService>,
    user_service: UserService,
}

impl Cli {
    /// Crea una nueva instancia de CLI.
    ///
    /// # Arguments
    /// * `task_service` - Servicio de tareas opcional, disponible cuando hay sesión activa.
    /// * `user_service` - Servicio de usuarios para autenticación y sesión.
    pub fn new(task_service: Option<TaskService>, user_service: UserService) -> Self {
        Self { task_service, user_service }
    }

    /// Obtiene una referencia al servicio de tareas si existe una sesión activa.
    /// Si no existe una sesión activa, devuelve un error indicando que el usuario debe iniciar sesión.
    ///
    /// # Returns
    /// * `Ok(&TaskService)` si hay sesión activa.
    /// * `Err(AppError::NoActiveSession)` si no hay sesión activa.
    fn require_task_service(&self) -> Result<&TaskService, AppError> {
        match &self.task_service {
            Some(service) => Ok(service),
            None => Err(AppError::NoActiveSession),
        } 
    }

    /// Procesa los argumentos de entrada, determina el comando y ejecuta su handler.
    ///
    /// # Arguments
    /// * `arguments` - Lista de argumentos recibidos desde la línea de comandos.
    pub fn process_arguments(&mut self, arguments: Vec<String>) {
        if arguments.is_empty() {
            self.print_help();
            return;
        }

        let command = self.parse_command(&arguments[0]);
        let remaining_args = arguments[1..].to_vec();

        let command = match command {
            Command::List => self.parse_list_subcommand(&remaining_args),
            _ => command,
        };

        let result = match command {
            Command::Register => self.handle_register(&remaining_args),
            Command::Login => self.handle_login(&remaining_args),
            Command::Logout => self.handle_logout(),
            Command::Add => self.handle_add(&remaining_args),
            Command::Update => self.handle_update(&remaining_args),
            Command::Delete => self.handle_delete(&remaining_args),
            Command::MarkInProgress => self.handle_mark_status(&remaining_args, TaskStatus::InProgress),
            Command::MarkDone => self.handle_mark_status(&remaining_args, TaskStatus::Done),
            Command::List => self.handle_list(),
            Command::ListDone => self.handle_list_by_status(TaskStatus::Done),
            Command::ListTodo => self.handle_list_by_status(TaskStatus::Todo),
            Command::ListInProgress => self.handle_list_by_status(TaskStatus::InProgress),
            Command::Help => {
                self.print_help();
                Ok(())
            }
            Command::Unknown => {
                println!("\nComando desconocido. Usa 'help' para ver la lista de comandos disponibles.");
                Ok(())
            }
        };

        // Solo se manejan errores relacionados con acceso a datos para evitar mostrar mensajes técnicos al usuario
        if let Err(error) = result {
            match &error {
                AppError::IoError(_) | AppError::JsonError(_) => {
                    println!("\nOcurrió un error al acceder a los datos. Por favor, inténtalo de nuevo.");
                    return;
                },
                _ => println!("Error: {}", error),
            }
        }
    }

    /// Convierte un comando en texto a su variante de `Command`.
    ///
    /// # Arguments
    /// * `raw_command` - Comando principal ingresado por el usuario.
    ///
    /// # Returns
    /// Devuelve la variante de `Command` correspondiente al texto recibido.
    /// Si no coincide con un comando conocido, retorna `Command::Unknown`.
    pub fn parse_command(&self, raw_command: &str) -> Command {
        match raw_command {
            "register" => Command::Register,
            "login" => Command::Login,
            "logout" => Command::Logout,
            "add" => Command::Add,
            "update" => Command::Update,
            "delete" => Command::Delete,
            "mark-in-progress" => Command::MarkInProgress,
            "mark-done" => Command::MarkDone,
            "list" => Command::List,
            "help" => Command::Help,
            _ => Command::Unknown,
        }
    }

    /// Interpreta el subcomando de `list` según los argumentos proporcionados.
    ///
    /// # Arguments
    /// * `args` - Argumentos restantes luego del comando principal.
    ///
    /// # Returns
    /// Retorna el filtro de listado correspondiente (`ListDone`, `ListTodo`, `ListInProgress`).
    /// Si no hay subcomando válido, retorna `Command::List`.
    pub fn parse_list_subcommand(&self, args: &[String]) -> Command {
        if args.is_empty() {
            return Command::List;
        }
        match args[0].as_str() {
            "done" => Command::ListDone,
            "todo" => Command::ListTodo,
            "in-progress" => Command::ListInProgress,
            _ => Command::List,
        }
    }

    /// Registra un nuevo usuario y crea su sesión.
    ///
    /// # Arguments
    /// * `args` - Argumentos esperados: usuario y contraseña.
    fn handle_register(&mut self, args: &[String]) -> Result<(), AppError> {
        let (username, password) = self.extract_credentials(args)?;
        let _ = self.user_service.register_user(username, password)?;
        println!("Usuario registrado exitosamente");
        Ok(())
    }

    /// Inicia sesión con credenciales válidas.
    ///
    /// # Arguments
    /// * `args` - Argumentos esperados: usuario y contraseña.
    fn handle_login(&mut self, args: &[String]) -> Result<(), AppError> {
        let (username, password) = self.extract_credentials(args)?;
        let user = self.user_service.login_user(username, password)?;
        println!("Usuario {} ha iniciado sesión exitosamente", user.username);
        Ok(())
    }

    /// Cierra la sesión activa del usuario.
    fn handle_logout(&mut self) -> Result<(), AppError> {
        self.user_service.logout_user()?;
        println!("Sesión cerrada exitosamente");
        Ok(())
    }

    /// Crea una nueva tarea para el usuario autenticado.
    ///
    /// # Arguments
    /// * `args` - Argumentos esperados para construir la descripción.
    fn handle_add(&self, args: &[String]) -> Result<(), AppError> {
        let task = self.require_task_service()?.add_task(args.to_vec())?;
        println!("Tarea: {} añadida exitosamente (ID: {})", task.description, task.id);
        Ok(())
    }

    /// Actualiza la descripción de una tarea existente.
    ///
    /// # Arguments
    /// * `args` - Argumentos esperados: id y nueva descripción.
    fn handle_update(&self, args: &[String]) -> Result<(), AppError> {
        let task = self.require_task_service()?.update_task(args.to_vec())?;
        println!("Tarea actualizada exitosamente (ID: {})", task.id);
        Ok(())
    }

    /// Elimina una tarea por su identificador.
    ///
    /// # Arguments
    /// * `args` - Argumentos esperados: id de la tarea.
    fn handle_delete(&self, args: &[String]) -> Result<(), AppError> {
        let id = self.require_task_service()?.delete_task(args.to_vec())?;
        println!("Tarea eliminada exitosamente (ID: {})", id);
        Ok(())
    }

    /// Marca una tarea con el estado indicado.
    ///
    /// # Arguments
    /// * `args` - Argumentos esperados: id de la tarea.
    /// * `status` - Nuevo estado a asignar.
    fn handle_mark_status(&self, args: &[String], status: TaskStatus) -> Result<(), AppError> {
        let task = self.require_task_service()?.mark_task_status(args.to_vec(), status)?;
        println!("Tarea {} marcada como {:?}", task.id, task.status);
        Ok(())
    }

    /// Lista todas las tareas del usuario autenticado.
    fn handle_list(&self) -> Result<(), AppError> {
        let tasks = self.require_task_service()?.list_tasks()?;
        if tasks.is_empty() {
            println!("No se encontraron tareas.");
        } else {
            println!("\nMis Tareas:");
            self.print_task_summary(&tasks);
            println!();
            tasks.iter().for_each(|t| println!("{}\n", t));
        }
        Ok(())
    }

    /// Lista tareas filtradas por estado.
    ///
    /// # Arguments
    /// * `status` - Estado por el cual se filtran las tareas.
    fn handle_list_by_status(&self, status: TaskStatus) -> Result<(), AppError> {
        let tasks = self.require_task_service()?.list_tasks_by_status(&status)?;
        if tasks.is_empty() {
            println!("\nNo se encontraron tareas con estado {:?}.", self.get_task_state(&status));
        } else {
            println!("\nMis Tareas con estado {}:", self.get_task_state(&status));
            println!("\nTareas totales: {}", tasks.len());
            tasks.iter().for_each(|t| println!("{}\n", t));
        }
        Ok(())
    }

    /// Extrae usuario y contraseña desde los argumentos del comando.
    ///
    /// # Arguments
    /// * `args` - Lista de argumentos de entrada.
    ///
    /// # Returns
    /// * `Ok((username, password))` cuando ambos valores están presentes.
    /// * `Err(AppError::MissingArgument(_))` cuando falta alguno de los dos.
    fn extract_credentials(&self, args: &[String]) -> Result<(String, String), AppError> {
        let username = args
            .first()
            .cloned()
            .ok_or_else(|| AppError::MissingArgument("username".to_string()))?;
        let password = args
            .get(1)
            .cloned()
            .ok_or_else(|| AppError::MissingArgument("password".to_string()))?;
        Ok((username, password))
    }

    /// Imprime la ayuda general y la lista de comandos disponibles.
    fn print_help(&self) {
        println!();
        println!("TaskTracker - Gestor de tareas por linea de comandos");
        println!();
        println!("Autores:");
        println!("- Abisaac Carmona - CI: 32.218.469");
        println!("- José Puerta     - CI: 31.904.115");           
        println!();
        println!("Uso: task-cli <command> [arguments]");
        println!();
        println!("Comandos:");
        println!("  register <usuario> <contraseña>   Registra un nuevo usuario");
        println!("  login <usuario> <contraseña>      Inicia sesión con un usuario existente");
        println!("  add <descripcion>           Agrega una nueva tarea");
        println!("  update <id> <descripcion>   Actualiza la descripcion de una tarea");
        println!("  delete <id>                 Elimina una tarea");
        println!("  mark-in-progress <id>       Marca una tarea como en progreso");
        println!("  mark-done <id>              Marca una tarea como completada");
        println!("  list                        Lista todas las tareas");
        println!("  list done                   Lista las tareas completadas");
        println!("  list todo                   Lista las tareas pendientes");
        println!("  list in-progress            Lista las tareas en progreso");
        println!("  help                        Muestra esta ayuda");
    }

    /// Imprime un resumen de cantidades por estado.
    ///
    /// # Arguments
    /// * `tasks` - Colección de tareas a resumir.
    fn print_task_summary(&self, tasks: &[Task]) {
        let mut todo = 0;
        let mut in_progress = 0;
        let mut done = 0;

        for task in tasks {
            match task.status {
                TaskStatus::Todo => todo += 1,
                TaskStatus::InProgress => in_progress += 1,
                TaskStatus::Done => done += 1,
            }
        }
        println!("\nTareas totales: {}", tasks.len());
        println!("Tareas Pendientes: {}", todo);
        println!("Tareas En Progreso: {}", in_progress);
        println!("Tareas Completadas: {}", done);
    }

    /// Devuelve una etiqueta legible para un estado de tarea.
    ///
    /// # Arguments
    /// * `status` - Estado a convertir en texto.
    ///
    /// # Returns
    /// Una cadena legible para mostrar el estado en la interfaz.
    fn get_task_state(&self, status: &TaskStatus) -> String {
        match status {
            TaskStatus::Todo => "Pendiente".to_string(),
            TaskStatus::InProgress => "En Progreso".to_string(),
            TaskStatus::Done => "Completada".to_string(),
        }
    }
}

mod traits;
mod entities;
mod repositories;
mod services;
mod cli;
mod errors;

use std::env;
use std::path::PathBuf;

use crate::repositories::json_task_repository::JsonTaskRepository;
use crate::repositories::json_user_repository::JsonUserRepository;
use crate::repositories::json_session_repository::JsonSessionRepository;
use crate::services::task_service::TaskService;
use crate::services::user_service::UserService;
use crate::cli::Cli;

/// Punto de entrada de la aplicación.
///
/// Inyecta las dependencias (repositorios → servicios → CLI),
/// verifica si hay un usuario con sesión activa para inicializar el
/// servicio de tareas, y delega el procesamiento de argumentos al CLI.
fn main() {
    let storage_path: PathBuf = PathBuf::from("src/storage");
    let users_file_path: PathBuf = storage_path.join("users.json");
    let session_file_path: PathBuf = storage_path.join("session.json");

    let session_repository: JsonSessionRepository = JsonSessionRepository::new(session_file_path);
    let user_repository: JsonUserRepository = JsonUserRepository::new(users_file_path);
    // Se inyectan las dependencias de los repositorios en el servicio de usuario
    // esto por si acaso en un futuro se tiene que cambiar a SQL
    let mut user_service: UserService = UserService::new(Box::new(user_repository), Box::new(session_repository));

    let active_user = match user_service.get_active_user() {
        Ok(user) => Some(user),
        Err(_) => None,
    };

    // Si hay un usuario activo, se crea el servicio de tareas con su repositorio correspondiente
    // En el caso de que no haya usuario activo, se pasa None y el CLI se encarga de decirle al usuario que se loguee
    let task_service = match active_user {
        Some(user) => {
            let path = storage_path.join(format!("tasks_{}.json", user.id));
            Some(TaskService::new(Box::new(JsonTaskRepository::new(path))))
        },
        None => None,
    };

    let mut cli = Cli::new(task_service, user_service);

    // Con esto se obtienen los argumentos que se pasan cuando se corre el programa
    let mut raw_args: Vec<String> = env::args().collect();
    let args: Vec<String> = raw_args.split_off(1);

    cli.process_arguments(args);
}

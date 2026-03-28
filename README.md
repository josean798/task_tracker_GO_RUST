# Task Tracker GO & RUST

Este Repositorio contiene tres implementaciones de un gestor de tareas por la terminal: dos variantes en Go y una en Rust. Cada implementacion permite registrar usuarios, gestionar sesiones y realizar operaciones CRUD sobre tareas.

---

## Clonar el repositorio

```bash
git clone https://github.com/josean798/task_tracker_GO_RUST.git
cd task_tracker_GO_RUST
```

---

## Indice

- [GoTaskTracker](#gotasktracker)
- [GoTaskTracker COBRA](#gotasktracker-cobra)
- [RustTaskTracker](#rusttasktracker)
- [Autores](#autores)

---

## GoTaskTracker

### Descripcion

Implementacion en Go del gestor de tareas utilizando unicamente la biblioteca estandar del lenguaje (Al principio pensamos que no se podía utilizar librerías). El parseo de comandos se realiza manualmente utilizando un `switch-case` sobre los argumentos de entrada. Es la version base del proyecto en Go.

### Requisitos previos

- Go 1.25 o superior

Verificar la version instalada:

```bash
go version
```

### Instalacion rapida

```bash
# Ubicarse en el directorio del proyecto
cd GoTaskTracker

# Compilar el binario
go build -o task-cli

# Ejecutar
./task-cli <comando> [argumentos]
```

### Arquitectura

El proyecto esta organizado en cinco paquetes:

```
GoTaskTracker/
├── main.go              # Punto de entrada. Parseo de argumentos con switch-case
├── go.mod               # Definicion del modulo Go (sin dependencias externas)
├── auth/
│   └── auth.go          # Registro, login y logout. Gestion del archivo de sesion
├── models/
│   └── models.go        # Estructuras de datos: User y Task
├── storage/
│   └── storage.go       # Lectura y escritura del archivo JSON de usuarios
└── tasks/
    └── tasks.go         # Operaciones CRUD sobre las tareas del usuario activo
```

**Flujo de ejecucion:**

```
main.go
  └── storage.LoadUsers()         # Carga usuarios desde usuarios.json
  └── auth.GetLoggedUser()        # Lee la sesion activa desde sesion.json
  └── switch command              # Despacha al paquete correspondiente
        ├── auth.Register()
        ├── auth.Login()
        ├── auth.Logout()
        ├── tasks.AddTask()
        ├── tasks.ListTasks()
        ├── tasks.DeleteTask()
        └── tasks.UpdateTask()
```

### Comandos disponibles

| Comando    | Argumentos                      | Descripcion                                          |
| ---------- | ------------------------------- | ---------------------------------------------------- |
| `register` | `<usuario> <password>`          | Registrar un nuevo usuario                           |
| `login`    | `<usuario> <password>`          | Iniciar sesion                                       |
| `logout`   | (ninguno)                       | Cerrar la sesion activa                              |
| `add`      | `<titulo...>`                   | Agregar una nueva tarea (admite titulo con espacios) |
| `list`     | `[todo\|done\|inprogress]`      | Listar todas las tareas o filtrar por estado         |
| `delete`   | `<id>`                          | Eliminar una tarea por su ID                         |
| `update`   | `<id> <todo\|done\|inprogress>` | Cambiar el estado de una tarea                       |

**Estados de tarea:**

| Valor CLI    | Descripcion       |
| ------------ | ----------------- |
| `todo`       | Tarea pendiente   |
| `inprogress` | Tarea en progreso |
| `done`       | Tarea completada  |

**Ejemplos de uso:**

```bash
# Registrar un usuario
./task-cli register juan password123

# Iniciar sesion
./task-cli login juan password123

# Agregar tareas
./task-cli add "Estudiar para café café"
./task-cli add "Estudiar para argenis"

# Listar tareas
./task-cli list
./task-cli list todo
./task-cli list inprogress
./task-cli list done

# Cambiar estado de una tarea
./task-cli update 1 inprogress
./task-cli update 1 done

# Eliminar una tarea
./task-cli delete 2

# Cerrar sesion
./task-cli logout
```

### Almacenamiento de datos

Los archivos se crean en el directorio desde donde se ejecuta el binario:

| Archivo         | Descripcion                                          |
| --------------- | ---------------------------------------------------- |
| `usuarios.json` | Lista de todos los usuarios con sus tareas embebidas |
| `sesion.json`   | Datos del usuario con sesion activa                  |

Estructura de `usuarios.json`:

```json
[
  {
    "id": 1,
    "username": "juan",
    "password": "password123",
    "tasks": [
      {
        "id": 1,
        "title": "Revisar el informe final",
        "status": "pending",
        "created_at": "2026-03-27 10:30:45"
      }
    ]
  }
]
```

> Nota: El valor `"pending"` en el campo `status` corresponde al estado `todo` en la interfaz de comandos.

---

## GoTaskTracker COBRA

### Descripcion

Variante mejorada de GoTaskTracker que utiliza el framework [Cobra](https://github.com/spf13/cobra) para la gestion de comandos. Ofrece la misma funcionalidad que la version base, pero agrega validacion automatica de argumentos, mensajes de ayuda generados automaticamente y una estructura de comandos mas robusta.

### Requisitos previos

- Go 1.25 o superior

Verificar la version instalada:

```bash
go version
```

### Instalacion rapida

```bash
# Ubicarse en el directorio del proyecto
cd GoTaskTracker_COBRA

# Descargar dependencias
go mod tidy

# Compilar el binario
go build -o task-cli

# Ejecutar
./task-cli <comando> [argumentos]
```

### Arquitectura

La estructura de paquetes es identica a GoTaskTracker. La diferencia principal reside en `main.go`, donde cada comando se define como una instancia de `cobra.Command` en lugar de una rama `switch-case`:

```
GoTaskTracker_COBRA/
├── main.go              # Punto de entrada con definicion de comandos Cobra
├── go.mod               # Modulo Go con dependencia de Cobra v1.10.2
├── go.sum               # Checksums de dependencias
├── auth/
│   └── auth.go          # Registro, login y logout (identico a GoTaskTracker)
├── models/
│   └── models.go        # Estructuras de datos (identico a GoTaskTracker)
├── storage/
│   └── storage.go       # Persistencia JSON (identico a GoTaskTracker)
└── tasks/
    └── tasks.go         # Operaciones CRUD (identico a GoTaskTracker)
```

**Dependencias externas:**

| Paquete                                | Version | Uso                                |
| -------------------------------------- | ------- | ---------------------------------- |
| `github.com/spf13/cobra`               | v1.10.2 | Framework CLI                      |
| `github.com/spf13/pflag`               | v1.0.9  | Gestion de flags (indirecto)       |
| `github.com/inconshreveable/mousetrap` | v1.1.0  | Compatibilidad Windows (indirecto) |

**Diferencias frente a GoTaskTracker:**

| Aspecto                  | GoTaskTracker       | GoTaskTracker COBRA                         |
| ------------------------ | ------------------- | ------------------------------------------- |
| Parseo de comandos       | Switch-case manual  | `cobra.Command`                             |
| Validacion de argumentos | Manual en cada caso | `ExactArgs`, `MinimumNArgs`, `MaximumNArgs` |
| Ayuda autogenerada       | No                  | Si (`--help`)                               |
| Dependencias externas    | Ninguna             | Cobra v1.10.2                               |

### Comandos disponibles

Los comandos son los mismos que en GoTaskTracker. Cobra agrega ademas soporte para ayuda integrada:

```bash
# Ver ayuda del comando raiz
./task-cli --help

# Ver ayuda de un subcomando especifico
./task-cli add --help
./task-cli list --help
./task-cli update --help
```

| Comando    | Argumentos                      | Validacion      |
| ---------- | ------------------------------- | --------------- |
| `register` | `<usuario> <password>`          | ExactArgs(2)    |
| `login`    | `<usuario> <password>`          | ExactArgs(2)    |
| `logout`   | (ninguno)                       | NoArgs          |
| `add`      | `<titulo...>`                   | MinimumNArgs(1) |
| `list`     | `[todo\|done\|inprogress]`      | MaximumNArgs(1) |
| `delete`   | `<id>`                          | ExactArgs(1)    |
| `update`   | `<id> <todo\|done\|inprogress>` | ExactArgs(2)    |

**Ejemplos de uso:**

```bash
# Registrar un usuario
./task-cli register juan password123

# Iniciar sesion
./task-cli login juan password123

# Agregar tareas
./task-cli add "Estudiar para café café"
./task-cli add "Estudiar para argenis"

# Listar tareas
./task-cli list
./task-cli list todo
./task-cli list inprogress
./task-cli list done

# Cambiar estado de una tarea
./task-cli update 1 inprogress
./task-cli update 1 done

# Eliminar una tarea
./task-cli delete 2

# Cerrar sesion
./task-cli logout
```

### Almacenamiento de datos

Identico a GoTaskTracker. Los archivos `usuarios.json` y `sesion.json` se crean en el directorio de ejecucion del binario.

---

## RustTaskTracker

### Descripcion

Implementacion en Rust del gestor de tareas con arquitectura por capas. Se separa la lógica del parseo de los comandos (cli) de la lógica de negocio (servicios), además se abstrae la lógica de persistencia (repositorios) mediante Traits, para facilitar una posible migración de json a una base de datos. Utiliza Argon2 para el hashing de contrasenas y un mecanismo de tokens para la validacion de sesion.

### Requisitos previos

- Rust 1.85 o superior (edition 2024)
- Cargo (incluido con Rust)

Verificar la version instalada:

```bash
rustc --version
cargo --version
```

Para instalar Rust: [https://rustup.rs](https://rustup.rs)

### Instalacion rapida

```bash
# Ubicarse en el directorio del proyecto
cd RustTaskTracker

# Compilar en modo release
cargo build --release

# El binario queda disponible en:
./target/release/task-cli <comando> [argumentos]
```

### Instalacion como comando global

```bash
cd RustTaskTracker

# Instala el binario en ~/.cargo/bin/ (debe estar en el PATH)
cargo install --path .

# Verificar instalacion
task-cli --help
```

Para que el comando este disponible en cualquier directorio, asegurarse de que `~/.cargo/bin` este en la variable de entorno `PATH`.

### Arquitectura

El proyecto sigue una arquitectura en capas con inversion de dependencias:

```
RustTaskTracker/
├── Cargo.toml                    # Configuracion del paquete y dependencias
├── src/
│   ├── main.rs                   # Punto de entrada. Inyeccion de dependencias
│   ├── cli.rs                    # Parseo de argumentos y despacho de comandos
│   ├── errors.rs                 # Enum AppError con thiserror
│   ├── entities/
│   │   ├── task.rs               # Entidad Task con enum TaskStatus
│   │   ├── user.rs               # Entidad User
│   │   └── session.rs            # Entidad Session con token hash
│   ├── traits/
│   │   ├── Itask_repository.rs   # Contrato de persistencia de tareas
│   │   ├── Iuser_repository.rs   # Contrato de persistencia de usuarios
│   │   └── Isession_repository.rs# Contrato de persistencia de sesion
│   ├── repositories/
│   │   ├── json_task_repository.rs    # Implementacion JSON del repositorio de tareas
│   │   ├── json_user_repository.rs    # Implementacion JSON del repositorio de usuarios
│   │   └── json_session_repository.rs # Implementacion JSON del repositorio de sesion
│   └── services/
│       ├── task_service.rs       # Logica de negocio de tareas
│       └── user_service.rs       # Logica de negocio de usuarios y sesion
```

**Diagrama de capas:**

```
CLI (cli.rs)
  |
  v
Services (task_service, user_service)
  |
  v
Traits (Itask_repository, Iuser_repository, Isession_repository)
  |
  v
Repositories (json_task_repository, json_user_repository, json_session_repository)
  |
  v
Archivos JSON (~/.task-cli/)
```

**Dependencias:**

| Crate        | Version | Uso                                   |
| ------------ | ------- | ------------------------------------- |
| `serde`      | 1.0.228 | Serializacion y deserializacion       |
| `serde_json` | 1.0.149 | Soporte JSON                          |
| `argon2`     | 0.5.3   | Hashing de contrasenas                |
| `rand`       | 0.8     | Generacion de salt aleatorio          |
| `chrono`     | 0.4.44  | Manejo de fechas y horas              |
| `thiserror`  | 1.0     | Macros para tipos de error            |
| `dirs`       | 6.0.0   | Acceso al directorio home del usuario |

### Comandos disponibles

| Comando            | Argumentos                  | Descripcion                                  |
| ------------------ | --------------------------- | -------------------------------------------- |
| `register`         | `<usuario> <password>`      | Registrar un nuevo usuario                   |
| `login`            | `<usuario> <password>`      | Iniciar sesion                               |
| `logout`           | (ninguno)                   | Cerrar la sesion activa                      |
| `add`              | `'<descripcion>'`           | Agregar una nueva tarea                      |
| `list`             | `[todo\|in-progress\|done]` | Listar todas las tareas o filtrar por estado |
| `delete`           | `<id>`                      | Eliminar una tarea por su ID                 |
| `update`           | `<id> '<descripcion>'`      | Actualizar la descripcion de una tarea       |
| `mark-in-progress` | `<id>`                      | Marcar una tarea como en progreso            |
| `mark-done`        | `<id>`                      | Marcar una tarea como completada             |

**Estados de tarea:**

| Valor CLI     | Descripcion       |
| ------------- | ----------------- |
| `todo`        | Tarea pendiente   |
| `in-progress` | Tarea en progreso |
| `done`        | Tarea completada  |

**Ejemplos de uso:**

```bash
# Registrar un usuario (minimo 3 caracteres de usuario, 6 de password)
task-cli register juan password123

# Iniciar sesion
task-cli login juan password123

# Agregar tareas
task-cli add 'Revisar el informe final'
task-cli add 'Preparar presentacion del proyecto'

# Listar tareas
task-cli list
task-cli list todo
task-cli list in-progress
task-cli list done

# Cambiar estado de una tarea
task-cli mark-in-progress 1
task-cli mark-done 1

# Actualizar la descripcion de una tarea
task-cli update 2 'Nueva descripcion de la tarea'

# Eliminar una tarea
task-cli delete 2

# Cerrar sesion
task-cli logout
```

**Validaciones de entrada:**

- Usuario: minimo 3 caracteres
- Contrasena: minimo 6 caracteres

### Almacenamiento de datos

Los archivos se almacenan en el directorio `~/.task-cli/` dentro del directorio home del usuario del sistema operativo:

| Archivo                | Descripcion                                          |
| ---------------------- | ---------------------------------------------------- |
| `users.json`           | Lista de usuarios con contrasenas hasheadas (Argon2) |
| `session.json`         | ID de usuario y hash del token de sesion activa      |
| `tasks_<user_id>.json` | Tareas del usuario con el ID correspondiente         |

Estructura de `users.json`:

```json
[
  {
    "id": 1,
    "username": "juan",
    "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$..."
  }
]
```

Estructura de `tasks_1.json`:

```json
[
  {
    "id": 1,
    "description": "Revisar el informe final",
    "status": "Todo",
    "created_at": "2026-03-27 10:30:45",
    "updated_at": "2026-03-27 10:30:45"
  }
]
```

---

## Autores

| Nombre          | CI         |
| --------------- | ---------- |
| Abisaac Carmona | 32.218.469 |
| Jose Puerta     | 31.904.115 |

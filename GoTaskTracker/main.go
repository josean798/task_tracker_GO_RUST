package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"task-cli/auth"
	"task-cli/storage"
	"task-cli/tasks"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Uso: task-cli <comando> [argumentos]")
		return
	}

	users := storage.LoadUsers()
	
	currentUser := auth.GetLoggedUser(users)
	
	command := os.Args[1]

	switch command {
	case "register":
		if len(os.Args) < 4 {
			fmt.Println("Uso: task-cli register <usuario> <password>")
			return
		}
		auth.Register(os.Args[2], os.Args[3], users)

	case "login":
		if len(os.Args) < 4 {
			fmt.Println("Uso: task-cli login <usuario> <password>")
			return
		}
		auth.Login(os.Args[2], os.Args[3], users)

	case "logout":
		auth.Logout()

	case "add":
		if currentUser == nil {
			fmt.Println("Debes iniciar sesión primero.")
			return
		}
		title := strings.Join(os.Args[2:], " ")
		tasks.AddTask(title, users, currentUser.Username)

	case "list":
		if currentUser == nil {
			fmt.Println("Debes iniciar sesión primero.")
			return
		}

		filter := ""
		if len(os.Args) >= 3 {
			filter = strings.ToLower(os.Args[2])
			if filter != "todo" && filter != "done" && filter != "inprogress" {
				fmt.Println("Uso: task-cli list [todo|done|inprogress]")
				return
			}
		}

		tasks.ListTasks(users, currentUser.Username, filter)

	case "delete":
		if currentUser == nil || len(os.Args) < 3 {
			fmt.Println("Uso: task-cli delete <id>")
			return
		}
		id, _ := strconv.Atoi(os.Args[2])
		tasks.DeleteTask(id, users, currentUser.Username)

	case "update":
		if currentUser == nil || len(os.Args) < 4 {
			fmt.Println("Uso: task-cli update <id> <estado>")
			return
		}
		id, err := strconv.Atoi(os.Args[2])
		if err != nil {
			fmt.Println("Error: El ID debe ser numérico.")
			return
		}

		status := strings.ToLower(os.Args[3])
		if status != "todo" && status != "done" && status != "inprogress" {
			fmt.Println("Uso: task-cli update <id> <todo|done|inprogress>")
			return
		}

		tasks.UpdateTask(id, status, users, currentUser.Username)

	default:
		fmt.Printf("Comando desconocido: %s\n", command)
	}
}
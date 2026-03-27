package tasks

import (
	"fmt"
	"task-cli/models"
	"task-cli/storage"
	"time"
)

func AddTask(title string, users []models.User, username string) {
	for i := range users {
		if users[i].Username == username {
			maxID := 0
			for _, task := range users[i].Tasks {
				if task.ID > maxID {
					maxID = task.ID
				}
			}

			newTask := models.Task{
				ID:     maxID + 1,
				Title:  title,
				Status: "pending",
				CreatedAt: time.Now().Format("2006-01-02 15:04:05"),
			}
			users[i].Tasks = append(users[i].Tasks, newTask)
			storage.SaveData(users)
			fmt.Printf("Tarea '%s' añadida para el usuario '%s'.\n", title, username)
			return
		}
	}
}

func ListTasks(users []models.User, username string, statusFilter string) {
	for _, user := range users {
		if user.Username == username {
			if len(user.Tasks) == 0 {
				fmt.Printf("El usuario '%s' no tiene tareas.\n", username)
				return
			}

			actualFilter := statusFilter
			if statusFilter == "todo" {
				actualFilter = "pending"
			}

			found := false
			for _, task := range user.Tasks {
				// Mostramos la tarea si no hay filtro, o si coincide con el filtro pedido
				if actualFilter == "" || task.Status == actualFilter {
					if !found {
						if statusFilter == "" {
							fmt.Printf("Todas las tareas de '%s':\n", username)
						} else {
							fmt.Printf("Tareas de '%s' con estado '%s':\n", username, statusFilter)
						}
						found = true
					}
					fmt.Printf("- [%s] ID: %d | %s | Creada: %s\n", task.Status, task.ID, task.Title, task.CreatedAt)
				}
			}

			if !found {
				fmt.Printf("No se encontraron tareas con el estado: %s\n", statusFilter)
			}
			return
		}
	}
}

func DeleteTask(taskID int, users []models.User, username string) {
	for i, user := range users {
		if user.Username == username {
			for j, task := range user.Tasks {
				if task.ID == taskID {
					users[i].Tasks = append(users[i].Tasks[:j], users[i].Tasks[j+1:]...)
					storage.SaveData(users)
					fmt.Printf("Tarea con ID %d eliminada.\n", taskID)
					return
				}
			}
			fmt.Printf("No se encontró la tarea con ID %d.\n", taskID)
			return
		}
	}
}

func UpdateTask(taskID int, newStatus string, users []models.User, username string) {
	actualStatus := newStatus
	if newStatus == "todo" {
		actualStatus = "pending"
	}

	if actualStatus != "pending" && actualStatus != "done" && actualStatus != "inprogress" {
		fmt.Println("Estado inválido. Estados permitidos: todo, done, inprogress.")
		return
	}

	for i, user := range users {
		if user.Username == username {
			for j, task := range user.Tasks {
				if task.ID == taskID {
					users[i].Tasks[j].Status = actualStatus
					storage.SaveData(users)
					fmt.Printf("Tarea %d actualizada a estado: '%s'.\n", taskID, newStatus)
					return
				}
			}
			fmt.Printf("No se encontró la tarea con ID %d.\n", taskID)
			return
		}
	}
}
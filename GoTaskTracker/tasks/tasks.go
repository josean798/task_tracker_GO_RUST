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

func ListTasks(users []models.User, username string) {
	for _, user := range users {
		if user.Username == username {
			if len(user.Tasks) == 0 {
				fmt.Printf("El usuario '%s' no tiene tareas.\n", username)
				return
			}
			fmt.Printf("Tareas de '%s':\n", username)
			for _, task := range user.Tasks {
				fmt.Printf("- [%s] ID: %d | %s | Creada: %s\n", task.Status, task.ID, task.Title, task.CreatedAt)
			}
			return
		}
	}
}

// DeleteTask elimina una tarea por su ID
func DeleteTask(taskID int, users []models.User, username string) {
	for i, user := range users {
		if user.Username == username {
			for j, task := range user.Tasks {
				if task.ID == taskID {
					// Eliminamos la tarea usando append y slicing
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
	for i, user := range users {
		if user.Username == username {
			for j, task := range user.Tasks {
				if task.ID == taskID {
					users[i].Tasks[j].Status = newStatus
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
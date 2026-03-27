package auth

import (
	"encoding/json"
	"fmt"
	"os"
	"task-cli/models"
	"task-cli/storage"
)

const sessionFile = "sesion.json"

func GetLoggedUser(users []models.User) *models.User {
	sesionData, err := os.ReadFile(sessionFile)
	if err != nil {
		return nil
	}

	var loggedUser models.User
	err = json.Unmarshal(sesionData, &loggedUser)
	if err != nil {
		return nil
	}

	for i := range users {
		if users[i].Username == loggedUser.Username {
			return &users[i]
		}
	}
	return nil
}

// Login crea el archivo de sesión si las credenciales son correctas
func Login(username, password string, users []models.User) {
	for _, user := range users {
		if user.Username == username && user.Password == password {
			sesionData, err := json.Marshal(user)
			if err != nil {
				fmt.Println("Error al crear la sesión:", err)
				return
			}
			err = os.WriteFile(sessionFile, sesionData, 0644)
			if err != nil {
				fmt.Println("Error al escribir archivo de sesión:", err)
				return
			}
			fmt.Printf("Usuario '%s' ha iniciado sesión correctamente.\n", username)
			return
		}
	}
	fmt.Println("Usuario o contraseña incorrectos.")
}

func Logout() {
	err := os.Remove(sessionFile)
	if err != nil {
		if os.IsNotExist(err) {
			fmt.Println("No hay ninguna sesión activa para cerrar.")
			return
		}
		fmt.Println("Error al cerrar sesión:", err)
		return
	}
	fmt.Println("Sesión cerrada con éxito.")
}

func Register(username, password string, users []models.User) {
	for _, user := range users {
		if user.Username == username {
			fmt.Printf("El nombre de usuario '%s' ya está en uso.\n", username)
			return
		}
	}

	newUser := models.User{
		ID:       len(users) + 1,
		Username: username,
		Password: password,
		Tasks:    []models.Task{},
	}

	users = append(users, newUser)
	storage.SaveData(users)
	fmt.Printf("Usuario '%s' registrado exitosamente.\n", username)
}
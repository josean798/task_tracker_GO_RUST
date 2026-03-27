package storage

import (
	"encoding/json"
	"fmt"
	"os"
	"task-cli/models"
)
const fileName = "usuarios.json"

func LoadUsers() []models.User {
	var users []models.User

	data, err := os.ReadFile(fileName)
	if err != nil {
		if os.IsNotExist(err) {
			return []models.User{}
		}
		fmt.Printf("Error al leer %s: %v\n", fileName, err)
		return []models.User{}
	}

	if len(data) == 0 {
		return []models.User{}
	}

	err = json.Unmarshal(data, &users)
	if err != nil {
		fmt.Printf("Error al procesar el JSON: %v\n", err)
		return []models.User{}
	}

	return users
}

func SaveData(users []models.User) {
	content, err := json.MarshalIndent(users, "", "  ")
	if err != nil {
		fmt.Printf("Error al convertir datos a JSON: %v\n", err)
		return
	}

	err = os.WriteFile(fileName, content, 0644)
	if err != nil {
		fmt.Printf("Error al escribir en %s: %v\n", fileName, err)
	}
}
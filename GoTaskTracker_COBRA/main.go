package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"task-cli/auth"
	"task-cli/storage"
	"task-cli/tasks"

	"github.com/spf13/cobra"
)

func main() {
	users := storage.LoadUsers()
	currentUser := auth.GetLoggedUser(users)

	var rootCmd = &cobra.Command{
		Use:   "task-cli",
		Short: "Una CLI para gestionar tus tareas",
	}

	var registerCmd = &cobra.Command{
		Use:   "register [usuario] [password]",
		Short: "Registrar un nuevo usuario",
		Args:  cobra.ExactArgs(2),
		Run: func(cmd *cobra.Command, args []string) {
			auth.Register(args[0], args[1], users)
		},
	}

	var loginCmd = &cobra.Command{
		Use:   "login [usuario] [password]",
		Short: "Iniciar sesión",
		Args:  cobra.ExactArgs(2),
		Run: func(cmd *cobra.Command, args []string) {
			auth.Login(args[0], args[1], users)
		},
	}

	var logoutCmd = &cobra.Command{
		Use:   "logout",
		Short: "Cerrar sesión",
		Args:  cobra.NoArgs,
		Run: func(cmd *cobra.Command, args []string) {
			auth.Logout()
		},
	}

	var addCmd = &cobra.Command{
		Use:   "add [título...]",
		Short: "Añadir una nueva tarea",
		Args:  cobra.MinimumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			if currentUser == nil {
				fmt.Println("Debes iniciar sesión primero.")
				return
			}
			title := strings.Join(args, " ")
			tasks.AddTask(title, users, currentUser.Username)
		},
	}

	var listCmd = &cobra.Command{
		Use:   "list [estado opcional]",
		Short: "Listar tareas (ej. task-cli list, task-cli list done)",
		Args:  cobra.MaximumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			if currentUser == nil {
				fmt.Println("Debes iniciar sesión primero.")
				return
			}
			
			filter := ""
			if len(args) > 0 {
				filter = args[0]
			}
			
			tasks.ListTasks(users, currentUser.Username, filter)
		},
	}

	var deleteCmd = &cobra.Command{
		Use:   "delete [id]",
		Short: "Eliminar una tarea por su ID",
		Args:  cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			if currentUser == nil {
				fmt.Println("Debes iniciar sesión primero.")
				return
			}
			id, err := strconv.Atoi(args[0])
			if err != nil {
				fmt.Println("Error: El ID debe ser numérico.")
				return
			}
			tasks.DeleteTask(id, users, currentUser.Username)
		},
	}

	var updateCmd = &cobra.Command{
		Use:   "update [id] [estado]",
		Short: "Actualizar el estado de una tarea (ej. done, in-progress)",
		Args:  cobra.ExactArgs(2),
		Run: func(cmd *cobra.Command, args []string) {
			if currentUser == nil {
				fmt.Println("Debes iniciar sesión primero.")
				return
			}
			id, err := strconv.Atoi(args[0])
			if err != nil {
				fmt.Println("Error: El ID debe ser numérico.")
				return
			}
			tasks.UpdateTask(id, args[1], users, currentUser.Username)
		},
	}

	// Agregamos todos los subcomandos al comando raíz
	rootCmd.AddCommand(registerCmd, loginCmd, logoutCmd, addCmd, listCmd, deleteCmd, updateCmd)

	// Ejecutamos Cobra
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
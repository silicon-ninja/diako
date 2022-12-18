//  Created by silicon-ninja on 13/11/2022
//  Copyright © 2022 Srikanth K. All rights reserved.

package diako

import (
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initializes Diako",
	Args:  cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		println("")
		color.Magenta("Welcome to Diako!")
		color.Magenta("Diako is a simple, yet powerful, and easy to use, command line tool for managing your git.")
		println("")
		color.Cyan("Let's get started! 🚀")
		println("")
		color.Cyan("Diako is a tool that helps you manage your git repositories.")
		color.Cyan("It helps you to create, clone, and manage your git repositories.")
		color.Cyan("It also helps you to manage your git issues.")
		println("")
		// 		prompt := promptui.Select{
		// 			Templates: &promptui.SelectTemplates{
		// 				Label:    "{{ . }}?",
		// 				Active:   "\U0001F449 {{ .Name | cyan }} ({{ .Description | red }})",
		// 				Inactive: "  {{ .Name | cyan }} ({{ .Description | red }})",
		// 				Selected: "\U0001F449 {{ .Name | red | cyan }}",
		// 				Details: `
		// --------- Git ----------
		// {{ "Name:" | faint }}	{{ .Name }}
		// {{ "Description:" | faint }}	{{ .Description }}
		// {{ "URL:" | faint }}	{{ .URL }}
		// {{ "SSH:" | faint }}	{{ .SSH }}
		// 				`,
		// 			},

		// 			Label:    "Please select an option below:",
		// 			HideHelp: true,

		// 			Items: []string{"Ready to Takeoff 🚀 ", "Exit"},
		// 		}
		// _, result, err := prompt.Run()
		// if err != nil {
		// 	color.Red("\nMission Failed ❌ - Exiting")
		// 	return
		// }

		// if result == "Ready to Takeoff 🚀 " {
		// 	color.Green("\nMission Successful ✅ - Takeoff")
		// }

	},
}

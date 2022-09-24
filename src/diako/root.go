/*
Copyright © 2022 Srikanth Kandarpa <srikanthkandarp@gmail.com>
*/
package diako

import (
	"os"

	"github.com/spf13/cobra"
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "diako",
	Short: "Diako is a simple, yet powerful, and easy to use, command line tool for managing your git.",
	Long: `

█▀▄ █ ▄▀█ █▄▀ █▀█ .______________________________________________________|_._._._._._._._._._.
█▄▀ █ █▀█ █░█ █▄█  \_____________________________________________________|_#_#_#_#_#_#_#_#_#_|
																		    l
Diako is a simple, yet powerful, and easy to use, command line tool for managing your git.

Built with ❤️  by Srikanth Kandarp from India.
`,
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	rootCmd.AddCommand(issuesCmd, initCmd)
}

// Created by silicon-ninja on 13/11/2022
// Copyright © 2022 Srikanth K. All rights reserved.

package diako

import (
	"fmt"

	"github.com/spf13/cobra"
)

var issuesCmd = &cobra.Command{
	Use:   "issues",
	Short: "Opens a ticket on GitHub 🐱",
	Args:  cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("issues called")
	},
}

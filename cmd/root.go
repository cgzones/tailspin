package cmd

import (
	"os"

	"github.com/logrusorgru/aurora/v3"
	"github.com/spf13/cobra"

	"spin/app"
	"spin/conf"
	"spin/file"
	"spin/mapper"
	"spin/styling"
)

var follow bool

func Root() *cobra.Command {
	rootCmd := &cobra.Command{
		Use:     "spin {file}",
		Short:   "\n" + aurora.Magenta("tailspin").Italic().String() + " is a log file highlighter",
		Example: "spin system.log -f",
		Version: app.Version,
		Args:    cobra.MinimumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			config := getConfig()
			theme := styling.GetTheme()
			scheme := mapper.MapTheme(theme)

			file.Setup(config, os.Args[1], scheme)
		},
	}

	rootCmd.CompletionOptions.DisableDefaultCmd = true

	rootCmd.AddCommand(versionCmd())
	rootCmd.AddCommand(debugCmd())

	configureFlags(rootCmd)

	return rootCmd
}

func configureFlags(rootCmd *cobra.Command) {
	rootCmd.PersistentFlags().BoolVarP(&follow, "follow", "f", false,
		"scroll forward, and keep trying to read when the end of file is reached\n"+
			"(Similar to "+aurora.Magenta("tail -f").Italic().String()+")")
}

func getConfig() *conf.Config {
	config := conf.New()

	config.Follow = follow

	if debugFile != 0 {
		config.DebugMode = true
		config.DebugFile = debugFile
	}

	return config
}

package main

import (
	"errors"
	"flag"
	"fmt"
	"os"
	"strings"

	"github.com/kako-jun/gitp/gitp-core"
)

// app version
var Version string = "1.0.0"

func parseArgs() (gitpCommand string, allRepo bool, repo string, gitCommandAndArgs []string, err error) {
	var (
		versionFlag bool
		allFlag     bool
	)

	flag.BoolVar(&versionFlag, "version", false, "print version number")
	flag.BoolVar(&allFlag, "all", false, "target all repositories")
	flag.BoolVar(&allFlag, "a", false, "target all repositories")

	flag.Parse()
	args := flag.Args()
	// fmt.Println(args)
	// fmt.Println(*all)

	if versionFlag {
		fmt.Println(Version)
		os.Exit(0)
	}

	allRepo = allFlag
	if allRepo {
		gitCommandAndArgs = args

		// fmt.Println(gitpCommand)
		// fmt.Println(allRepo)
		// fmt.Println(repo)
		// fmt.Println(gitCommandAndArgs)
		return
	}

	if flag.NArg() >= 1 {
		commandOrRepo := args[0]
		switch commandOrRepo {
		case "init", "clone", "remote", "config", "pull", "push":
			gitpCommand = commandOrRepo
		default:
			repo = commandOrRepo
		}
	}

	if repo != "" {
		if flag.NArg() >= 2 {
			gitCommandAndArgs = args[1:]

			repo = strings.Replace(repo, "/", "", -1)
			repo = strings.Replace(repo, "\\", "", -1)

			// fmt.Println(gitpCommand)
			// fmt.Println(allRepo)
			// fmt.Println(repo)
			// fmt.Println(gitCommandAndArgs)
			return
		}
	}

	if gitpCommand != "" {
		if flag.NArg() >= 2 {
			switch gitpCommand {
			case "remote":
				if args[1] == "add" {
					gitpSubCommand := args[1]
					gitpCommand = gitpCommand + " " + gitpSubCommand

					if flag.NArg() >= 3 {
						repo = args[2]
					} else {
						allRepo = true
					}
				}
			case "config":
				if args[1] == "user" {
					gitpSubCommand := args[1]
					gitpCommand = gitpCommand + " " + gitpSubCommand

					if flag.NArg() >= 3 {
						repo = args[2]
					} else {
						allRepo = true
					}
				}
			default:
				repo = args[1]
			}
		} else {
			allRepo = true
		}
	}

	repo = strings.Replace(repo, "/", "", -1)
	repo = strings.Replace(repo, "\\", "", -1)

	// fmt.Println(gitpCommand)
	// fmt.Println(allRepo)
	// fmt.Println(repo)
	// fmt.Println(gitCommandAndArgs)

	// check error.
	if gitpCommand == "" && !allRepo {
		err = errors.New("invalid argument")
		return
	}

	if gitpCommand == "" && allRepo && len(gitCommandAndArgs) == 0 {
		err = errors.New("invalid argument")
		return
	}

	if gitpCommand == "" && repo == "" {
		err = errors.New("invalid argument")
		return
	}

	if gitpCommand == "" && repo != "" && len(gitCommandAndArgs) == 0 {
		err = errors.New("invalid argument")
		return
	}

	switch gitpCommand {
	case "", "init", "clone", "remote add", "config user", "pull", "push":
	default:
		err = errors.New("invalid argument")
		return
	}

	return
}

// entry point
func main() {
	gitpCommand, allRepo, repo, gitCommandAndArgs, err := parseArgs()
	if err != nil {
		fmt.Println("error:", err)
		fmt.Println("usage:")
		fmt.Println("  gitp init")
		fmt.Println("  gitp clone")
		fmt.Println("  gitp remote add")
		fmt.Println("  gitp config user")
		fmt.Println("  gitp pull")
		fmt.Println("  gitp push")
		fmt.Println("")
		fmt.Println("  gitp clone [repository name]")
		fmt.Println("  gitp remote add [repository name]")
		fmt.Println("  gitp config user [repository name]")
		fmt.Println("  gitp pull [repository name]")
		fmt.Println("  gitp push [repository name]")
		fmt.Println("")
		fmt.Println("  gitp -a [every git command]")
		fmt.Println("    e.g.  gitp -a checkout .")
		fmt.Println("  gitp [repository name] [every git command]")
		fmt.Println("    e.g.  gitp [repository name] checkout .")
		return
	}

	gitp.Exec(gitpCommand, allRepo, repo, gitCommandAndArgs...)
}

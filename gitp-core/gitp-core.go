// Package gitp is ***
package gitp

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strings"

	"github.com/kako-jun/cdand/cdand-core"
)

// Repo for json
type Repo struct {
	Name    string `json:"name"`
	Remotes struct {
		Origin struct {
			SSH   string `json:"ssh"`
			HTTPS string `json:"https"`
		} `json:"origin"`
		Second struct {
			SSH   string `json:"ssh"`
			HTTPS string `json:"https"`
		} `json:"second"`
	} `json:"remotes"`
	Enabled bool `json:"enabled"`
}

// User for json
type User struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

// Config for json
type Config struct {
	Repos    []Repo `json:"repos"`
	Comments struct {
		Default string `json:"default"`
	} `json:"comments"`
	User User `json:"user"`
}

// GitP is ***
type GitP struct{}

func (gitp GitP) exists(path string) bool {
	if _, err := os.Stat(path); err != nil {
		return false
	}

	return true
}

func (gitp GitP) saw() {
	fmt.Println(strings.Repeat("-", 42))
}

func (gitp GitP) getConfigRepo(repo string, configRepos []Repo) (configRepoReturn Repo, result bool) {
	for _, configRepo := range configRepos {
		if configRepo.Enabled {
			if configRepo.Name == repo {
				configRepoReturn = configRepo
				result = true
			}
		}
	}

	return
}

func (gitp GitP) init(configFilePath string) (err error) {
	if gitp.exists(configFilePath) {
		err = errors.New(configFilePath + " already exists")
		return
	}

	var config Config
	var repo1 Repo
	config.Repos = append(config.Repos, repo1)
	var repo2 Repo
	config.Repos = append(config.Repos, repo2)

	jsonBytes, err := json.Marshal(&config)
	if err != nil {
		log.Fatal(err)
	}

	jsonBuffer := new(bytes.Buffer)
	err = json.Indent(jsonBuffer, jsonBytes, "", "  ")
	if err != nil {
		log.Fatal(err)
	}

	err = ioutil.WriteFile(configFilePath, jsonBuffer.Bytes(), os.ModePerm)
	if err != nil {
		log.Fatal(err)
	}

	return
}

func (gitp GitP) clone(repo Repo) (err error) {
	if gitp.exists(repo.Name) {
		fmt.Println(repo.Name + " already exists")
		// err = errors.New(repo.Name + " already exists")
		return
	}

	if repo.Remotes.Origin.SSH != "" {
		fmt.Println("git", "clone", repo.Remotes.Origin.SSH)
		err = cdand.Exec(".", "git", "clone", repo.Remotes.Origin.SSH)
	} else if repo.Remotes.Origin.HTTPS != "" {
		fmt.Println("git", "clone", repo.Remotes.Origin.HTTPS)
		err = cdand.Exec(".", "git", "clone", repo.Remotes.Origin.HTTPS)
	}

	return
}

func (gitp GitP) addRemote(repo Repo) (err error) {
	if !gitp.exists(repo.Name) {
		err = errors.New(repo.Name + " not found")
		return
	}

	if repo.Remotes.Second.SSH != "" {
		fmt.Println("git", "remote", "add", "second", repo.Remotes.Second.SSH)
		err = cdand.Exec(repo.Name, "git", "remote", "add", "second", repo.Remotes.Second.SSH)
	} else if repo.Remotes.Second.HTTPS != "" {
		fmt.Println("git", "remote", "add", "second", repo.Remotes.Second.HTTPS)
		err = cdand.Exec(repo.Name, "git", "remote", "add", "second", repo.Remotes.Second.HTTPS)
	}

	return
}

func (gitp GitP) configUser(repo Repo, user User) (err error) {
	if !gitp.exists(repo.Name) {
		err = errors.New(repo.Name + " not found")
		return
	}

	if user.Name != "" {
		fmt.Println("git", "config", "user.name", user.Name)
		err = cdand.Exec(repo.Name, "git", "config", "user.name", user.Name)
	}

	if user.Email != "" {
		fmt.Println("git", "config", "user.email", user.Email)
		err = cdand.Exec(repo.Name, "git", "config", "user.email", user.Email)
	}

	return
}

func (gitp GitP) pull(repo Repo) (err error) {
	if !gitp.exists(repo.Name) {
		err = errors.New(repo.Name + " not found")
		return
	}

	fmt.Println("git", "pull", "origin", "master")
	err = cdand.Exec(repo.Name, "git", "pull", "origin", "master")

	if repo.Remotes.Second.SSH != "" || repo.Remotes.Second.HTTPS != "" {
		fmt.Println("")
		fmt.Println("git", "pull", "second", "master")
		err = cdand.Exec(repo.Name, "git", "pull", "second", "master")
	}

	return
}

func (gitp GitP) push(repo Repo, defaultComment string) (err error) {
	if !gitp.exists(repo.Name) {
		err = errors.New(repo.Name + " not found")
		return
	}

	fmt.Println("git", "add", "-A")
	err = cdand.Exec(repo.Name, "git", "add", "-A")

	fmt.Println("git", "commit", "-m", defaultComment)
	err = cdand.Exec(repo.Name, "git", "commit", "-m", defaultComment)

	fmt.Println("git", "push", "origin", "master")
	err = cdand.Exec(repo.Name, "git", "push", "origin", "master")

	if repo.Remotes.Second.SSH != "" || repo.Remotes.Second.HTTPS != "" {
		fmt.Println("")
		fmt.Println("git", "push", "second", "master")
		err = cdand.Exec(repo.Name, "git", "push", "second", "master")
	}

	return
}

func (gitp GitP) gitCommand(repo string, gitCommandAndArgs ...string) (err error) {
	if !gitp.exists(repo) {
		err = errors.New(repo + " not found")
		return
	}

	fmt.Println("git", strings.Join(gitCommandAndArgs, " "))
	err = cdand.Exec(repo, "git", gitCommandAndArgs...)
	return
}

func (gitp GitP) start(gitpCommand string, allRepo bool, repo string, gitCommandAndArgs ...string) (err error) {
	configFilePath := "./gitp_config.json"

	if gitpCommand == "init" {
		// gitp init
		err = gitp.init(configFilePath)
		if err != nil {
			log.Fatal(err)
		}

		return
	}

	jsonBytes, err := ioutil.ReadFile(configFilePath)
	if err != nil {
		log.Fatal(err)
	}

	var config Config
	if err := json.Unmarshal(jsonBytes, &config); err != nil {
		log.Fatal(err)
	}

	if allRepo {
		if gitpCommand != "" {
			// gitp clone
			for _, configRepo := range config.Repos {
				err = gitp.start(gitpCommand, false, configRepo.Name, gitCommandAndArgs...)
				if err != nil {
					return
				}
			}
		} else {
			// gitp -a [any git command]
			for _, configRepo := range config.Repos {
				if configRepo.Enabled {
					gitp.saw()
					fmt.Println("[" + configRepo.Name + "]")
					fmt.Println("")

					err = gitp.gitCommand(configRepo.Name, gitCommandAndArgs...)
				}
			}
		}
	} else {
		if gitpCommand != "" {
			// gitp clone [repository name]
			configRepo, result := gitp.getConfigRepo(repo, config.Repos)
			if result {
				gitp.saw()
				fmt.Println("[" + configRepo.Name + "]")
				fmt.Println("")

				switch gitpCommand {
				case "clone":
					err = gitp.clone(configRepo)
				case "remote add":
					err = gitp.addRemote(configRepo)
				case "config user":
					err = gitp.configUser(configRepo, config.User)
				case "pull":
					err = gitp.pull(configRepo)
				case "push":
					err = gitp.push(configRepo, config.Comments.Default)
				default:
					log.Fatal(err)
				}
			}
		} else {
			// gitp [repository name] [any git command]
			gitp.saw()
			fmt.Println("[" + repo + "]")
			fmt.Println("")

			err = gitp.gitCommand(repo, gitCommandAndArgs...)
		}
	}

	return
}

// Exec is ***
func Exec(gitpCommand string, allRepo bool, repo string, gitCommandAndArgs ...string) (errReturn error) {
	gitp := new(GitP)
	if err := gitp.start(gitpCommand, allRepo, repo, gitCommandAndArgs...); err != nil {
		fmt.Println("error:", err)
		errReturn = errors.New("error")
		return
	}

	return
}

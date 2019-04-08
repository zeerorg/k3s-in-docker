package main

import (
	"log"
	"os"

	"github.com/iwilltry42/k3d-go/cli"
	"github.com/iwilltry42/k3d-go/version"
	"github.com/urfave/cli"
)

// main represents the CLI application
func main() {

	// App Details
	app := cli.NewApp()
	app.Name = "k3d"
	app.Usage = "Run k3s in Docker!"
	app.Version = version.GetVersion()
	app.Authors = []cli.Author{
		cli.Author{
			Name:  "iwilltry42",
			Email: "iwilltry42@gmail.com",
		},
		cli.Author{
			Name:  "Rishabh Gupta(zeerorg)",
			Email: "r.g.gupta@outlook.com",
		},
	}

	// commands that you can execute
	app.Commands = []cli.Command{
		{
			// check-tools verifies that docker is up and running
			Name:    "check-tools",
			Aliases: []string{"ct"},
			Usage:   "Check if docker is running",
			Action:  run.CheckTools,
		},
		{
			// create creates a new k3s cluster in a container
			Name:    "create",
			Aliases: []string{"c"},
			Usage:   "Create a single node k3s cluster in a container",
			Flags: []cli.Flag{
				cli.StringFlag{
					Name:  "name, n",
					Value: "k3s_default",
					Usage: "Set a name for the cluster",
				},
				cli.StringFlag{
					Name:  "volume, v",
					Usage: "Mount a volume into the cluster node (Docker notation: `source:destination`)",
				},
				cli.IntFlag{
					Name:  "port, p",
					Value: 6443,
					Usage: "Set a port on which the ApiServer will listen",
				},
			},
			Action: run.CreateCluster,
		},
		{
			// delete deletes an existing k3s cluster (remove container and cluster directory)
			Name:    "delete",
			Aliases: []string{"d"},
			Usage:   "Delete cluster",
			Flags: []cli.Flag{
				cli.StringFlag{
					Name:  "name, n",
					Value: "k3s_default",
					Usage: "name of the cluster",
				},
			},
			Action: run.DeleteCluster,
		},
		{
			// stop stopy a running cluster (its container) so it's restartable
			Name:  "stop",
			Usage: "Stop cluster",
			Flags: []cli.Flag{
				cli.StringFlag{
					Name:  "name, n",
					Value: "k3s_default",
					Usage: "name of the cluster",
				},
			},
			Action: run.StopCluster,
		},
		{
			// start restarts a stopped cluster container
			Name:  "start",
			Usage: "Start a stopped cluster",
			Flags: []cli.Flag{
				cli.StringFlag{
					Name:  "name, n",
					Value: "k3s_default",
					Usage: "name of the cluster",
				},
			},
			Action: run.StartCluster,
		},
		{
			// list prints a list of created clusters
			Name:    "list",
			Aliases: []string{"ls", "l"},
			Usage:   "List all clusters",
			Flags: []cli.Flag{
				cli.BoolFlag{
					Name:  "all, a",
					Usage: "also show non-running clusters",
				},
			},
			Action: run.ListClusters,
		},
		{
			// get-kubeconfig grabs the kubeconfig from the cluster and prints the path to it
			Name:  "get-kubeconfig",
			Usage: "Get kubeconfig location for cluster",
			Flags: []cli.Flag{
				cli.StringFlag{
					Name:  "name, n",
					Value: "k3s_default",
					Usage: "name of the cluster",
				},
			},
			Action: run.GetKubeConfig,
		},
	}

	// run the whole thing
	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

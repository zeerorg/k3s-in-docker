extern crate clap;

mod cmd;

use std::fmt::{self};
use std::io::{self, Write};
use std::process::{self, Command};
use clap::{Arg, App, SubCommand};

fn main() {

    let matches = App::new("k3d")
        .version("0.1.0")
        .author("Rishabh Gupta <r.g.gupta@outlook.com>")
        .about("Run k3s in Docker")
        .subcommand(SubCommand::with_name("check-tools")
                    .about("Check docker running"))
        .subcommand(SubCommand::with_name("create")
                    .about("Create a single node k3s server"))
        .subcommand(SubCommand::with_name("delete")
                    .about("Delete cluster"))
        .subcommand(SubCommand::with_name("stop")
                    .about("Stop cluster"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("check-tools") {
        check_tools();
        println!("Tools running correctly");
    }

    if let Some(_) = matches.subcommand_matches("create") {
        check_tools();
        create_cluster("k3s_default", "6443");
        println!("Created cluster")
    }

    if let Some(_) = matches.subcommand_matches("delete") {
        check_tools();
        stop_cluster("k3s_default");
        delete_cluster("k3s_default");
        println!("Cluster deleted");
    }
}

fn create_cluster(name: &str, port: &str) {
    let port_format = format!("{port}:{port}", port=port);
    let k3_arg = ["run", "--name", name,
                    "-e", "K3S_KUBECONFIG_OUTPUT=/output/kubeconfig.yaml", 
                    "--publish", port_format.as_str(),
                    "--privileged", "-d", 
                    "rancher/k3s:v0.1.0", 
                    "server",  "--https-listen-port", port];
    let mut command = Command::new("docker");
    
    println!("Creating cluster {}", name);
    println!("Running command: docker {}", &k3_arg.join(" "));

    match cmd::run_command(command.args(&k3_arg)) {
        Ok(out) => {},
        Err(e) => {
            eprintln!("Couldn't start k3s");
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}

fn delete_cluster(name: &str) {
    let mut command = Command::new("docker");
    println!("Deleting cluster {}", name);
    match cmd::run_command(command.args(&["rm", name])) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Normal remove doesn't work, doing force remove");
            eprintln!("{}", e);
            let mut command = Command::new("docker");
            match cmd::run_command(command.args(&["rm", "-f", name])) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Force remove doesn't work, unknown error");
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
    }
}

fn stop_cluster(name: &str) {
    let mut command = Command::new("docker");
    println!("Stopping cluster {}", name);
    match cmd::run_command(command.args(&["stop", name])) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("k3s doesn't stop");
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn check_tools() {
    let mut command = Command::new("docker");
    match cmd::run_command(command.args(&["version"])) {
        Ok(s) => {},
        Err(s) => {
            eprintln!("Docker not started");
            eprintln!("{}", s);
            process::exit(1);
        }
    };
}

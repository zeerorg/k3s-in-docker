mod cmd;
mod data;

use std::process::{self, Command};
use std::{thread, time};

pub fn create_cluster(name: &str, port: &str, wait: bool, timeout: u64) {
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
        Ok(_) => {},
        Err(e) => {
            eprintln!("Couldn't start k3s");
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut running = false;
    let start_time = time::SystemTime::now();
    while wait && !running {
        let log_arg = ["logs", name];
        let mut log_command = Command::new("docker");
        match cmd::run_command(log_command.args(&log_arg)) {
            Ok(s) => {
                running = s.contains("Running kubelet");
            },
            Err(s) => {
                eprintln!("Cannot get docker logs.");
                eprintln!("{}", s);
                cleanup(name);
            }
        }

        if (timeout > 0) && (time::SystemTime::now() > (start_time + time::Duration::from_secs(timeout))) {
            eprintln!("Timeout expired.");
            cleanup(name);
        }
        thread::sleep(time::Duration::from_secs(2));
    }

    match data::create_cluster_dir(name) {
        Err(_) => {
            eprintln!("Couldn't create directory.");
            cleanup(name);
        },
        _ => {}
    }
}

fn cleanup(name: &str) {
    delete_cluster(name);
    process::exit(1);
}

pub fn delete_cluster(name: &str) {
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

    match data::delete_cluster_dir(name) {
        Err(_) => {
            eprintln!("Couldn't delete directory.");
            process::exit(1)
        },
        _ => {}
    }
}

pub fn stop_cluster(name: &str) {
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

pub fn check_tools() {
    let mut command = Command::new("docker");
    match cmd::run_command(command.args(&["version"])) {
        Ok(_) => {},
        Err(s) => {
            eprintln!("Docker not started");
            eprintln!("{}", s);
            process::exit(1);
        }
    };
}

pub fn list_clusters() {
    match data::get_directory() {
        Ok(path) => {
            let dir = path.read_dir().unwrap();
            for dirs in dir {
                println!("{:?}", dirs.unwrap().file_name());
            }
        },
        Err(_) => {
            eprintln!("Can't get path directory.");
            process::exit(1);
        }
    }
}

pub fn start_cluster(name: &str) {
    let mut command = Command::new("docker");
    println!("Starting cluster {}", name);
    match cmd::run_command(command.args(&["start", name])) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("k3s doesn't start");
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

pub fn get_kubeconfig(name: &str) {
    let mut command = Command::new("docker");
    let src_fmt = format!("{}:/output/kubeconfig.yaml", name);
    let mut dest = data::get_cluster_path(name).unwrap();
    let args = vec!("cp", src_fmt.as_str(), dest.to_str().unwrap());

    match cmd::run_command(command.args(&args)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Couldn't copy kubeconfig from docker container");
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    dest.push("kubeconfig.yaml");
    println!("{}", dest.to_str().unwrap());
}
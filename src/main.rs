extern crate clap;

pub mod cluster;

use clap::{App, SubCommand, Arg};

fn main() {
    let matches = App::new("k3d")
        .version(env!("GIT_TAG"))
        .author("Rishabh Gupta <r.g.gupta@outlook.com>")
        .about("Run k3s in Docker")
        .subcommand(SubCommand::with_name("check-tools")
                    .about("Check docker running"))
        .subcommand(SubCommand::with_name("create")
                    .about("Create a single node k3s server")
                    .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .default_value("k3s_default")
                            .help("give a name to cluster"))
                    .arg(Arg::with_name("port")
                            .short("p")
                            .long("port")
                            .default_value("6443")
                            .help("provide a different port for cluster"))
                    .arg(Arg::from_usage("-w --wait 'wait for the cluster to start'"))
                    .arg(Arg::with_name("timeout")
                            .long("timeout")
                            .help("Set timeout in seconds when --wait flag is given. (0 for indefinite)")
                            .requires("wait")
                            .takes_value(true)))
        .subcommand(SubCommand::with_name("delete")
                    .about("Delete cluster")
                    .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .default_value("k3s_default")
                            .help("name of the cluster")))
        .subcommand(SubCommand::with_name("stop")
                    .about("Stop a cluster")
                    .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .default_value("k3s_default")
                            .help("name of the cluster")))
        .subcommand(SubCommand::with_name("start")
                    .about("Start a stopped cluster")
                    .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .default_value("k3s_default")
                            .help("name of the cluster")))
        .subcommand(SubCommand::with_name("list")
                    .about("List all clusters"))
        .subcommand(SubCommand::with_name("get-kubeconfig")
                    .about("get kubeconfig.yaml location")
                    .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .default_value("k3s_default")
                            .help("name of the cluster")))
        .get_matches();

    match matches.subcommand() {
        ("check-tools", _) => {
            cluster::check_tools();
            println!("Tools running correctly");
        },
        ("create", Some(subm)) => {
            cluster::check_tools();
            let mut timeout = 0;
            if subm.is_present("timeout") {
                timeout = subm.value_of("timeout").unwrap().parse::<u64>().unwrap();
            }
            cluster::create_cluster(subm.value_of("name").unwrap(), subm.value_of("port").unwrap(), subm.is_present("wait"), timeout);
            println!("Created cluster");
        },
        ("delete", Some(subm)) => {
            cluster::check_tools();
            cluster::stop_cluster(subm.value_of("name").unwrap());
            cluster::delete_cluster(subm.value_of("name").unwrap());
            println!("Cluster deleted");
        },
        ("stop", Some(subm)) => {
            cluster::check_tools();
            cluster::stop_cluster(subm.value_of("name").unwrap());
            println!("Cluster {} stopped", subm.value_of("name").unwrap());
        },
        ("start", Some(subm)) => {
            cluster::check_tools();
            cluster::start_cluster(subm.value_of("name").unwrap());
            println!("Cluster {} started", subm.value_of("name").unwrap());
        },
        ("list", _) => {
            cluster::check_tools();
            cluster::list_clusters();
        },
        ("get-kubeconfig", Some(subm)) => {
            cluster::check_tools();
            cluster::get_kubeconfig(subm.value_of("name").unwrap());
        }
        _ => {}
    }
}

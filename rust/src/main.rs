use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <Dockerfile>", args[0]);
        std::process::exit(1);
    }

    let dockerfile_path = &args[1];
    let file = File::open(dockerfile_path).expect("Failed to open Dockerfile");
    let reader = BufReader::new(file);

    let mut bash_script = Vec::new();
    let mut in_run_instruction = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();

        if line.starts_with("RUN ") {
            // Remove the 'RUN ' prefix
            let bash_command = &line[4..];
            bash_script.push(bash_command.to_string());
            in_run_instruction = true;
        } else if line.starts_with("ENV ") {
            // Parse environment variables and set them in Bash
            let env_parts: Vec<&str> = line.splitn(3, ' ').collect();
            if env_parts.len() >= 3 {
                let env_var = env_parts[1];
                let env_value = env_parts[2];
                bash_script.push(format!("export {}=\"{}\"", env_var, env_value));
            }
        } else if line.starts_with("WORKDIR ") {
            // Change working directory in Bash
            let workdir = &line[8..];
            bash_script.push(format!("cd {}", workdir));
        } else if line.starts_with("COPY ") {
            // Parse COPY instruction and convert it to Bash
            let copy_parts: Vec<&str> = line.split_whitespace().collect();
            if copy_parts.len() >= 3 {
                let source = copy_parts[1];
                let dest = copy_parts[2];
                bash_script.push(format!("cp -r {} {}", source, dest));
            }
        } else if line.starts_with("EXPOSE ") {
            // Exposing ports in Bash (not exact equivalent, just for demonstration)
            let expose_ports: Vec<&str> = line[7..].split_whitespace().collect();
            for port in expose_ports {
                bash_script.push(format!("echo \"Port {} is exposed in Bash\"", port));
            }
        } else if line.starts_with("LABEL ") {
            // Ignoring LABEL instruction in this example
        } else if line.starts_with("ENTRYPOINT ") {
            // Parsing ENTRYPOINT and converting to Bash
            let entrypoint_parts: Vec<&str> = line.splitn(2, ' ').collect();
            if entrypoint_parts.len() >= 2 {
                let entrypoint = entrypoint_parts[1];
                bash_script.push(format!("exec {}", entrypoint));
            }
        } else if line.starts_with("CMD ") {
            // Parsing CMD and converting to Bash
            let cmd_parts: Vec<&str> = line.splitn(2, ' ').collect();
            if cmd_parts.len() >= 2 {
                let cmd = cmd_parts[1];
                bash_script.push(cmd.to_string());
            }
        } else if in_run_instruction && !line.ends_with("\\") {
            in_run_instruction = false;
        }
    }

    // Print the generated Bash script
    for cmd in bash_script {
        println!("{}", cmd);
    }
}
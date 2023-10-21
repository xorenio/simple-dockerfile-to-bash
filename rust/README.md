# dockerfile_to_bash.rs

You can use this script to convert a Dockerfile to a Bash script that contains the RUN instructions as Bash commands:

## Compile the script using the Rust compiler:

```bash
## Compile the script using the Rust compiler:
$ ./rustc dockerfile_to_bash.rs
```

## Run the compiled binary, passing the Dockerfile as an argument:
```bash
$ ./dockerfile_to_bash Dockerfile > converted_script.sh
```

This will provide a bash script in the tty. Please note that this script is a basic example and may not cover all possible complexities of Dockerfile syntax and escaping rules. It assumes a relatively simple Dockerfile format for demonstration purposes.

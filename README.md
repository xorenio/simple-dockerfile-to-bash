# simple_dockerfile_to_bash.py

You can use this script to convert a Dockerfile to a Bash script that contains the RUN instructions as Bash commands:

```bash
$ ./dockerfile_to_bash.py Dockerfile > converted_script.sh
```

This will create a converted_script.sh file with the extracted RUN instructions from the Dockerfile. Please note that this script is a basic example and may not cover all possible complexities of Dockerfile syntax and escaping rules. It assumes a relatively simple Dockerfile format for demonstration purposes.

#!/usr/bin/env python

import sys
import re

if len(sys.argv) != 2:
    print("Usage: python dockerfile_to_bash.py <Dockerfile>")
    sys.exit(1)

dockerfile_path = sys.argv[1]

bash_script = []

with open(dockerfile_path, 'r') as dockerfile:
    for line in dockerfile:
        line = line.strip()
        if line.startswith('RUN '):
            # Remove the 'RUN ' prefix
            bash_command = line[4:]
            bash_script.append(bash_command)
        elif line.startswith('ENV '):
            # Parse environment variables and set them in Bash
            env_match = re.match(r'^ENV\s+(\w+)\s+(.+)$', line)
            if env_match:
                env_var, env_value = env_match.groups()
                bash_script.append(f'export {env_var}="{env_value}"')
        elif line.startswith('WORKDIR '):
            # Change working directory in Bash
            workdir = line[8:]
            bash_script.append(f'cd {workdir}')
        elif line.startswith('COPY '):
            # Parse COPY instruction and convert it to Bash
            copy_match = re.match(r'^COPY\s+(.*?)\s+(.*?)$', line)
            if copy_match:
                source, dest = copy_match.groups()
                bash_script.append(f'cp -r {source} {dest}')
        elif line.startswith('EXPOSE '):
            # Exposing ports in Bash (not exact equivalent, just for demonstration)
            expose_ports = line[7:].split()
            for port in expose_ports:
                bash_script.append(f'echo "Port {port} is exposed in Bash"')
        elif line.startswith('LABEL '):
            # Ignoring LABEL instruction in this example
            pass

# Print the generated Bash script
for cmd in bash_script:
    print(cmd)

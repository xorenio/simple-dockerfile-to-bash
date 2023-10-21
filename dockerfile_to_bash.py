#!/usr/bin/env python

import sys

if len(sys.argv) != 2:
    print("Usage: python dockerfile_to_bash.py <Dockerfile>")
    sys.exit(1)

dockerfile_path = sys.argv[1]

bash_script = []

with open(dockerfile_path, 'r') as dockerfile:
    in_run_instruction = False
    for line in dockerfile:
        line = line.strip()
        if line.startswith('RUN '):
            # Remove the 'RUN ' prefix
            bash_command = line[4:]
            # Append the Bash command to the script
            bash_script.append(bash_command)
            in_run_instruction = True
        elif in_run_instruction:
            # If we're inside a RUN instruction and the line is not a continuation,
            # end the current RUN instruction
            if line.endswith('\\'):
                # Remove the trailing backslash and continue the current RUN instruction
                bash_script[-1] = bash_script[-1] + line[:-1]
            else:
                in_run_instruction = False

# Print the generated Bash script
for cmd in bash_script:
    print(cmd)

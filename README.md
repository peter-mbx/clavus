# Clavus
### Clavum Lateris
#### In vetusta bibliotheca, clavum lateris aperiebat fores librarii occultum, ubi sapientia et arcana conservabantur.

## Description
A lightweight command-line tool designed for anyone in the tech field, specially for IT consultants or professionals managing multiple clients environments. This tool simplifies the management of configurations on your machine, allowing you to configure your environment and load the necessary resources, like files and/or commands.

## Installation
Clone the repository:
```sh
git clone https://github.com/peter-mbx/clavus.git
```
Build the project:
```sh
cargo build --release
```
Run the executable:
```sh
./target/release/clavus
```

## Usage
```
clavus [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    state            Show state file
    status           Show current status
    list             List configurations
    up               Activate a configuration
    down             Deactivate a configuration
    config           Edit configurations

```

## Main Commands

### Show State File
```
clavus state
```

### Show Current Status
```
clavus status
```

### List Configurations
```
clavus list
```

### Activate Configuration
```
clavus up --config-name <name>
```

### Deactivate Configuration
```
clavus down
```

## Configurations Commands

### Create New Configuration
```
clavus config new --config-name <name>
```

### Delete Configuration
```
clavus config delete --config-name <name>
```

### Add File to Configuration
```
clavus config add-file --config-name <name> --id <id> --source <source_path> --target <target_path>
```

### Delete File from Configuration
```
clavus config delete-file --config-name <name> --id <id>
```

### Add Command to Configuration
```
clavus config add-command --config-name <name> --id <id> --up <up_command> --down <down_command>
```

### Delete Command from Configuration
```
clavus config delete-command --config-name <name> --id <id>
```

## About
The tool reads and writes its state file in `JSON` format, typically located at `~/.clavus/state.json`. This file stores all information about existing configurations, such as files and commands associated with each configuration. Manual editing of this file is not recommended, as it may corrupt the tool's functionality.
Files are stored as content in `Base64` format, retaining their original permissions. This means that if a source file has permissions set to mode 664 (readonly=false for Windows), these permissions are replicated to the target path when the configuration is activated.
If a file already exists in the target path, its content is saved in the state file as 'old_content' and its permissions as 'old_permissions'. This allows the file to be restored to its previous state, with its original permissions and content, when the configuration is deactivated.
I am currently developing support for remotely managing the state file, such as from an AWS bucket, allowing all configurations to be accessed from multiple devices.

### An example of a state file with one configuration:

```
{
  "configs": [
    {
      "name": "main",
      "files": [
        {
          "id": "gitconfig",
          "target": "/Users/pietro/.gitconfig",
          "content": "W3VzZXJdCgluYW1lID0gUGlldHJvIE1vYnJpY2kKCWVtYWlsID0gcGlldHJvLm1vYnJpY2lAZ21haWwuY29tCg==",
          "old_content": null,
          "permissions": {
            "mode": "644",
            "readonly": false
          },
          "old_permissions": null
        },
        {
          "id": "sshconfig",
          "target": "/Users/pietro/.ssh/config",
          "content": "SG9zdCBnaXRodWIuY29tCkhvc3ROYW1lIGdpdGh1Yi5jb20KSWRlbnRpdHlGaWxlIH4vLnNzaC9rZXkucHJpdgo=",
          "old_content": null,
          "permissions": {
            "mode": "644",
            "readonly": false
          },
          "old_permissions": null
        },
        {
          "id": "sshkey",
          "target": "/Users/pietro/.ssh/key.priv",
          "content": "LS0tLS1CRUdJTiBPUEVOU1NIIFBSSVZBVEUgS0VZLS0tLS0KY29udGVudAotLS0tLUVORCBPUEVOU1NIIFBSSVZBVEUgS0VZLS0tLS0K",
          "old_content": null,
          "permissions": {
            "mode": "400",
            "readonly": true
          },
          "old_permissions": null
        },
        {
          "id": "aws",
          "target": "/Users/pietro/.aws/config",
          "content": "W3Byb2ZpbGUgbWFpbl0KYXdzX2FjY2Vzc19rZXlfaWQgPSBhY2Nlc3MKYXdzX3NlY3JldF9hY2Nlc3Nfa2V5ID0gc2VjcmV0CnJlZ2lvbiA9IHVzLWVhc3QtMQpvdXRwdXQgPSBqc29uCg==",
          "old_content": null,
          "permissions": {
            "mode": "644",
            "readonly": false
          },
          "old_permissions": null
        }
      ],
      "commands": [
        {
          "id": "salute",
          "up": "echo 'Hello, World!'",
          "down": "echo 'Goodbye, World!'"
        }
      ]
    }
  ],
  "active": ""
}
```

## Dependencies (crates)
```
base64
clap
colored
dirs
once_cell
serde
serde_json
which
```

## Disclaimer
I am relatively new to Rust and have been learning the language for just 1 month. While I strive to write clean and efficient code, it's important to note that there may be instances of suboptimal or non-idiomatic Rust practices in this project. I welcome feedback and suggestions from experienced Rust developers to improve the quality of this project and my understanding of the language.

## Author

```
Pietro Mobrici
pietro.mobrici@gmail.com
```

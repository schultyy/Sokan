# Sokan
Provisioning with Rust

## Configuration
A machine get's a Yaml file which describes its final state

```yaml
default:
  files:
    -
      path: '/home/jane/hello.txt'
      content: 'Hello from Jane'
    -
      path: '/home/jane/script.sh'
      content: >
        #!/bin/bash

        echo "Hello from Bash"

  packages:
    - vim
    - git
  package_install_command: "yum install -y"
```

Every definition starts with the machine name. If there's only one machine you can use `default`.

## Apply configuration to machine

```bash
$ sokan
```
This requires, that there is a file with `default.yaml`.

## Requirements

This requires Rust 1.2 for building the `sokan` executable.

## License

MIT

# Sokan
Provisioning with Rust

## Configuration
A machine get's a Yaml file which describes its final state

```yaml
default:
  commands:
    - sudo yum clean all
    - touch hello.txt
    - echo "test" > hello.txt
    - cat hello.txt
    - rm hello.txt
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

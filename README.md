# Sokan
Provisioning with Rust

## Configuration

```yaml
default:
  - touch hello.txt
  - echo "some content" > hello.txt
  - apt-get install vim
```

This first creates a file `hello.txt`, then echoes "some content" into it and last installs vim.
Every definition starts with the machine name. If there's only one machine you can use `default`.
The configuration needs to be stored in a yaml file.

## Apply configuration to machine

```bash
$ sokan apply my_config.yaml
```

## Requirements

This requires Rust for building the `sokan` executable.

## License

MIT

# Sokan
Sokan is a provisioner written in Rust. It consumes ordinary Yaml configuration files.

## Configuration
You are able to specify the following things in the configuration:
- A hostname
- A package install command
- packages to be installed
- Files which shall be placed at certains paths with specified content

### Host names

You are able to set a host name for your machine. The host name is set via the top level key:

```yaml
srv01:
  #...
```

Setting it is optional. If you don't want to specify it, leave it to `default`.

```yaml
default:
  #...
```

### Packages

Via the `packages` key you're able to specify which packages should be installed on your machine:

```yaml
default:
  - packages:
    - vim
    - git
    - postgres
```

### Files

It is also possible to specify a list of files which need to be present:

```yaml
default:
  - files:
    -
      path: '/home/john/.gitconfig`
      content: >

        [core]
	        excludesfile = ~/.gitignore
	        editor = vim

        [push]
	        default = simple
```

A file is only written if not existant or if already there, the content is not equal to the one to be written.

## Apply configuration to machine

You need to pass the configuration file as an argument:

```bash
$ sokan default.yaml
```

## Requirements

This requires Rust 1.2 for building the `sokan` executable.

## License

MIT

# Indice

A simple CLI tool to navigate between projects using configuration in `.indicerc`.


# Setup

**TODO add more configuration steps**

```bash
make build
make release
```

Add to `~/.bashrc` or `~/.zshrc`
```bash
indice() {
    output=$(command indice "$@")
    if [[ $output == *CHANGE_DIR=* ]]; then
        # Extract the directory path and cd to it
        dir=$(echo "$output" | grep CHANGE_DIR= | cut -d= -f2)
        cd "$dir"
    else
        echo "$output"
    fi
}
```


# Configuration

After installation, edit your `~/.indicerc` file to add your projects:

```yaml
# Enable or disable features
enable_feature: true

# Project definitions
projects:
  # Add your projects here
  myproject:
    - path: /absolute/path/to/myproject

  another-project:
    - path: /path/to/another/project
```

Each project entry requires:
- A unique project name
- A path to the project directory

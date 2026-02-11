# Indice

A simple CLI tool to navigate between projects using configuration in `.indicerc`. Use `--config` to specify a different config file.


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

# Project definitions (paths support $HOME, ~, or bare relative → ~/)
projects:
  # Add your projects here
  myproject:
    - path: $HOME/projects/myproject

  another-project:
    - path: ~/workspace/another-project
```

Each project entry requires:
- A unique project name
- A path to the project directory:
  - `$HOME/path` or `~/path` — explicit home-relative (portable)
  - `projects/foo` — bare relative paths are treated as `~/projects/foo`

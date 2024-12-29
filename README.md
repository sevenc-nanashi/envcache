# envcache / Cache your command to speed up your bashrc

envcache is a simple tool to cache the output of a command to speed up your profile.

## Installation

```bash
cargo install --git https://github.com/sevenc-nanashi/envcache.git
```

## Usage

```bash
# Replace this...
eval "$(github-copilot-cli alias -- "$0")"
# ...with this
eval "$(envcache github-copilot-cli alias -- "$0")"
```

```
❯ envcache
Usage: /home/sevenc7c/.cargo/bin/envcache <command> [args...]
Options:
  --version    Print version information
  --help       Print this help message
  --cache-dir  Print the cache directory
  --purge      Purge the cache directory
```

## License

MIT License.

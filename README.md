# zellij-ssh-split

A standalone Zellij plugin that adds key-triggerable actions for:

- opening a new pane that reuses the current pane's `ssh ...` command
- opening a new tab that reuses the current pane's `ssh ...` command

If the focused pane is not currently running `ssh`, it falls back to normal Zellij behavior:

- new pane: open the default shell in a new pane
- new tab: open the default shell in a new tab, preserving the focused pane's cwd when available

SSH-cloned panes are named with a `[ssh]` prefix.

## Install

The recommended installation method is to define a plugin alias that points at the versioned `.wasm` asset on GitHub Releases.

Replace `v0.1.0` with the release you want:

```kdl
plugins {
    ssh-split location="https://github.com/yqmmm/zellij-ssh-split/releases/download/v0.1.0/zellij-ssh-split.wasm"
}

load_plugins {
    "ssh-split"
}
```

Then bind keys with `MessagePlugin "ssh-split"`:

```kdl
keybinds {
    normal {
        bind "Alt s" {
            MessagePlugin "ssh-split" {
                payload "pane"
            }
        }
        bind "Alt t" {
            MessagePlugin "ssh-split" {
                payload "tab"
            }
        }
    }
    pane {
        bind "r" {
            MessagePlugin "ssh-split" {
                payload "pane-right"
            }
            SwitchToMode "Normal"
        }
        bind "d" {
            MessagePlugin "ssh-split" {
                payload "pane-down"
            }
            SwitchToMode "Normal"
        }
    }
}
```

Use a versioned release URL for stability. `latest/download/...` is convenient, but it will change as new releases are published.

## Release Compatibility

Zellij plugins are version-sensitive. Build this plugin against the same Zellij plugin API version as the Zellij binary you run.

Current dependency versions in this repo:

```toml
zellij-tile = "0.44.0"
zellij-utils = "0.44.0"
```

If your installed Zellij is newer or older than `0.44.0`, update the crate versions in [`Cargo.toml`](/Users/yuqianmian/code/zellij-ssh-split/Cargo.toml) accordingly.

## Local Build

This plugin uses published crates from crates.io and does not depend on a local Zellij checkout.

1. Install the WASI target:

```bash
rustup target add wasm32-wasip1
```

2. Build the plugin:

```bash
cargo build --release --target wasm32-wasip1
```

The resulting plugin will be:

```text
target/wasm32-wasip1/release/zellij-ssh-split.wasm
```

For a local checkout, use:

```kdl
plugins {
    ssh-split location="file:/Users/yuqianmian/code/zellij-ssh-split/target/wasm32-wasip1/release/zellij-ssh-split.wasm"
}

load_plugins {
    "ssh-split"
}
```

## Notes

- The plugin only clones `ssh` commands. It intentionally ignores other foreground commands.
- It uses Zellij's plugin APIs to inspect the focused pane and then dispatches normal Zellij actions.
- No changes to the Zellij repo are required.
- This repo includes a GitHub Actions workflow that builds `zellij-ssh-split.wasm` and uploads it to tagged releases.

# zellij-ssh-split

A standalone Zellij plugin that adds key-triggerable actions for:

- opening a new pane that reuses the current pane's `ssh ...` command
- opening a new tab that reuses the current pane's `ssh ...` command

If the focused pane is not currently running `ssh`, it falls back to normal Zellij behavior:

- new pane: open the default shell in a new pane
- new tab: open the default shell in a new tab, preserving the focused pane's cwd when available

## Build

This plugin uses published crates from crates.io and does not depend on a local Zellij checkout.

Current dependency versions in this repo:

```toml
zellij-tile = "0.44.0"
zellij-utils = "0.44.0"
```

Important: Zellij plugins are version-sensitive. Build this plugin against the same Zellij plugin API version as the Zellij binary you run. If your installed Zellij is newer or older than `0.44.0`, update the crate versions in [`Cargo.toml`](/Users/yuqianmian/code/zellij-ssh-split/Cargo.toml) accordingly.

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

## Zellij Config

Add the plugin to `load_plugins` so it runs in the background on session start:

```kdl
load_plugins {
    "file:/Users/yuqianmian/code/zellij-ssh-split/target/wasm32-wasip1/release/zellij-ssh-split.wasm"
}
```

Bind keys with `MessagePlugin`:

```kdl
keybinds {
    normal {
        bind "Alt s" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-split/target/wasm32-wasip1/release/zellij-ssh-split.wasm" {
                payload "pane"
            }
        }
        bind "Alt t" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-split/target/wasm32-wasip1/release/zellij-ssh-split.wasm" {
                payload "tab"
            }
        }
    }
    pane {
        bind "r" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-split/target/wasm32-wasip1/release/zellij-ssh-split.wasm" {
                payload "pane-right"
            }
            SwitchToMode "Normal"
        }
        bind "d" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-split/target/wasm32-wasip1/release/zellij-ssh-split.wasm" {
                payload "pane-down"
            }
            SwitchToMode "Normal"
        }
    }
}
```

## Notes

- The plugin only clones `ssh` commands. It intentionally ignores other foreground commands.
- It uses Zellij's plugin APIs to inspect the focused pane and then dispatches normal Zellij actions.
- No changes to the Zellij repo are required.

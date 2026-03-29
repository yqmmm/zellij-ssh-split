# zellij-ssh-clone-plugin

A standalone Zellij plugin that adds key-triggerable actions for:

- opening a new pane that reuses the current pane's `ssh ...` command
- opening a new tab that reuses the current pane's `ssh ...` command

If the focused pane is not currently running `ssh`, it falls back to normal Zellij behavior:

- new pane: open the default shell in a new pane
- new tab: open the default shell in a new tab, preserving the focused pane's cwd when available

## Build

This plugin uses your local `~/code/zellij` checkout as a path dependency.

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
target/wasm32-wasip1/release/zellij_ssh_clone_plugin.wasm
```

## Zellij Config

Add the plugin to `load_plugins` so it runs in the background on session start:

```kdl
load_plugins {
    "file:/Users/yuqianmian/code/zellij-ssh-clone-plugin/target/wasm32-wasip1/release/zellij_ssh_clone_plugin.wasm"
}
```

Bind keys with `MessagePlugin`:

```kdl
keybinds {
    normal {
        bind "Alt s" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-clone-plugin/target/wasm32-wasip1/release/zellij_ssh_clone_plugin.wasm" {
                payload "pane"
            }
        }
        bind "Alt t" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-clone-plugin/target/wasm32-wasip1/release/zellij_ssh_clone_plugin.wasm" {
                payload "tab"
            }
        }
    }
    pane {
        bind "r" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-clone-plugin/target/wasm32-wasip1/release/zellij_ssh_clone_plugin.wasm" {
                payload "pane-right"
            }
            SwitchToMode "Normal"
        }
        bind "d" {
            MessagePlugin "file:/Users/yuqianmian/code/zellij-ssh-clone-plugin/target/wasm32-wasip1/release/zellij_ssh_clone_plugin.wasm" {
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

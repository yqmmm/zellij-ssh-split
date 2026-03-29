use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use zellij_tile::prelude::*;
use zellij_tile::prelude::actions::Action;
use zellij_utils::input::command::RunCommandAction;

#[derive(Default)]
struct State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        set_selectable(false);
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::RunActionsAsUser,
        ]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        match pipe_message.payload.as_deref().or(Some(pipe_message.name.as_str())) {
            Some("pane") => self.open_pane(None),
            Some("pane-right") => self.open_pane(Some(Direction::Right)),
            Some("pane-left") => self.open_pane(Some(Direction::Left)),
            Some("pane-up") => self.open_pane(Some(Direction::Up)),
            Some("pane-down") => self.open_pane(Some(Direction::Down)),
            Some("tab") => self.open_tab(),
            _ => {},
        }
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}

impl State {
    fn open_pane(&mut self, direction: Option<Direction>) {
        let Some(context) = self.focused_terminal_context() else {
            return;
        };
        let command = context.ssh_run_command.map(Into::into);
        run_action(
            Action::NewTiledPane {
                direction,
                command,
                pane_name: context.pane_name,
                near_current_pane: false,
                borderless: None,
            },
            BTreeMap::new(),
        );
    }

    fn open_tab(&mut self) {
        let Some(context) = self.focused_terminal_context() else {
            return;
        };
        let initial_panes = context
            .ssh_run_command
            .map(|run_command| vec![CommandOrPlugin::Command(run_command.into())]);
        run_action(
            Action::NewTab {
                tiled_layout: None,
                floating_layouts: vec![],
                swap_tiled_layouts: None,
                swap_floating_layouts: None,
                tab_name: None,
                should_change_focus_to_new_tab: true,
                cwd: context.cwd,
                initial_panes,
                first_pane_unblock_condition: None,
            },
            BTreeMap::new(),
        );
    }

    fn focused_terminal_context(&self) -> Option<FocusedTerminalContext> {
        let (_tab_index, pane_id) = get_focused_pane_info().ok()?;
        let cwd = get_pane_cwd(pane_id).ok().or_else(fallback_cwd);
        let running_command = get_pane_running_command(pane_id).ok();
        let pane_name = running_command
            .as_ref()
            .and_then(|running_command| ssh_pane_name(running_command));
        let ssh_run_command =
            running_command.and_then(|running_command| ssh_run_command(running_command, cwd.clone()));
        Some(FocusedTerminalContext {
            cwd,
            pane_name,
            ssh_run_command,
        })
    }
}

struct FocusedTerminalContext {
    cwd: Option<PathBuf>,
    pane_name: Option<String>,
    ssh_run_command: Option<RunCommandAction>,
}

fn ssh_run_command(command: Vec<String>, cwd: Option<PathBuf>) -> Option<RunCommandAction> {
    let executable = command.first()?;
    let executable_name = Path::new(executable)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(executable)
        .to_ascii_lowercase();
    if executable_name != "ssh" {
        return None;
    }
    Some(RunCommandAction {
        command: PathBuf::from(executable),
        args: command.iter().skip(1).cloned().collect(),
        cwd,
        direction: None,
        hold_on_close: false,
        hold_on_start: false,
        originating_plugin: None,
        use_terminal_title: false,
    })
}

fn ssh_pane_name(command: &[String]) -> Option<String> {
    let executable = command.first()?;
    let executable_name = Path::new(executable)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(executable)
        .to_ascii_lowercase();
    if executable_name != "ssh" {
        return None;
    }
    ssh_target(&command[1..]).map(|target| format!("[ssh] {target}"))
}

fn ssh_target(args: &[String]) -> Option<&str> {
    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        if arg == "--" {
            return iter.next().map(|s| s.as_str());
        }
        if !arg.starts_with('-') || arg == "-" {
            return Some(arg.as_str());
        }
        if option_takes_value(arg) && !option_value_is_attached(arg) {
            iter.next();
        }
    }
    None
}

fn option_takes_value(arg: &str) -> bool {
    matches!(
        arg.chars().nth(1),
        Some(
            'B'
                | 'b'
                | 'c'
                | 'D'
                | 'E'
                | 'e'
                | 'F'
                | 'I'
                | 'i'
                | 'J'
                | 'L'
                | 'l'
                | 'm'
                | 'O'
                | 'o'
                | 'p'
                | 'Q'
                | 'R'
                | 'S'
                | 'W'
                | 'w'
        )
    )
}

fn option_value_is_attached(arg: &str) -> bool {
    arg.len() > 2 && !arg.starts_with("--")
}

fn fallback_cwd() -> Option<PathBuf> {
    let plugin_ids = get_plugin_ids();
    if plugin_ids.initial_cwd.as_os_str().is_empty() {
        None
    } else {
        Some(plugin_ids.initial_cwd)
    }
}

use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommandSet, AddConsoleCommand, ConsoleCommand, ConsolePlugin};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum GameState {
    Loading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_state(GameState::Loading)
        .add_console_command::<ChangeStateCommand, _, _>(state_command)
        .add_console_command_set::<LogCommand>(
            SystemSet::on_update(GameState::Loading).with_system(log_command_in_loading),
        )
        .add_console_command_set::<LogCommand>(
            SystemSet::on_update(GameState::InGame).with_system(log_command),
        )
        .run();
}

fn log_command_in_loading(mut log: ConsoleCommand<LogCommand>) {
    if let Some(LogCommand { msg, num }) = log.take() {
        let repeat_count = num.unwrap_or(1);
        reply!(log, "in loading");
        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}

/// Prints given arguments to the console
#[derive(ConsoleCommand)]
#[console_command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}

fn log_command(mut log: ConsoleCommand<LogCommand>) {
    if let Some(LogCommand { msg, num }) = log.take() {
        let repeat_count = num.unwrap_or(1);
        reply!(log, "in in-game");
        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}

/// Prints given arguments to the console
#[derive(ConsoleCommand)]
#[console_command(name = "state")]
struct ChangeStateCommand {
    /// Message to print
    game_state: String,
}

fn state_command(mut log: ConsoleCommand<ChangeStateCommand>, mut state: ResMut<State<GameState>>) {
    if let Some(ChangeStateCommand { game_state }) = log.take() {
        if game_state == "in-game" {
            state.push(GameState::InGame).expect("fail to push state");
            log.ok();
        } else if game_state == "loading" {
            state.push(GameState::Loading).expect("fail to push state");
            log.ok();
        } else {
            log.failed();
        }
    }
}

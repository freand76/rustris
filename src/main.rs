mod tetris_gui;
mod tetris_model;

use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use tetris_model::TetrisState;

#[derive(PartialEq)]
pub enum GameState {
    Intro,
    Game,
    End,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn intro_state_control(key: KeyEvent, tetris_state: &mut TetrisState) -> GameState {
    match key.code {
        KeyCode::Char(' ') => {
            tetris_state.restart(0);
            GameState::Game
        }
        KeyCode::Char('q') => GameState::End,
        _ => GameState::Intro,
    }
}

fn game_state_control(key: KeyEvent, tetris_state: &mut TetrisState) -> GameState {
    if key.code == KeyCode::Esc {
        return GameState::Intro;
    }

    match key.code {
        KeyCode::Up => {
            tetris_state.rotate_cw();
        }
        KeyCode::Down => {
            tetris_state.rotate_ccw();
        }
        KeyCode::Left => {
            tetris_state.move_left();
        }
        KeyCode::Right => {
            tetris_state.move_right();
        }
        KeyCode::Char(' ') => {
            tetris_state.drop();
        }
        _ => {}
    }

    GameState::Game
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut game_state = GameState::Intro;

    let mut tetris_state = TetrisState::new(0);

    loop {
        terminal.draw(|f| {
            tetris_gui::draw(f, &game_state, &tetris_state);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                game_state = match game_state {
                    GameState::Intro => intro_state_control(key, &mut tetris_state),
                    GameState::Game => game_state_control(key, &mut tetris_state),
                    _ => game_state,
                }
            }
        }

        if game_state == GameState::Game {
            tetris_state.tick();

            if tetris_state.is_game_over() {
                game_state = GameState::Intro;
            }
        }

        if game_state == GameState::End {
            break;
        }
    }

    Ok(())
}

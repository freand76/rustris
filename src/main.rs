mod tetris_model;

use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use tetris_model::{create_tetris_game, BlockColor, TetrisState};

#[derive(PartialEq)]
enum GameState {
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

fn intro_state_control(key: KeyEvent) -> GameState {
    // Input handling
    return match key.code {
        KeyCode::Char(' ') => GameState::Game,
        KeyCode::Char('q') => GameState::End,
        _ => GameState::Intro,
    };
}

fn game_state_control(key: KeyEvent, tetris_state: &mut TetrisState) -> GameState {
    // Input handling
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

fn intro_field(f: &mut Frame, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Rustris");

    //f.render_widget(block, area);

    let content = "Press Space to Start!";

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Cyan))
        .block(block);

    f.render_widget(paragraph, area);
}

fn from_block_color(block_color: BlockColor) -> Color {
    match block_color {
        BlockColor::Black => Color::Black,
        BlockColor::Red => Color::Red,
        BlockColor::Blue => Color::Blue,
        BlockColor::Yellow => Color::Yellow,
        BlockColor::Green => Color::Green,
        BlockColor::Magenta => Color::Magenta,
        BlockColor::Cyan => Color::Cyan,
        BlockColor::Orange => Color::LightRed,
    }
}

fn game_field(f: &mut Frame, area: Rect, tetris_state: &TetrisState) {
    let block = Block::default().borders(Borders::ALL).title("Rustris");

    //f.render_widget(block, area);

    let content = "Hello";

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Cyan))
        .block(block);

    f.render_widget(paragraph, area);
    let field = tetris_state.get_field();

    for y in 0..field.height() {
        for x in 0..field.width() {
            let cell_x = area.x + 1 + x as u16;
            let cell_y = area.y + 1 + y as u16;

            let cell =
                Paragraph::new("█").style(Style::default().fg(from_block_color(field.data[y][x])));
            f.render_widget(cell, Rect::new(cell_x, cell_y, 1, 1));
        }
    }
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut game_state = GameState::Intro;

    let mut tetris_state = create_tetris_game(0);

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            match game_state {
                GameState::Intro => intro_field(f, chunks[0]),
                GameState::Game => game_field(f, chunks[0], &tetris_state),
                _ => {}
            }
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                game_state = match game_state {
                    GameState::Intro => intro_state_control(key),
                    GameState::Game => game_state_control(key, &mut tetris_state),
                    _ => game_state,
                }
            }
        }

        if game_state == GameState::End {
            break;
        }
    }

    Ok(())
}

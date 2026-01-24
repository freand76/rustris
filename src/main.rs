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

fn game_state_control(key: KeyEvent) -> GameState {
    // Input handling
    return match key.code {
        KeyCode::Esc => GameState::Intro,
        _ => GameState::Game,
    };
}

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

type Playfield = [[Color; WIDTH]; HEIGHT];
fn empty_field() -> Playfield {
    [[Color::Blue; WIDTH]; HEIGHT]
}

fn demo_field() -> Playfield {
    let mut field = empty_field();
    field[0][0] = Color::Red;
    field[0][9] = Color::Red;
    field[19][0] = Color::Red;
    field[19][9] = Color::Red;
    field[19][4] = Color::Cyan;
    field[19][5] = Color::Cyan;
    field[18][5] = Color::Cyan;
    field[18][6] = Color::Cyan;
    field
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

fn game_field(f: &mut Frame, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Rustris");

    //f.render_widget(block, area);

    let content = "Hello";

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Cyan))
        .block(block);

    f.render_widget(paragraph, area);
    let field = demo_field();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cell_x = area.x + 1 + x as u16;
            let cell_y = area.y + 1 + y as u16;

            let cell = Paragraph::new("█").style(Style::default().fg(field[y][x]));
            f.render_widget(cell, Rect::new(cell_x, cell_y, 1, 1));
        }
    }
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut game_state = GameState::Intro;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            match game_state {
                GameState::Intro => intro_field(f, chunks[0]),
                GameState::Game => game_field(f, chunks[0]),
                _ => {}
            }
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                game_state = match game_state {
                    GameState::Intro => intro_state_control(key),
                    GameState::Game => game_state_control(key),
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

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
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use tetris_model::{BlockColor, Playfield, TetrisState};

const FRAME_WIDTH: u16 = 80;
const FRAME_HEIGHT: u16 = 24;
const BOARD_HEIGHT: u16 = Playfield::height() as u16 + 2; // playfield rows + 2 for borders

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

fn intro_state_control(key: KeyEvent, tetris_state: &mut TetrisState) -> GameState {
    // Input handling
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
    let content = "Press Space to Start!";

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Cyan))
        .alignment(ratatui::layout::Alignment::Center);

    let v_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage(45),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Percentage(45),
        ])
        .split(area);

    f.render_widget(paragraph, v_chunks[1]);
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
    let level = tetris_state.level();
    let score_content = format!("Level: {}\nLines: 0\nScore: 0", level);
    let score_block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .title("ScoreBoard");
    let score_paragraph = Paragraph::new(score_content)
        .block(score_block)
        .style(Style::default().fg(Color::Cyan));

    let next_piece_block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .title("Next Piece");
    let next_piece_paragraph = Paragraph::new("").block(next_piece_block);

    // Split the 80x40 area into the Board (middle) and the Side Panel (right).
    // Let's divide 80 into: 30 (left padding), 20 (board), 30 (right side panel)
    let h_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Length(30), // Left spacer
            ratatui::layout::Constraint::Length(22), // Board width (20 + 2 for borders)
            ratatui::layout::Constraint::Min(20),    // Right panel
        ])
        .split(area);

    // Vertical padding to center the 22-tall board in the inner area
    // Inner height = FRAME_HEIGHT - 2 (outer borders)
    let inner_height = FRAME_HEIGHT - 2;
    let board_v_pad = (inner_height - BOARD_HEIGHT) / 2;

    let board_v_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(board_v_pad),
            ratatui::layout::Constraint::Length(BOARD_HEIGHT),
            ratatui::layout::Constraint::Min(0),
        ])
        .split(h_chunks[1]);

    let board_area = board_v_chunks[1];

    // Inside the right panel, center scoreboard and next piece vertically.
    let right_panel_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(board_v_pad), // Match board top padding
            ratatui::layout::Constraint::Length(6),           // Scoreboard height
            ratatui::layout::Constraint::Length(1),           // Gap
            ratatui::layout::Constraint::Length(6),           // Next piece height
            ratatui::layout::Constraint::Min(0),              // Bottom flex
        ])
        .split(h_chunks[2]);

    let score_area = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Min(0),     // Left flex
            ratatui::layout::Constraint::Length(18),  // score box width
            ratatui::layout::Constraint::Min(0),     // Right flex
        ])
        .split(right_panel_chunks[1])[1];

    let next_piece_area = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Min(0),     // Left flex
            ratatui::layout::Constraint::Length(18),  // Next piece box width
            ratatui::layout::Constraint::Min(0),     // Right flex
        ])
        .split(right_panel_chunks[3])[1];

    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new("").block(block);
    f.render_widget(paragraph, board_area);

    let field = tetris_state.field();
    let data = field.data();

    // Now draw the actual cells inside the board_area offset
    for (y, row) in data.iter().enumerate().take(Playfield::height()) {
        for (x, &color) in row.iter().enumerate().take(Playfield::width()) {
            let cell_x = board_area.x + 1 + 2 * x as u16;
            let cell_y = board_area.y + 1 + y as u16;

            let cell = Paragraph::new("██").style(Style::default().fg(from_block_color(color)));
            f.render_widget(cell, Rect::new(cell_x, cell_y, 2, 1));
        }
    }

    // Finally render the scoreboard and next piece
    f.render_widget(score_paragraph, score_area);
    f.render_widget(next_piece_paragraph, next_piece_area);

    // ASCII Rustris logo in the left panel — each letter colored from the Tetris palette
    // Letter definitions: 4 rows each, 3 chars wide
    let letter_data: [([&str; 4], Color); 7] = [
        (["█▀▄", "██▀", "█ █", "▀ ▀"], Color::Red),       // R
        (["█ █", "█ █", "█ █", "▀█▀"], Color::Cyan),      // U
        (["▄█▀", "▀█▄", "▄▄█", "▀▀ "], Color::Yellow),    // S
        (["▀█▀", " █ ", " █ ", " ▀ "], Color::Green),     // T
        (["█▀▄", "██▀", "█ █", "▀ ▀"], Color::Magenta),   // R
        ([" █ ", " █ ", " █ ", " ▀ "], Color::Blue),      // I
        (["▄█▀", "▀█▄", "▄▄█", "▀▀ "], Color::LightRed),  // S
    ];

    let mut logo_lines: Vec<Line> = Vec::new();
    for row in 0..4 {
        let mut spans: Vec<Span> = Vec::new();
        for (i, (rows, color)) in letter_data.iter().enumerate() {
            spans.push(Span::styled(
                rows[row],
                Style::default().fg(*color),
            ));
            if i < 6 {
                spans.push(Span::raw(" "));
            }
        }
        logo_lines.push(Line::from(spans));
    }

    let logo_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));
    let logo_paragraph = Paragraph::new(logo_lines)
        .block(logo_block)
        .alignment(ratatui::layout::Alignment::Center);

    let logo_v_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(0),
            ratatui::layout::Constraint::Length(6),
            ratatui::layout::Constraint::Min(0),
        ])
        .split(h_chunks[0]);

    let logo_h_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Length(0),
            ratatui::layout::Constraint::Length(29),
            ratatui::layout::Constraint::Min(0),
        ])
        .split(logo_v_chunks[1]);

    f.render_widget(logo_paragraph, logo_h_chunks[1]);
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut game_state = GameState::Intro;

    let mut tetris_state = TetrisState::new(0);

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Check if terminal is too small
            if size.width < FRAME_WIDTH || size.height < FRAME_HEIGHT {
                let warning = Paragraph::new(format!(
                    "Terminal too small!\nRequires at least {}x{}.",
                    FRAME_WIDTH, FRAME_HEIGHT
                ))
                .style(Style::default().fg(Color::Red))
                .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(warning, size);
                return;
            }

            // Horizontal split
            let h_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),              // Left flex
                    Constraint::Length(FRAME_WIDTH),  // Total wrapper width
                    Constraint::Min(0),              // Right flex
                ])
                .split(size);

            // Vertical split of the middle segment
            let v_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),               // Top flex
                    Constraint::Length(FRAME_HEIGHT),  // Total wrapper height
                    Constraint::Min(0),               // Bottom flex
                ])
                .split(h_chunks[1]);

            let outer_frame = Block::default().borders(Borders::ALL);
            f.render_widget(outer_frame, v_chunks[1]);

            let inner_area = v_chunks[1].inner(&ratatui::layout::Margin {
                vertical: 1,
                horizontal: 1,
            });

            match game_state {
                GameState::Intro => intro_field(f, inner_area),
                GameState::Game => game_field(f, inner_area, &tetris_state),
                _ => {}
            }
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

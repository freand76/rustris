use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tetris_model::{BlockColor, Playfield, TetrisState};

pub const FRAME_WIDTH: u16 = 80;
pub const FRAME_HEIGHT: u16 = 24;
const BOARD_HEIGHT: u16 = Playfield::height() as u16 + 2; // playfield rows + 2 for borders

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

fn intro_field(f: &mut Frame, area: Rect) {
    let content = "Press Space to Start!";

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(1),
            Constraint::Percentage(45),
        ])
        .split(area);

    f.render_widget(paragraph, v_chunks[1]);
}

fn game_field(f: &mut Frame, area: Rect, tetris_state: &TetrisState) {
    let level = tetris_state.level();
    let score_content = format!("Level: {}\nLines: 0\nScore: 0", level);
    let score_block = Block::default().borders(Borders::ALL).title("ScoreBoard");
    let score_paragraph = Paragraph::new(score_content)
        .block(score_block)
        .style(Style::default().fg(Color::Cyan));

    let next_piece_block = Block::default().borders(Borders::ALL).title("Next Piece");
    let next_piece_paragraph = Paragraph::new("").block(next_piece_block);

    // Split area into: left logo panel, board, right side panel
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30), // Left spacer
            Constraint::Length(22), // Board width (20 + 2 for borders)
            Constraint::Min(20),    // Right panel
        ])
        .split(area);

    // Vertical padding to center the board in the inner area
    let inner_height = FRAME_HEIGHT - 2;
    let board_v_pad = (inner_height - BOARD_HEIGHT) / 2;

    let board_v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(board_v_pad),
            Constraint::Length(BOARD_HEIGHT),
            Constraint::Min(0),
        ])
        .split(h_chunks[1]);

    let board_area = board_v_chunks[1];

    // Right panel: scoreboard and next piece, horizontally centered
    let right_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(board_v_pad), // Match board top padding
            Constraint::Length(6),           // Scoreboard height
            Constraint::Length(1),           // Gap
            Constraint::Length(6),           // Next piece height
            Constraint::Min(0),              // Bottom flex
        ])
        .split(h_chunks[2]);

    let score_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left flex
            Constraint::Length(18), // score box width
            Constraint::Min(0),     // Right flex
        ])
        .split(right_panel_chunks[1])[1];

    let next_piece_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left flex
            Constraint::Length(18), // Next piece box width
            Constraint::Min(0),     // Right flex
        ])
        .split(right_panel_chunks[3])[1];

    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new("").block(block);
    f.render_widget(paragraph, board_area);

    let field = tetris_state.field();
    let data = field.data();

    // Draw the actual cells inside the board_area
    for (y, row) in data.iter().enumerate().take(Playfield::height()) {
        for (x, &color) in row.iter().enumerate().take(Playfield::width()) {
            let cell_x = board_area.x + 1 + 2 * x as u16;
            let cell_y = board_area.y + 1 + y as u16;

            let cell = Paragraph::new("██").style(Style::default().fg(from_block_color(color)));
            f.render_widget(cell, Rect::new(cell_x, cell_y, 2, 1));
        }
    }

    // Render scoreboard and next piece
    f.render_widget(score_paragraph, score_area);
    f.render_widget(next_piece_paragraph, next_piece_area);

    // ASCII Rustris logo in the left panel — each letter colored from the Tetris palette
    let letter_data: [([&str; 4], Color); 7] = [
        (["█▀▄", "██▀", "█ █", "▀ ▀"], Color::Red),      // R
        (["█ █", "█ █", "█ █", "▀█▀"], Color::Cyan),     // U
        (["▄█▀", "▀█▄", "▄▄█", "▀▀ "], Color::Yellow),   // S
        (["▀█▀", " █ ", " █ ", " ▀ "], Color::Green),    // T
        (["█▀▄", "██▀", "█ █", "▀ ▀"], Color::Magenta),  // R
        ([" █ ", " █ ", " █ ", " ▀ "], Color::Blue),     // I
        (["▄█▀", "▀█▄", "▄▄█", "▀▀ "], Color::LightRed), // S
    ];

    let mut logo_lines: Vec<Line> = Vec::new();
    for row in 0..4 {
        let mut spans: Vec<Span> = Vec::new();
        for (i, (rows, color)) in letter_data.iter().enumerate() {
            spans.push(Span::styled(rows[row], Style::default().fg(*color)));
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
        .alignment(Alignment::Center);

    let logo_v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(0),
            Constraint::Length(6),
            Constraint::Min(0),
        ])
        .split(h_chunks[0]);

    let logo_h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(0),
            Constraint::Length(29),
            Constraint::Min(0),
        ])
        .split(logo_v_chunks[1]);

    f.render_widget(logo_paragraph, logo_h_chunks[1]);
}

/// Main draw function called from the game loop.
/// Handles size checking, outer frame, and dispatching to intro/game rendering.
pub fn draw(f: &mut Frame, game_state: &super::GameState, tetris_state: &TetrisState) {
    let size = f.size();

    // Check if terminal is too small
    if size.width < FRAME_WIDTH || size.height < FRAME_HEIGHT {
        let warning = Paragraph::new(format!(
            "Terminal too small!\nRequires at least {}x{}.",
            FRAME_WIDTH, FRAME_HEIGHT
        ))
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center);
        f.render_widget(warning, size);
        return;
    }

    // Horizontal centering
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),              // Left flex
            Constraint::Length(FRAME_WIDTH), // Total wrapper width
            Constraint::Min(0),              // Right flex
        ])
        .split(size);

    // Vertical centering
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),               // Top flex
            Constraint::Length(FRAME_HEIGHT), // Total wrapper height
            Constraint::Min(0),               // Bottom flex
        ])
        .split(h_chunks[1]);

    let outer_frame = Block::default().borders(Borders::ALL);
    f.render_widget(outer_frame, v_chunks[1]);

    let inner_area = v_chunks[1].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    match game_state {
        super::GameState::Intro => intro_field(f, inner_area),
        super::GameState::Game => game_field(f, inner_area, tetris_state),
        _ => {}
    }
}

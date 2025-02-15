use color_eyre::Result;
use keyboard::{buildKeyboard, key_display, Key};
use matrix::{CharMatrix, Position};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::fmt::Debug;

pub mod keyboard;
pub mod matrix;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result?;
    Ok(())
}

struct InteractiveState {
    position: Position<usize>,
    layer: usize,
    text: String,
}

fn run(mut terminal: DefaultTerminal) -> Result<String, std::io::Error> {
    let keyboard = buildKeyboard();
    let mut state = InteractiveState {
        position: Position::<usize> { x: 0, y: 0 },
        layer: 0,
        text: String::new(),
    };
    loop {
        terminal.draw(|frame| render(frame, &keyboard, &state))?;
        let mut delta: Option<Position<i8>> = None; //Position::<i8> {x: 0, y: 0};
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        match keyboard.layers[state.layer].rows[state.position.y].items
                            [state.position.x]
                        {
                            keyboard::Key::Char { c } => {
                                state.text = format!("{}{}", &state.text, c);
                            }
                            keyboard::Key::Layer {
                                display,
                                target_layer,
                            } => {
                                state.layer = target_layer;
                            }
                            keyboard::Key::Confirm { display } => break Ok(state.text.clone()),
                            keyboard::Key::Readline { display } => {}
                        };
                    }
                    KeyCode::Tab => {
                        let current_layer = state.layer;
                        state.layer = match current_layer {
                            0 => 1,
                            1 => 0,
                            _ => 0,
                        };
                    }
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Esc | KeyCode::Backspace => {
                        let len = state.text.len();
                        if len > 0 {
                            state.text.truncate(len - 1);
                        }
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        delta = Some(Position::<i8> { x: -1, y: 0 })
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        delta = Some(Position::<i8> { x: 0, y: 1 })
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        delta = Some(Position::<i8> { x: 0, y: -1 })
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        delta = Some(Position::<i8> { x: 1, y: 0 })
                    }
                    _ => {}
                }
            }
        }

        if let Some(delta) = delta {
            let row_count = keyboard.layers[0].rows.len();
            state.position.y =
                (row_count as i8 + state.position.y as i8 + delta.y) as usize % row_count;
            let col_count = keyboard.layers[0].rows[state.position.y].items.len();
            state.position.x =
                (col_count as i8 + state.position.x as i8 + delta.x) as usize % col_count;
        }
    }
}

fn render(frame: &mut Frame, keyboard: &CharMatrix<Key>, state: &InteractiveState) {
    let pos = &state.position;

    let footer = Text::from_iter([
        Line::from("<q> Quit | <arrows / hjkl> Move"),
        Line::from(format!("Current position: {pos:?}")),
    ]);

    let vertical = Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        //Constraint::Length(footer.height() as u16),
    ]);
    let [text_area, area, /*label_area*/] = vertical.areas(frame.area());
    //frame.render_widget(footer.centered(), label_area);
    frame.render_widget(
        Paragraph::new(state.text.clone())
            .block(
                Block::new()
                    .border_style(Style::default().fg(Color::Gray))
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            )
            .wrap(Wrap { trim: false }),
        text_area,
    );

    let layer = &keyboard.layers[state.layer];

    let keyb_row_constraints: Vec<Constraint> =
        layer.rows.iter().map(|r| Constraint::Fill(1)).collect();
    let keyb_vert = Layout::vertical(keyb_row_constraints).split(area);

    for r in 0..layer.rows.len() {
        let row = &layer.rows[r];
        let keyb_col_constraints: Vec<Constraint> =
            row.items.iter().map(|r| Constraint::Fill(1)).collect();
        let keyb_cols = Layout::horizontal(keyb_col_constraints).split(keyb_vert[r]);
        for c in 0..row.items.len() {
            let (selected, border) = if c == pos.x && r == pos.y {
                (true, BorderType::QuadrantOutside)
            } else {
                (false, BorderType::Rounded)
            };

            let key = &row.items[c];
            let color = key_to_color(key, selected);

            frame.render_widget(
                Paragraph::new(key_display(key))
                    .style(Style::default().fg(color))
                    .alignment(Alignment::Center)
                    .block(
                        Block::new()
                            .border_style(Style::default().fg(color))
                            .border_type(border)
                            .borders(Borders::ALL)
                            .style(Style::default().fg(color)),
                    ),
                keyb_cols[c],
            );
        }
    }
}

fn key_to_color(key: &Key, selected: bool) -> Color {
    match (key, selected) {
        (Key::Char { .. }, true) => Color::White,
        (Key::Char { .. }, false) => Color::Gray,
        (Key::Layer { .. }, true) => Color::LightMagenta,
        (Key::Layer { .. }, false) => Color::Magenta,
        (Key::Confirm { .. }, true) => Color::LightGreen,
        (Key::Confirm { .. }, false) => Color::Green,
    }
}

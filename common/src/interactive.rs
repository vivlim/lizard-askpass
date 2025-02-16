use crate::keyboard::{self, buildKeyboard, key_display, LizardKey};
use crate::matrix::{CharMatrix, Position};
use color_eyre::Result;
use ratatui::prelude::Backend;
use ratatui::termion::event::Key;
use ratatui::termion::input::TermRead;
use ratatui::Terminal;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::io::Read;
use std::{
    fmt::Debug,
    io::{stdout, Write},
};
use tui_textarea::TextArea;

pub fn run<B: Backend, I: Read + TermRead>(terminal: Terminal<B>, input: I) -> Result<String> {
    eprintln!("entered interactive::run()");
    color_eyre::install()?;
    eprintln!("color_eyre installed");
    //let terminal = ratatui::init();
    let result = run_osk(terminal, input);
    eprintln!("run_osk finished");
    ratatui::restore();
    let text = match result? {
        RunResult::Text { text } => text,
        RunResult::MoveToReadline { starting_text } => {
            let terminal = ratatui::init();
            let result = readline(terminal, starting_text);
            ratatui::restore();
            result?
        }
        RunResult::Abort => {
            return Ok(String::new());
        }
    };
    Ok(text)
}

struct InteractiveState {
    position: Position<usize>,
    layer: usize,
    text: String,
    visible: bool,
}

fn run_osk<B: Backend, I: Read + TermRead>(
    mut terminal: Terminal<B>,
    mut input: I,
) -> Result<RunResult> {
    let keyboard = buildKeyboard();
    let mut state = InteractiveState {
        position: Position::<usize> { x: 0, y: 0 },
        layer: 0,
        text: String::new(),
        visible: false,
    };
    let mut result: Option<RunResult> = None;
    terminal.draw(|frame| render(frame, &keyboard, &state))?;
    for k in input.keys() {
        //eprintln!("drawing frame");
        let mut delta: Option<Position<i8>> = None; //Position::<i8> {x: 0, y: 0};
        match k? {
            Key::Char('\n') => {
                match keyboard.layers[state.layer].rows[state.position.y].items[state.position.x] {
                    keyboard::LizardKey::Char { c } => {
                        state.text = format!("{}{}", &state.text, c);
                    }
                    keyboard::LizardKey::Layer {
                        display,
                        target_layer,
                    } => {
                        state.layer = target_layer;
                    }
                    keyboard::LizardKey::Confirm { display } => {
                        result = Some(RunResult::Text {
                            text: state.text.clone(),
                        });
                        break;
                    }
                    keyboard::LizardKey::Readline { display } => {
                        result = Some(RunResult::MoveToReadline {
                            starting_text: if state.visible {
                                state.text.clone()
                            } else {
                                String::new()
                            },
                        });
                        break;
                    }
                    LizardKey::ToggleVisible { display } => {
                        state.visible = !state.visible;
                    }
                };
            }
            Key::Char('\t') => {
                let current_layer = state.layer;
                state.layer = match current_layer {
                    0 => 1,
                    1 => 0,
                    _ => 0,
                };
            }
            Key::Char('q') => {
                result = Some(RunResult::Abort);
                break;
            }
            Key::Esc | Key::Backspace => {
                let len = state.text.len();
                if len > 0 {
                    state.text.truncate(len - 1);
                }
            }
            Key::Char('h') | Key::Left => delta = Some(Position::<i8> { x: -1, y: 0 }),
            Key::Char('j') | Key::Down => delta = Some(Position::<i8> { x: 0, y: 1 }),
            Key::Char('k') | Key::Up => delta = Some(Position::<i8> { x: 0, y: -1 }),
            Key::Char('l') | Key::Right => delta = Some(Position::<i8> { x: 1, y: 0 }),
            _ => {}
        }

        if let Some(delta) = delta {
            let row_count = keyboard.layers[0].rows.len();
            state.position.y =
                (row_count as i8 + state.position.y as i8 + delta.y) as usize % row_count;
            let col_count = keyboard.layers[0].rows[state.position.y].items.len();
            state.position.x =
                (col_count as i8 + state.position.x as i8 + delta.x) as usize % col_count;
        }

        terminal.draw(|frame| render(frame, &keyboard, &state))?;
    }
    Ok(result.expect("result wasn't set"))
}

fn render(frame: &mut Frame, keyboard: &CharMatrix<LizardKey>, state: &InteractiveState) {
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
    let display_text = match state.visible {
        true => state.text.clone(),
        false => "*".repeat(state.text.len()),
    };

    frame.render_widget(
        Paragraph::new(display_text)
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

fn readline(mut term: DefaultTerminal, starting_text: String) -> Result<String> {
    let mut textarea = TextArea::default();
    textarea.insert_str(starting_text);
    textarea.set_cursor_line_style(Style::default());
    textarea.set_style(Style::default().fg(Color::White));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Color::Gray)
            .border_type(BorderType::Rounded)
            .title("Text entry (press Enter to confirm)"),
    );
    let layout = Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)]);
    loop {
        term.draw(|f| {
            let chunks = layout.split(f.area());
            f.render_widget(&textarea, chunks[0]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key {
                KeyEvent {
                    kind: KeyEventKind::Press,
                    code: KeyCode::Enter,
                    ..
                } => break Ok(String::new()),
                k => {
                    textarea.input(k);
                }
            };
        }
    }
}

fn key_to_color(key: &LizardKey, selected: bool) -> Color {
    match (key, selected) {
        (LizardKey::Char { .. }, true) => Color::White,
        (LizardKey::Char { .. }, false) => Color::Gray,
        (LizardKey::Layer { .. }, true) => Color::LightMagenta,
        (LizardKey::Layer { .. }, false) => Color::Magenta,
        (LizardKey::Confirm { .. }, true) => Color::LightGreen,
        (LizardKey::Confirm { .. }, false) => Color::Green,
        (_, true) => Color::LightRed,
        (_, false) => Color::Red,
    }
}

enum RunResult {
    Text { text: String },
    MoveToReadline { starting_text: String },
    Abort,
}

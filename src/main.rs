use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::fmt::Debug;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result?;
    Ok(())
}

#[derive(Debug)]
struct CharMatrix<T>
where
    T: Debug + Default,
{
    layers: Vec<Layer<T>>,
}

#[derive(Debug, Default)]
struct Layer<T>
where
    T: Debug + Default,
{
    rows: Vec<Row<T>>,
}

#[derive(Debug, Default)]
struct Row<T>
where
    T: Debug + Default,
{
    items: Vec<T>,
}

#[derive(Debug, Default)]
struct Dimension<T> {
    w: T,
    h: T,
}

#[derive(Debug, Default)]
struct Position<T> {
    x: T,
    y: T,
}

struct InteractiveState {
    position: Position<usize>,
    layer: usize,
    text: String,
}

fn buildKeyboard() -> CharMatrix<char> {
    CharMatrix::<char> {
        layers: vec![Layer::<char> {
            rows: vec![
                Row::<char> {
                    items: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
                },
                Row::<char> {
                    items: vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
                },
                Row::<char> {
                    items: vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
                },
                Row::<char> {
                    items: vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.'],
                },
            ],
        }],
    }
}

fn run(mut terminal: DefaultTerminal) -> Result<(), std::io::Error> {
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
                        let selected = keyboard.layers[state.layer].rows[state.position.y].items
                            [state.position.x];
                        state.text = format!("{}{}", &state.text, selected);
                    }
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Esc => {
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

fn render(frame: &mut Frame, keyboard: &CharMatrix<char>, state: &InteractiveState) {
    let pos = &state.position;

    let footer = Text::from_iter([
        Line::from("<q> Quit | <arrows / hjkl> Move"),
        Line::from(format!("Current position: {pos:?}")),
    ]);

    let vertical = Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Length(footer.height() as u16),
    ]);
    let [text_area, area, label_area] = vertical.areas(frame.area());
    frame.render_widget(footer.centered(), label_area);
    frame.render_widget(
        Paragraph::new(state.text.clone()).block(
            Block::new()
                .border_style(Style::default().fg(Color::Gray))
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        ),
        text_area,
    );

    let keyb_row_constraints: Vec<Constraint> = keyboard.layers[0]
        .rows
        .iter()
        .map(|r| Constraint::Fill(1))
        .collect();
    let keyb_vert = Layout::vertical(keyb_row_constraints).split(area);

    for r in 0..keyboard.layers[0].rows.len() {
        let row = &keyboard.layers[0].rows[r];
        let keyb_col_constraints: Vec<Constraint> =
            row.items.iter().map(|r| Constraint::Fill(1)).collect();
        let keyb_cols = Layout::horizontal(keyb_col_constraints).split(keyb_vert[r]);
        for c in 0..row.items.len() {
            let (color, border) = if c == pos.x && r == pos.y {
                (Color::LightGreen, BorderType::QuadrantOutside)
            } else {
                (Color::Green, BorderType::Rounded)
            };

            frame.render_widget(
                Paragraph::new(format!("{}", row.items[c]))
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

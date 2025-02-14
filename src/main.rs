use std::fmt::Debug;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::{Color, Style, Stylize}, text::{Line, Text}, widgets::{Block, BorderType, Borders}, DefaultTerminal, Frame
};
use color_eyre::Result;

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
where T: Debug + Default
{
    layers: Vec<Layer<T>>,
}

#[derive(Debug, Default)]
struct Layer<T> where T: Debug + Default { rows: Vec<Row<T>> }

#[derive(Debug, Default)]
struct Row<T> where T: Debug + Default { items: Vec<T> }

#[derive(Debug, Default)]
struct Dimension<T> { w: T, h: T }

#[derive(Debug, Default)]
struct Position<T> { x: T, y: T }

fn buildKeyboard() -> CharMatrix<char> {
    CharMatrix::<char> {
        layers: vec![
            Layer::<char> {
                rows: vec![
                    Row::<char> { items: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'] },
                    Row::<char> { items: vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'] },
                    Row::<char> { items: vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'] },
                    Row::<char> { items: vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.'] },
                ],
            },
        ],
    }
}

fn run(mut terminal: DefaultTerminal) -> Result<(), std::io::Error> {
    let keyboard = buildKeyboard();
    let mut pos: Position<usize> = Position::<usize> { x: 0, y: 0 };
    loop {
        terminal.draw(|frame| render(frame, &keyboard, &pos))?;
        let mut delta: Option<Position<i8>> = None; //Position::<i8> {x: 0, y: 0};
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => break Ok(()),
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Esc => break Ok(()),
                    KeyCode::Char('h') | KeyCode::Left => delta = Some(Position::<i8> { x: -1, y: 0 }),
                    KeyCode::Char('j') | KeyCode::Down => delta = Some(Position::<i8> { x: 0, y: 1 }),
                    KeyCode::Char('k') | KeyCode::Up => delta = Some(Position::<i8> { x: 0, y: -1 }),
                    KeyCode::Char('l') | KeyCode::Right => delta = Some(Position::<i8> { x: 1, y: 0 }),
                    _ => {}
                }
            }
        }

        if let Some(delta) = delta {
            let row_count = keyboard.layers[0].rows.len();
            pos.y = (row_count as i8 + pos.y as i8 + delta.y) as usize % row_count;
            let col_count = keyboard.layers[0].rows[pos.y].items.len();
            pos.x = (col_count as i8 + pos.x as i8 + delta.x) as usize % col_count;
        }

    }
}

fn render(frame: &mut Frame, keyboard: &CharMatrix<char>, pos: &Position<usize>) {
    let header = Text::from_iter([
        Line::from("Calendar Example".bold()),
        Line::from(
            "<q> Quit | <hjkl> Move",
        ),
        Line::from(format!(
            "Current position: {pos:?}"
        )),
    ]);

    let vertical = Layout::vertical([
        Constraint::Length(header.height() as u16),
        Constraint::Fill(1),
    ]);
    let [text_area, area] = vertical.areas(frame.area());
    frame.render_widget(header.centered(), text_area);

    let keyb_row_constraints: Vec<Constraint> = keyboard.layers[0].rows.iter().map(|r| Constraint::Fill(1)).collect();
    let keyb_vert = Layout::vertical(keyb_row_constraints);

    for r in 0..keyboard.layers[0].rows.len() {
        

    }

}


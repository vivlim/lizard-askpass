use std::fmt::Debug;

use crate::matrix::{CharMatrix, Layer, Row};

pub fn buildKeyboard() -> CharMatrix<Key> {
    CharMatrix::<Key> {
        layers: vec![Layer::<Key> {
            rows: vec![
                Row::<Key> {
                    items: vec![
                        Key::Char { c: '1' },
                        Key::Char { c: '2' },
                        Key::Char { c: '3' },
                        Key::Char { c: '4' },
                        Key::Char { c: '5' },
                        Key::Char { c: '6' },
                        Key::Char { c: '7' },
                        Key::Char { c: '8' },
                        Key::Char { c: '9' },
                        Key::Char { c: '0' },
                    ],
                },
                Row::<Key> {
                    items: vec![
                        Key::Char { c: 'q' },
                        Key::Char { c: 'w' },
                        Key::Char { c: 'e' },
                        Key::Char { c: 'r' },
                        Key::Char { c: 't' },
                        Key::Char { c: 'y' },
                        Key::Char { c: 'u' },
                        Key::Char { c: 'i' },
                        Key::Char { c: 'o' },
                        Key::Char { c: 'p' },
                    ],
                },
                Row::<Key> {
                    items: vec![
                        Key::Char { c: 'a' },
                        Key::Char { c: 's' },
                        Key::Char { c: 'd' },
                        Key::Char { c: 'f' },
                        Key::Char { c: 'g' },
                        Key::Char { c: 'h' },
                        Key::Char { c: 'j' },
                        Key::Char { c: 'k' },
                        Key::Char { c: 'l' },
                        Key::Char { c: ';' },
                    ],
                },
                Row::<Key> {
                    items: vec![
                        Key::Char { c: 'z' },
                        Key::Char { c: 'x' },
                        Key::Char { c: 'c' },
                        Key::Char { c: 'v' },
                        Key::Char { c: 'b' },
                        Key::Char { c: 'n' },
                        Key::Char { c: 'm' },
                        Key::Char { c: ',' },
                        Key::Char { c: '.' },
                        Key::Char { c: '/' },
                    ],
                },
                Row::<Key> {
                    items: vec![
                        Key::Layer {
                            display: "shift",
                            target_layer: 1,
                        },
                        Key::Char { c: ' ' },
                        Key::Char { c: '[' },
                        Key::Char { c: ']' },
                        Key::Char { c: '\'' },
                        Key::Char { c: '\\' },
                        Key::Char { c: '-' },
                        Key::Char { c: '+' },
                        Key::Char { c: ' ' },
                        Key::Confirm { display: "OK" },
                    ],
                },
            ],
        }],
    }
}

#[derive(Debug)]
pub enum Key {
    Char {
        c: char,
    },
    Layer {
        display: &'static str,
        target_layer: usize,
    },
    Confirm {
        display: &'static str,
    },
}

pub fn key_display(k: &Key) -> String {
    match k {
        Key::Char { c } => format!("{}", c),
        Key::Layer {
            display,
            target_layer,
        } => display.to_string(),
        Key::Confirm { display } => display.to_string(),
    }
}

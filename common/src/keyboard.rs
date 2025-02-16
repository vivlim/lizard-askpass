use std::fmt::Debug;

use crate::matrix::{CharMatrix, Layer, Row};

pub fn buildKeyboard() -> CharMatrix<LizardKey> {
    CharMatrix::<LizardKey> {
        layers: vec![
            Layer::<LizardKey> {
                rows: vec![
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: '1' },
                            LizardKey::Char { c: '2' },
                            LizardKey::Char { c: '3' },
                            LizardKey::Char { c: '4' },
                            LizardKey::Char { c: '5' },
                            LizardKey::Char { c: '6' },
                            LizardKey::Char { c: '7' },
                            LizardKey::Char { c: '8' },
                            LizardKey::Char { c: '9' },
                            LizardKey::Char { c: '0' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: 'q' },
                            LizardKey::Char { c: 'w' },
                            LizardKey::Char { c: 'e' },
                            LizardKey::Char { c: 'r' },
                            LizardKey::Char { c: 't' },
                            LizardKey::Char { c: 'y' },
                            LizardKey::Char { c: 'u' },
                            LizardKey::Char { c: 'i' },
                            LizardKey::Char { c: 'o' },
                            LizardKey::Char { c: 'p' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: 'a' },
                            LizardKey::Char { c: 's' },
                            LizardKey::Char { c: 'd' },
                            LizardKey::Char { c: 'f' },
                            LizardKey::Char { c: 'g' },
                            LizardKey::Char { c: 'h' },
                            LizardKey::Char { c: 'j' },
                            LizardKey::Char { c: 'k' },
                            LizardKey::Char { c: 'l' },
                            LizardKey::Char { c: ';' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: 'z' },
                            LizardKey::Char { c: 'x' },
                            LizardKey::Char { c: 'c' },
                            LizardKey::Char { c: 'v' },
                            LizardKey::Char { c: 'b' },
                            LizardKey::Char { c: 'n' },
                            LizardKey::Char { c: 'm' },
                            LizardKey::Char { c: ',' },
                            LizardKey::Char { c: '.' },
                            LizardKey::Char { c: '/' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Layer {
                                display: "shift",
                                target_layer: 1,
                            },
                            LizardKey::Char { c: ' ' },
                            LizardKey::Char { c: '[' },
                            LizardKey::Char { c: ']' },
                            LizardKey::Char { c: '\'' },
                            LizardKey::Char { c: '\\' },
                            LizardKey::Char { c: '-' },
                            LizardKey::Char { c: '_' },
                            LizardKey::Char { c: '=' },
                            LizardKey::Confirm { display: "OK" },
                        ],
                    },
                ],
            },
            Layer::<LizardKey> {
                rows: vec![
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: '!' },
                            LizardKey::Char { c: '@' },
                            LizardKey::Char { c: '#' },
                            LizardKey::Char { c: '$' },
                            LizardKey::Char { c: '%' },
                            LizardKey::Char { c: '^' },
                            LizardKey::Char { c: '&' },
                            LizardKey::Char { c: '*' },
                            LizardKey::Char { c: '(' },
                            LizardKey::Char { c: ')' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: 'Q' },
                            LizardKey::Char { c: 'W' },
                            LizardKey::Char { c: 'E' },
                            LizardKey::Char { c: 'R' },
                            LizardKey::Char { c: 'T' },
                            LizardKey::Char { c: 'Y' },
                            LizardKey::Char { c: 'U' },
                            LizardKey::Char { c: 'I' },
                            LizardKey::Char { c: 'O' },
                            LizardKey::Char { c: 'P' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: 'A' },
                            LizardKey::Char { c: 'S' },
                            LizardKey::Char { c: 'D' },
                            LizardKey::Char { c: 'F' },
                            LizardKey::Char { c: 'G' },
                            LizardKey::Char { c: 'H' },
                            LizardKey::Char { c: 'J' },
                            LizardKey::Char { c: 'K' },
                            LizardKey::Char { c: 'L' },
                            LizardKey::Char { c: ':' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Char { c: 'Z' },
                            LizardKey::Char { c: 'X' },
                            LizardKey::Char { c: 'C' },
                            LizardKey::Char { c: 'V' },
                            LizardKey::Char { c: 'B' },
                            LizardKey::Char { c: 'N' },
                            LizardKey::Char { c: 'M' },
                            LizardKey::Char { c: '<' },
                            LizardKey::Char { c: '>' },
                            LizardKey::Char { c: '?' },
                        ],
                    },
                    Row::<LizardKey> {
                        items: vec![
                            LizardKey::Layer {
                                display: "SHIFT",
                                target_layer: 0,
                            },
                            LizardKey::Char { c: ' ' },
                            LizardKey::Char { c: '{' },
                            LizardKey::Char { c: '}' },
                            LizardKey::Char { c: '"' },
                            LizardKey::Char { c: '|' },
                            LizardKey::Char { c: '+' },
                            LizardKey::ToggleVisible { display: "show" },
                            LizardKey::Readline { display: "edit" },
                            LizardKey::Confirm { display: "OK" },
                        ],
                    },
                ],
            },
        ],
    }
}

#[derive(Debug)]
pub enum LizardKey {
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
    ToggleVisible {
        display: &'static str,
    },
    Readline {
        display: &'static str,
    },
}

pub fn key_display(k: &LizardKey) -> String {
    match k {
        LizardKey::Char { c } => format!("{}", c),
        LizardKey::Layer {
            display,
            target_layer,
        } => display.to_string(),
        LizardKey::Confirm { display } => display.to_string(),
        LizardKey::Readline { display } => display.to_string(),
        LizardKey::ToggleVisible { display } => display.to_string(),
    }
}

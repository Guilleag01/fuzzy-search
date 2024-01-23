use std::{
    // io::stdout,
    sync::{Arc, RwLock},
    thread,
};

use crossterm::{
    self,
    event::{read, KeyEvent},
};

pub fn start_get_input(
    width: usize,
    buffer: Arc<RwLock<(Vec<char>, String)>>,
    cont_arc: Arc<RwLock<bool>>,
    cursor_arc: Arc<RwLock<usize>>,
) {
    thread::spawn(move || get_input(width, buffer, cont_arc, cursor_arc));
}

fn get_input(
    width: usize,
    buffer: Arc<RwLock<(Vec<char>, String)>>,
    cont_arc: Arc<RwLock<bool>>,
    cursor_arc: Arc<RwLock<usize>>,
) {
    let mut input: Vec<char> = Vec::new();
    let mut cursor: usize = 0;
    let mut cont = true;
    let mut window_offset: usize = 0;

    while cont {
        if let crossterm::event::Event::Key(ke) = read().unwrap() {
            handle_ke(
                ke,
                &mut input,
                &mut cursor,
                &mut cont,
                &mut window_offset,
                width,
            );

            let text = String::from_iter(
                &input.clone()[window_offset..(window_offset + width - 4).min(input.len())],
            );

            let spaces = String::from_iter(vec![' '; (width - 4) - text.chars().count()]);

            buffer.write().unwrap().0 = input.clone();
            buffer.write().unwrap().1 = format!("{}{}", text, spaces);

            *cont_arc.write().unwrap() = cont;

            *cursor_arc.write().unwrap() = cursor - window_offset;
        }
    }
}

fn handle_ke(
    ke: KeyEvent,
    input: &mut Vec<char>,
    cursor: &mut usize,
    cont: &mut bool,
    window_offset: &mut usize,
    width: usize,
) {
    match ke.code {
        crossterm::event::KeyCode::Esc => {
            *cont = false;
        }
        crossterm::event::KeyCode::Backspace => {
            if *cursor > 0 {
                input.remove(*cursor - 1);
                if *cursor > 0 {
                    if *cursor == *window_offset && *cursor > 0 {
                        *window_offset -= 1;
                    }
                    *cursor -= 1;
                }
            }
        }
        crossterm::event::KeyCode::Left => {
            if *cursor > 0 {
                if *cursor == *window_offset {
                    *window_offset -= 1;
                }
                *cursor -= 1;
            }
        }
        crossterm::event::KeyCode::Right => {
            if *cursor < input.len() {
                if *cursor == *window_offset + (width - 4) {
                    *window_offset += 1;
                }
                *cursor += 1;
            }
        }
        // crossterm::event::KeyCode::Up => todo!(),
        // crossterm::event::KeyCode::Down => todo!(),
        crossterm::event::KeyCode::Home => {
            *cursor = 0;
            *window_offset = 0;
        }
        crossterm::event::KeyCode::End => {
            *cursor = input.len();
            if *cursor > *window_offset + (width - 4) {
                *window_offset = input.len() - (width - 4);
            }
        }
        crossterm::event::KeyCode::Delete => {
            if *cursor < input.len() {
                input.remove(*cursor);
            }
        }
        crossterm::event::KeyCode::Char(c) => {
            input.insert(*cursor, c);
            *cursor += 1;
            if *cursor > width - 5 {
                *window_offset += 1;
            }
        }
        // crossterm::event::KeyCode::CapsLock => todo!(),
        // crossterm::event::KeyCode::Modifier(_) => todo!(),
        _ => {}
    }
}

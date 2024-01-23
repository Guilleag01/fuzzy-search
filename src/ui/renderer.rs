use std::{
    io::stdout,
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, Show},
    execute, queue,
    style::Print,
    terminal,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::fuzzy_search::test;

use super::input::start_get_input;

pub fn start() -> (usize, usize) {
    let size = terminal::size().unwrap();
    let width = size.0 as usize;
    let height = size.1 as usize;

    terminal::enable_raw_mode().unwrap();

    queue!(
        stdout(),
        Clear(ClearType::All),
        EnterAlternateScreen,
        MoveTo(0, 0),
    )
    .unwrap();

    print_frame(width, height);

    execute!(stdout(), MoveTo(2, 1)).unwrap();

    (width, height)
}

pub fn run() {
    let size = terminal::size().unwrap();

    let input: Arc<RwLock<(Vec<char>, String)>> =
        Arc::new(RwLock::new((Vec::new(), String::new())));

    let cont: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));

    let cursor: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));

    let results: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));

    start_get_input(size.0 as usize, input.clone(), cont.clone(), cursor.clone());

    let mut search_thread = test(input.clone(), results.clone());

    let mut last_input: Vec<char> = Vec::new();

    while cont.read().unwrap().to_owned() {
        if input.read().unwrap().clone().0 != last_input && search_thread.is_finished() {
            search_thread = test(input.clone(), results.clone());
        }
        for (i, res) in results.read().unwrap().iter().enumerate() {
            queue!(stdout(), MoveTo(2, 3 + i as u16), Print(res)).unwrap();
        }
        queue!(
            stdout(),
            MoveTo(2, 1),
            Print(input.read().unwrap().clone().1),
            MoveTo(cursor.read().unwrap().to_owned() as u16 + 2, 1)
        )
        .unwrap();
        execute!(stdout()).unwrap();
        sleep(Duration::from_millis(1));
        last_input = input.read().unwrap().0.to_owned();
    }
}

pub fn stop() {
    terminal::disable_raw_mode().unwrap();
    execute!(stdout(), Show, LeaveAlternateScreen,).unwrap();
}

fn print_frame(width: usize, height: usize) {
    let spaces = String::from_iter(vec![' '; width - 2]);
    let lines = String::from_iter(vec!['─'; width - 2]);

    queue!(
        stdout(),
        MoveTo(0, 0),
        Print("╭"),
        Print(lines.clone()),
        Print("╮"),
        MoveTo(0, 1),
        Print("│"),
        Print(spaces.clone()),
        Print("│"),
        MoveTo(0, 2),
        Print("├"),
        Print(lines.clone()),
        Print("┤")
    )
    .unwrap();

    for i in 0..(height - 4) {
        queue!(
            stdout(),
            MoveTo(0, i as u16 + 3),
            Print("│"),
            Print(spaces.clone()),
            Print("│")
        )
        .unwrap();
    }

    queue!(stdout(), Print("╰"), Print(lines.clone()), Print("╯")).unwrap();

    execute!(stdout()).unwrap();
}
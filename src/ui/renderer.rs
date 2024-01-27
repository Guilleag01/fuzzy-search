use std::{
    fs::{metadata, read_dir},
    io::stdout,
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
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

pub fn run(path: &str, recursive: usize) {
    let size = terminal::size().unwrap();
    let input: Arc<RwLock<(Vec<char>, String)>> =
        Arc::new(RwLock::new((Vec::new(), String::new())));
    let cont: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
    let cursor: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let results: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));
    start_get_input(size.0 as usize, input.clone(), cont.clone(), cursor.clone());

    execute!(stdout(), MoveTo(2, 3), Print("Reading files...")).unwrap();
    let mut list = Vec::new();
    get_files(&mut list, path, recursive);
    execute!(stdout(), MoveTo(2, 3), Print("                ")).unwrap();

    let mut search_thread = test(
        list.clone(),
        input.clone(),
        results.clone(),
        size.1 as usize - 4,
    );
    let mut processing = false;
    let mut last_input = String::new();

    let mut loading_message = String::from("Loading...");
    loading_message.push_str(String::from_iter(vec![' '; size.0 as usize - (4 + 10)]).as_str());
    let clear_loading = String::from_iter(vec![' '; size.0 as usize - 4]);

    while cont.read().unwrap().to_owned() {
        let input_buff = input.read().unwrap().0.clone();
        if String::from_iter(input.read().unwrap().clone().0).as_str() != last_input.as_str() {
            search_thread = test(
                list.clone(),
                input.clone(),
                results.clone(),
                size.1 as usize - 4,
            );
            processing = true;
            execute!(stdout(), MoveTo(2, 3), Print(loading_message.clone())).unwrap();
        }

        if processing && search_thread.is_finished() {
            let res_list = results.read().unwrap();
            execute!(stdout(), Hide, MoveTo(2, 3), Print(clear_loading.clone())).unwrap();

            for (i, res) in res_list[0..(size.1 as usize - 4).min(res_list.len())]
                .iter()
                .enumerate()
            {
                let spaces =
                    String::from_iter(vec![' '; (size.0 as usize - 4) - res.chars().count()]);
                queue!(
                    stdout(),
                    MoveTo(2, 3 + i as u16),
                    Print(format!("{}{}", res, spaces)),
                    // MoveTo(2, 3),
                    // Print(format!("{} ", res_list.len())),
                )
                .unwrap();
            }
            queue!(stdout(), Show).unwrap();
            processing = false;
        }

        queue!(
            stdout(),
            MoveTo(2, 1),
            Print(input.read().unwrap().clone().1),
            MoveTo(cursor.read().unwrap().to_owned() as u16 + 2, 1),
        )
        .unwrap();

        execute!(stdout()).unwrap();

        last_input = String::from_iter(input_buff);
        sleep(Duration::from_millis(1));
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

fn get_files(list: &mut Vec<String>, path: &str, recursive: usize) {
    // let mut list2 = list.clone();

    let path_format = if path == "." {
        "".to_string()
    } else {
        format!("{}/", path)
    };

    if recursive < 1 {
        if let Ok(file_list) = read_dir(path) {
            list.append(
                &mut file_list
                    .map(|x| {
                        format!(
                            "{}{}",
                            path_format,
                            x.unwrap().file_name().to_str().unwrap()
                        )
                    })
                    .collect(),
            );
        }
    } else {
        for e in read_dir(path)
            .unwrap()
            .map(|x| x.unwrap().file_name().to_str().unwrap().to_string())
            .collect::<Vec<String>>()
        {
            let e_path = format!("{}{}", path_format, e);
            list.push(e_path.clone());

            if let Ok(met) = metadata(e_path.clone()) {
                if met.is_dir() {
                    get_files(list, e_path.as_str().clone(), recursive - 1);
                }
            }
        }
    }
}

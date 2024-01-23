use std::{
    // io::stdout,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

use self::fuzzy::fzs;

pub mod fuzzy;

pub fn test(
    list: Vec<String>,
    input: Arc<RwLock<(Vec<char>, String)>>,
    result: Arc<RwLock<Vec<String>>>,
) -> JoinHandle<()> {
    // let list = [
    //     "pear",
    //     "tree",
    //     "platypus",
    //     "building",
    //     "test",
    //     "lime",
    //     "stationary",
    // ];

    // let list = read_dir(".")
    //     .unwrap()
    //     .map(|x| x.unwrap().file_name().to_str().unwrap().to_string())
    //     .collect::<Vec<String>>();

    let mut list2 = list;

    thread::spawn(move || {
        let pattern = String::from_iter(input.read().unwrap().0.clone());

        fzs(pattern.as_str(), &mut list2);
        *result.write().unwrap() = list2;
    })
}

use std::{
    // io::stdout,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle}, // time::Duration,
};

use self::fuzzy::fzs;

pub mod fuzzy;

pub fn test(
    input: Arc<RwLock<(Vec<char>, String)>>,
    result: Arc<RwLock<Vec<String>>>,
) -> JoinHandle<()> {
    let list = [
        "pear",
        "tree",
        "platypus",
        "building",
        "test",
        "lime",
        "stationary",
    ];

    let mut list2 = list;

    thread::spawn(move || {
        let pattern = String::from_iter(input.read().unwrap().0.clone());

        fzs(pattern.as_str(), &mut list2);
        *result.write().unwrap() = list2.map(|x| x.to_string()).to_vec();
    })
}

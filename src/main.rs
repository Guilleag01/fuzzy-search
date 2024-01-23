use fuzzy_search::ui::renderer;

fn main() {
    // let list = [
    //     "pear",
    //     "tree",
    //     "platypus",
    //     "building",
    //     "test",
    //     "lime",
    //     "stationary",
    // ];

    // let mut result = list;

    // fzs("aaa", &mut result);

    // println!("{:?}", result);

    renderer::start();
    renderer::run();
    renderer::stop();
}

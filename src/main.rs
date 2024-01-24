use clap::Parser;
use fuzzy_search::ui::renderer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Print contents of directories recursively,
    /// specify maximum recursive depth
    #[arg(short, long, default_value_t = 0)]
    recursive: usize,

    /// Path of the directory to list
    #[arg(default_value_t = String::from("."))]
    path: String,
}

fn main() {
    let args = Args::parse();

    renderer::start();
    renderer::run(&args.path, args.recursive);
    renderer::stop();
}

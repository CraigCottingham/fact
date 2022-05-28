use clap::Parser;
use clap::Subcommand;

/// A simple command-line utility for exploring Factorio <https://factorio.com/>.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Subcommand
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    CraftTree,
}

fn main() {
    let args = Args::parse();
    // println!("{:#?}", args);

    match args.action {
        Action::CraftTree => craft_tree("/Users/ccottingham/Library/Application Support/Steam/steamapps/common/Factorio/factorio.app/Contents/data/base/prototypes/recipe.lua".to_string()),
    }
}

fn craft_tree(path: String) {
    println!("trying to read {}", path);
    let content = std::fs::read_to_string(path).expect("could not read file");
    for line in content.lines() {
        println!("{}", line);
    }
}

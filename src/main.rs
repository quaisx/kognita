/*
 _        _______  _______  _       __________________ _______
| \    /\(  ___  )(  ____ \( (    /|\__   __/\__   __/(  ___  )
|  \  / /| (   ) || (    \/|  \  ( |   ) (      ) (   | (   ) |
|  (_/ / | |   | || |      |   \ | |   | |      | |   | (___) |
|   _ (  | |   | || | ____ | (\ \) |   | |      | |   |  ___  |
|  ( \ \ | |   | || | \_  )| | \   |   | |      | |   | (   ) |
|  /  \ \| (___) || (___) || )  \  |___) (___   | |   | )   ( |
|_/    \/(_______)(_______)|/    )_)\_______/   )_(   |/     \|

@authors: free thinkers of the world
    1. Qua Is X (Ukraine) qua.is.kyiv.ua@gmail.com
    /add your name here.../

 */

use std::error::Error;
use cli::args::parse_cli;
use node::run;
use cfg::load;

mod cli;
mod node;
mod cfg;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_cli();
    // load node configuration file (.toml)
    let result = load::load_node_config(&args.config);
    if let Err(x) =  result {
        eprintln!("{:#?}", x);
        std::process::exit(1);
    } else {
        let node_config = result.unwrap();
        println!("<ARGS> {args}");
        run::run(&args, node_config).await
    }
}

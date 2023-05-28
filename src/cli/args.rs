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

use clap::Parser;

#[derive(Parser)]
#[command(name = "Kognita")]
#[command(author = "Qua Is X")]
#[command(version = "1.0")]
#[command(
    about = "Kognita crypto platform", 
    long_about = "Kognita is an open source project implemented in Rust"
)]
pub struct NodeCliArgs {
    #[arg(long)]
    pub node: String,
}

pub fn parse_args() -> NodeCliArgs {
   NodeCliArgs::parse()
}
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
mod node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    node::run().await
}

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
use std::{env, path::PathBuf};
use tonic_build;

// fn main() {
//    tonic_build::compile_protos("proto/message.proto")
//        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tonic_build::configure()
//          .build_server(false)
//          .compile(
//              &["proto/message.proto"],
//              &["proto"],
//          )?;
//     Ok(())
//  }

 fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("message"))
        .compile(&["crates/p2p/proto/message.proto"], &["message"])
        .unwrap();
    Ok(())
 }
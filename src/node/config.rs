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

use emojis;
use once_cell;

static STR_ID: &str = "identification_card";
static STR_WARN: &str = "warning";
static STR_PING: &str = "ping_pong";
static STR_INTR: &str = "left_right_arrow";
static STR_DISC: &str = "globe_with_meridians";
static STR_EVT: &str = "incoming_envelope";
static STR_ROCK: &str = "rocket";
static STR_PLUG: &str = "electric_plug";
static STR_ERR: &str = "collision";
static STR_DIAL: &str = "telephone_receiver";

pub static E_ID: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_ID).unwrap());
pub static E_WARN: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_WARN).unwrap());
pub static E_PING: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_PING).unwrap());
pub static E_INTR: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_INTR).unwrap());
pub static E_DISC: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_DISC).unwrap());
pub static E_EVT: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_EVT).unwrap());
pub static E_ROCK: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_ROCK).unwrap());
pub static E_PLUG: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_PLUG).unwrap());
pub static E_ERR: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_ERR).unwrap());
pub static E_DIAL: once_cell::sync::Lazy<&emojis::Emoji> =
    once_cell::sync::Lazy::new(|| emojis::get_by_shortcode(&STR_DIAL).unwrap());
use emojis;
use once_cell;
pub const PING_INTERVAL: u64 = 30;

pub const HEARTBEAT_INTERVAL: u64 = 15; // gossibsub hb interval in seconds

static STR_ID: &str = "identification_card";
static STR_WARN: &str = "warning";
static STR_PING: &str = "ping_pong";
static STR_INTR: &str = "left_right_arrow";
static STR_DISC: &str = "globe_with_meridians";
static STR_EVT: &str = "incoming_envelope";
static STR_ROCK: &str = "rocket";

pub static E_ID: once_cell::sync::Lazy<&emojis::Emoji> = once_cell::sync::Lazy::new( || emojis::get_by_shortcode(&STR_ID).unwrap());
pub static E_WARN: once_cell::sync::Lazy<&emojis::Emoji> = once_cell::sync::Lazy::new( || emojis::get_by_shortcode(&STR_WARN).unwrap());
pub static E_PING: once_cell::sync::Lazy<&emojis::Emoji> = once_cell::sync::Lazy::new( || emojis::get_by_shortcode(&STR_PING).unwrap());
pub static E_INTR: once_cell::sync::Lazy<&emojis::Emoji> = once_cell::sync::Lazy::new( || emojis::get_by_shortcode(&STR_INTR).unwrap());
pub static E_DISC: once_cell::sync::Lazy<&emojis::Emoji> = once_cell::sync::Lazy::new( || emojis::get_by_shortcode(&STR_DISC).unwrap());
pub static E_EVT: once_cell::sync::Lazy<&emojis::Emoji> = once_cell::sync::Lazy::new( || emojis::get_by_shortcode(&STR_EVT).unwrap());
pub static E_ROCK: once_cell::sync::Lazy<&emojis::Emoji> = once_cell::sync::Lazy::new( || emojis::get_by_shortcode(&STR_ROCK).unwrap());
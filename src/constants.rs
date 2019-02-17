pub const TITLE: &str = "Git Interactive Rebase Tool";
pub const TITLE_LENGTH: i32 = 27;
pub const TITLE_SHORT: &str = "Git Rebase";
pub const TITLE_SHORT_LENGTH: i32 = 10;
pub const TITLE_HELP_INDICATOR: &str = "Help: ?";
pub const TITLE_HELP_INDICATOR_LENGTH: i32 = 7;

pub const FOOTER_FULL: &str = " up, down, q/Q, w/W, c, j, k, p, r, e, s, f, d, !, ?";
pub const FOOTER_FULL_WIDTH: i32 = 52;
pub const FOOTER_COMPACT: &str = "up,dn.q/Q,w/W,c,j,k,p,r,e,s,f,d,!,?";
pub const FOOTER_COMPACT_WIDTH: i32 = 35;

pub const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue\n";
pub const MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH: i32 = 45;
pub const SHORT_ERROR_MESSAGE: &str = "Window too small\n";
pub const SHORT_ERROR_MESSAGE_WIDTH: i32 = 16;

pub const MINIMUM_WINDOW_HEIGHT: i32 = 5; // title + pad top + line + pad bottom + help
pub const MINIMUM_COMPACT_WINDOW_WIDTH: i32 = 20; //">s ccc mmmmmmmmmmmmm".len()
pub const MINIMUM_FULL_WINDOW_WIDTH: i32 = 34; // " > squash cccccccc mmmmmmmmmmmmm %".len()

pub const TO_FILE_INDICATOR: &str = " -> ";
pub const TO_FILE_INDICATOR_SHORT: &str = ">";

pub const NAME: &str = "interactive-rebase-tool";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const HELP_LINES: &[(&str, &str)] = &[
	("Up", "Move selection up"),
	("Down", "Move selection down"),
	("PgUp", "Move selection up 5 lines"),
	("PgDn", "Move selection down 5 lines"),
	("q", "Abort interactive rebase"),
	("Q", "Immediately abort interactive rebase"),
	("w", "Write interactive rebase file"),
	("W", "Immediately write interactive rebase file"),
	("?", "Show help"),
	("c", "Show commit information"),
	("j", "Move selected commit down"),
	("k", "Move selected commit up"),
	("p", "Set selected commit to be picked"),
	("r", "Set selected commit to be reworded"),
	("e", "Set selected commit to be edited"),
	("s", "Set selected commit to be squashed"),
	("f", "Set selected commit to be fixed-up"),
	("d", "Set selected commit to be dropped"),
	("!", "Open the todo file in the default editor"),
];

pub const EXIT_CODE_GOOD: i32 = 0;
pub const EXIT_CODE_CONFIG_ERROR: i32 = 1;
pub const EXIT_CODE_FILE_READ_ERROR: i32 = 2;
pub const EXIT_CODE_FILE_WRITE_ERROR: i32  = 3;
pub const EXIT_CODE_WRITE_ERROR: i32 = 4;
pub const EXIT_CODE_STATE_ERROR: i32 = 5;

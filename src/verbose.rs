use std::sync::OnceLock;

/// Global verbose flag state
static VERBOSE: OnceLock<bool> = OnceLock::new();

/// Initialize the verbose flag
pub fn init_verbose(verbose: bool) {
    VERBOSE
        .set(verbose)
        .expect("Verbose flag already initialized");
}

/// Check if verbose mode is enabled
pub fn is_verbose() -> bool {
    *VERBOSE.get().unwrap_or(&false)
}

/// Print a formatted message in verbose mode
#[macro_export]
macro_rules! vprintln {
    ($($arg:tt)*) => {
        if $crate::verbose::is_verbose() {
            println!("[VERBOSE] {}", format!($($arg)*));
        }
    };
}

/// Print a formatted message in info mode
#[macro_export]
macro_rules! iprintln {
    ($($arg:tt)*) => {
        println!("[INFO]    {}", format!($($arg)*));
    };
}

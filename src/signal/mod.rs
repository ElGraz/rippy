use std::sync::atomic::{AtomicBool, Ordering};

/// Shared flag indicating whether Ctrl+C was pressed.
/// Use `is_interrupted()` to check during long-running operations.
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Sets the interrupt flag when Ctrl+C is received.
fn handle_interrupt() {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

/// Registers a signal handler that sets the interrupt flag on Ctrl+C.
/// This should be called early in `main()` before starting any long-running operations.
pub fn init_handler() {
    ctrlc::set_handler(move || {
        handle_interrupt();
    })
    .expect("Error setting Ctrl+C handler");
}

/// Returns whether the user pressed Ctrl+C.
/// The flag persists once set — subsequent calls will always return `true`.
pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

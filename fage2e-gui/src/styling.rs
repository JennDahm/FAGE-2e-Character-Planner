
/// Returns the CSS class to use for an advancement whose "completeness" status
/// is given.
pub fn class_for_completeness(completeness: Result<bool, ()>) -> &'static str {
    match completeness {
        Ok(true) => "",
        Ok(false) => "incomplete",
        Err(()) => "error",
    }
}

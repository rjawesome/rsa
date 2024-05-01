use std::{error::Error, fmt::{Display, Formatter, Result}};

#[derive(Debug, Clone)]
pub struct InvalidFileError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl Display for InvalidFileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "invalid private/public key file given")
    }
}
impl Error for InvalidFileError {

}
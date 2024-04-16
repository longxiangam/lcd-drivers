
use crate::traits;


#[allow(dead_code)]
#[derive(Copy, Clone)]
pub(crate) enum Command {
    
}

impl crate::Command for Command {
    /// Returns the address of the command
    fn address(self) -> u8 {
        self as u8
    }
}

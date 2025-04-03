use log::debug;
use thiserror::Error;
pub mod networking;
pub mod schemas;
pub mod crypto;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn current_dir() -> Result<(), FSError> {
    let path = std::env::current_dir();

    match path {
        Ok(path) => {
            debug!("The current directory is {}", path.display());
            Ok(())
        }
        Err(_) => Err(FSError::NoCurrentDir),
    }
}

#[derive(Error, Debug)]
pub enum FSError {
    #[error(
        "Can't access current directory: either directory invalid or app has insufficient permissions"
    )]
    NoCurrentDir,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
//}

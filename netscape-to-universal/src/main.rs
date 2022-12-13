use netscape_to_universal::{convert, read_path};
use std::{
    env,
    io::{self, Error, ErrorKind},
    path::PathBuf,
};

fn main() -> Result<(), Error> {
    let input_path = match env::args_os().nth(1) {
        Some(ref input) => PathBuf::from(input),
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Usage: netscape-to-univeral <input_file>",
            ));
        }
    };

    match read_path(input_path) {
        Ok(mut input) => convert(&mut input, &mut io::stdout()),
        Err(err) => Err(err),
    }
}

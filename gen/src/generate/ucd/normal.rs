use std::io::{self, Read, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeVersion};

pub fn generate<P: AsRef<Path>>(
    dir: P,
    version: &UnicodeVersion,
    data: &UnicodeData,
) -> io::Result<()>
{
    Ok(())
}
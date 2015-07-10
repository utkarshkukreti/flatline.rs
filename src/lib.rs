use std::io;

pub struct Flatline<R: io::Read, W: io::Write> {
    reader: R,
    writer: W
}

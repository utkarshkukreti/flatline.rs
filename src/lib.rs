use std::io;

pub struct Flatline<R: io::Read, W: io::Write> {
    reader: R,
    writer: W
}

impl Flatline<io::Stdin, io::Stdout> {
    pub fn with_stdio() -> Flatline<io::Stdin, io::Stdout> {
        Flatline {
            reader: io::stdin(),
            writer: io::stdout()
        }
    }
}

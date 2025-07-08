use std::io::{self, BufRead, Read, Write};
use std::net::{TcpStream};

pub struct TcpStreamWriter {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl TcpStreamWriter {
    /// Encapsulate a TcpStream with reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    /// Write this line (with a '\n' suffix) to the TcpStream
    pub fn send_message(&mut self, message: &[u8]) -> io::Result<()> {
        self.writer.write_all(&message)?;
        // This will also signal a `writer.flush()` for us!
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    /// Read a received message from the TcpStream
    pub fn read_message(&mut self) -> io::Result<Vec<u8>> {
        let mut line: Vec<u8> = Vec::new();
        // Use `BufRead::read_line()` to read a line from the TcpStream
        // self.reader.read(line.as_slice())?;
        self.reader.read(&mut line);
        line.pop(); // Drop the trailing "\n"
        Ok(line)
    }
}

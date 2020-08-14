//! Library to decode ea .big files
//!
//! ```rust
//! use std::fs::File;
//! use std::io;
//!
//! fn main() -> io::Result<()> {
//!     let file = File::open("./example.big");
//!     let (header, entries) = ea_big::from_reader(&file)?;
//!
//!     let embed = ea_big::open_file(&file, &entries[0]);
//!
//!     Ok(())   
//! }
//! ```
use std::ffi::CString;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

/// Header of the BIG file
#[derive(Debug, Clone)]
pub struct Header {
    /// Header name
    pub name: String,
    /// File size
    pub size: u32,
    /// Embedded files
    pub files: u32,
    /// Index table size
    pub indices: u32,
}

impl Header {
    /// Decodes the Header from a [`Reader`](https://doc.rust-lang.org/std/io/trait.Read.html)
    pub fn from_reader<M: Read>(mut reader: M) -> Result<Self> {
        let name = {
            let mut buf = vec![0; 4];
            reader.read_exact(&mut buf)?;
            String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))?
        };

        let size = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        let files = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            u32::from_be_bytes(buf)
        };

        let indices = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            u32::from_be_bytes(buf)
        };

        Ok(Header {
            name,
            size,
            files,
            indices,
        })
    }
}

/// A entry in the table of indices
#[derive(Debug, Clone)]
pub struct TableEntry {
    /// position of the embedded file within the big file
    pub pos: u32,
    /// size of the embedded file
    pub size: u32,
    /// embedded file name
    pub name: String,
}

impl TableEntry {
    /// Decodes a table entry from a [`Reader`](https://doc.rust-lang.org/std/io/trait.Read.html)
    ///
    /// The [`Reader`](https://doc.rust-lang.org/std/io/trait.Read.html) must be at the start of an entry
    pub fn from_reader<M: Read>(mut reader: M) -> Result<Self> {
        let pos = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            u32::from_be_bytes(buf)
        };

        let size = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            u32::from_be_bytes(buf)
        };

        let bytes = reader
            .bytes()
            .scan((), |_, opt| opt.ok())
            .take_while(|&n| n != 0)
            .collect::<Vec<u8>>();

        let name = CString::new(bytes)?.to_string_lossy().into_owned();

        Ok(TableEntry { pos, size, name })
    }
}

/// Helper struct to read a file embedded in the big file
pub struct EmbeddedFile<M: Read + Seek> {
    reader: M,
    offset: u64,
    size: u64,
    cursor: u64,
}

impl<M: Read + Seek> EmbeddedFile<M> {
    fn new(reader: M, offset: u64, size: u64) -> Self {
        EmbeddedFile {
            reader,
            offset,
            size,
            cursor: 0,
        }
    }
}

impl<M: Read + Seek> Read for EmbeddedFile<M> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader
            .seek(SeekFrom::Start(self.offset + self.cursor))?;

        let len = buf.len().min((self.size - self.cursor) as usize);

        self.reader.read(&mut buf[..len])
    }
}

impl<M: Read + Seek> Seek for EmbeddedFile<M> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let offset = match pos {
            SeekFrom::Start(pos) => pos as i64,
            SeekFrom::End(pos) => self.size as i64 + pos,
            SeekFrom::Current(pos) => self.cursor as i64 + pos,
        };

        if offset < 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                String::from("Cannot seek behind 0"),
            ));
        }

        self.cursor = offset as u64;

        Ok(self.cursor)
    }
}

/// Decodes the header and the indices table from a [`Reader`](https://doc.rust-lang.org/std/io/trait.Read.html)
pub fn from_reader<M: Read>(mut reader: M) -> Result<(Header, Vec<TableEntry>)> {
    let header = Header::from_reader(&mut reader)?;

    let mut entries = Vec::with_capacity(header.files as usize);

    for _ in 0..header.files as usize {
        let entry = TableEntry::from_reader(&mut reader)?;

        entries.push(entry);
    }

    Ok((header, entries))
}

/// Returns a [`EmbeddedFile`](struct.EmbeddedFile.html) representing the [`TableEntry`](struct.TableEntry.html)
pub fn open_file<M: Read + Seek>(reader: M, entry: &TableEntry) -> EmbeddedFile<M> {
    EmbeddedFile::new(reader, entry.pos as u64, entry.size as u64)
}

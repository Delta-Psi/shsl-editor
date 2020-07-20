//! WAD files are the highest level container for game data.

use std::io::{prelude::*, SeekFrom};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use error_chain::{error_chain, bail};
error_chain! {
    foreign_links {
        Io(::std::io::Error);
        InvalidString(::std::string::FromUtf8Error);
    }

    errors {
        UnknownPath(p: String)
    }
}

pub mod header {
    use std::io::{prelude::*, SeekFrom, BufReader};
    use byteorder::{ByteOrder, ReadBytesExt, LittleEndian as LE};
    use super::*;

    #[derive(Debug)]
    pub struct Header {
        pub version: [u32; 2],
        pub files: Vec<FileEntry>,
        pub dirs: Vec<DirEntry>,
        pub size: u64,
    }

    #[derive(Debug)]
    pub struct FileEntry {
        pub entry_offset: u64,
        pub path: String,
        pub size: u64,
        pub offset: u64,
    }

    #[derive(Debug)]
    pub struct DirEntry {
        pub entry_offset: u64,
        pub path: String,
        pub subfiles: Vec<SubfileEntry>,
    }

    #[derive(Debug)]
    pub struct SubfileEntry {
        pub entry_offset: u64,
        pub name: String,
        pub is_directory: bool,
    }

    impl Header {
        pub fn read_from<R: Read + Seek>(reader: R) -> Result<Self> {
            // buffer i/o calls
            let mut reader = BufReader::new(reader);

            // store the offset at which the header begins (usually 0)
            let begin = reader.seek(SeekFrom::Current(0))?;

            // read the first 16 bytes
            let mut buf = vec![0; 16];
            reader.read_exact(&mut buf)?;

            // check if the magic bytes are present
            if &buf[0..4] != b"AGAR" {
                bail!("invalid magic bytes");
            }
            let version = [LE::read_u32(&buf[4..8]), LE::read_u32(&buf[8..12])];

            // now, read file metadata
            let file_count = reader.read_u32::<LE>()?;
            let mut files = Vec::with_capacity(file_count as usize);
            for _ in 0..file_count {
                let entry_offset = reader.seek(SeekFrom::Current(0))?;

                let path_length = reader.read_u32::<LE>()?;
                let mut buf = vec![0; path_length as usize];
                reader.read_exact(&mut buf)?;
                let path = String::from_utf8(buf)?;

                let size = reader.read_u64::<LE>()?;
                let offset = reader.read_u64::<LE>()?;

                files.push(FileEntry {
                    entry_offset,
                    path,
                    size,
                    offset,
                });
            }

            // finally read directory metadata
            let dir_count = reader.read_u32::<LE>()?;
            let mut dirs = Vec::with_capacity(dir_count as usize);
            for _ in 0..dir_count {
                let entry_offset = reader.seek(SeekFrom::Current(0))?;

                let path_length = reader.read_u32::<LE>()?;
                let mut buf = vec![0; path_length as usize];
                reader.read_exact(&mut buf)?;
                let path = String::from_utf8(buf)?;

                let subfile_count = reader.read_u32::<LE>()?;
                let mut subfiles = Vec::with_capacity(subfile_count as usize);

                for _ in 0..subfile_count {
                    let entry_offset = reader.seek(SeekFrom::Current(0))?;

                    let name_length = reader.read_u32::<LE>()?;
                    let mut buf = vec![0; name_length as usize];
                    reader.read_exact(&mut buf)?;
                    let name = String::from_utf8(buf)?;
                    
                    let is_directory = reader.read_u8()? != 0;

                    subfiles.push(SubfileEntry {
                        entry_offset,
                        name,
                        is_directory,
                    });
                }

                dirs.push(DirEntry {
                    entry_offset,
                    path,
                    subfiles,
                });
            }

            // check the total header size
            let end = reader.seek(SeekFrom::Current(0))?;
            let size = end - begin;

            Ok(Header {
                version,
                files,
                dirs,
                size,
            })
        }
    }
}

pub use header::Header;

pub struct Wad {
    header: Header,
    wad_path: PathBuf,
    file: File,

    files: HashMap<String, usize>,
    dirs: HashMap<String, usize>,
}

impl Wad {
    pub fn open<P: AsRef<Path>>(wad_path: P) -> Result<Self> {
        let wad_path = wad_path.as_ref();

        let mut file = File::open(wad_path)?;
        let header = Header::read_from(&mut file)?;

        // construct the path-to-entry-index hashmaps
        let files = header.files.iter().enumerate()
            .map(|(i, entry)| (entry.path.clone(), i))
            .collect();
        let dirs = header.dirs.iter().enumerate()
            .map(|(i, entry)| (entry.path.clone(), i))
            .collect();

        Ok(Wad {
            header,
            wad_path: wad_path.to_path_buf(),
            file,

            files,
            dirs,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn files(&self) -> &HashMap<String, usize> {
        &self.files
    }

    pub fn dirs(&self) -> &HashMap<String, usize> {
        &self.dirs
    }

    /// Reads the entire file in the specified path, if any, and appends it
    /// to buf.
    pub fn read_file(&mut self, path: &str, buf: &mut Vec<u8>) -> Result<()> {
        let index = *self.files.get(path).ok_or_else(|| ErrorKind::UnknownPath(path.to_string()))?;
        let entry = &self.header.files[index];

        // allocate enough space for the data
        let begin = buf.len();
        buf.resize(begin + entry.size as usize, 0);

        // read the data
        let offset = self.header.size + entry.offset;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut buf[begin..])?;

        // we're done
        Ok(())
    }

    /// Injects a modified file into the WAD.
    pub fn inject_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        let index = *self.files.get(path).ok_or_else(|| ErrorKind::UnknownPath(path.to_string()))?;

        // reopen the file in write mode
        self.file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.wad_path)?;

        let result = self.inject_file_inner(index, data);

        // before checking for success, reopen in read mode
        self.file = File::open(&self.wad_path)?;
        
        result
    }

    fn inject_file_inner(&mut self, index: usize, data: &[u8]) -> Result<()> {
        use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian as LE};

        let header_entry = &mut self.header.files[index];
        let old_size = header_entry.size;
        let new_size = data.len() as u64;

        // seek to the offset offset
        self.file.seek(SeekFrom::Start(header_entry.entry_offset))?;
        assert_eq!(self.file.read_u32::<LE>()?, header_entry.path.len() as u32);
        self.file.seek(SeekFrom::Current(header_entry.path.len() as i64))?;

        // write new size
        self.file.write_u64::<LE>(new_size)?;

        if new_size <= old_size {
            // don't overwrite offset
            self.file.seek(SeekFrom::Start(self.header.size + header_entry.offset))?;
            self.file.write_all(data)?;
        } else {
            // overwrite offset and append data
            let total_size = self.file.metadata()?.len();
            let new_offset = total_size - self.header.size;
            self.file.write_u64::<LE>(new_offset)?;
            header_entry.offset = new_offset;

            self.file.seek(SeekFrom::End(0))?;
            self.file.set_len(total_size + new_size)?;

            self.file.write_all(data)?;
        }

        header_entry.size = new_size;

        Ok(())
    }
}

//! WAD files are the highest level container for game data.

use log::info;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use std::path::{Path, PathBuf};

use crate::errors::*;
use error_chain::bail;

pub mod header {
    use super::*;
    use byteorder::{ByteOrder, LittleEndian as LE, ReadBytesExt};
    use std::io::{BufReader, SeekFrom};

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
                let path = String::from_utf8(buf)
                    .chain_err(|| "file path is not valif UTF-8")?;

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
                let path = String::from_utf8(buf)
                    .chain_err(|| "directory path is not valid UTF-8")?;

                let subfile_count = reader.read_u32::<LE>()?;
                let mut subfiles = Vec::with_capacity(subfile_count as usize);

                for _ in 0..subfile_count {
                    let entry_offset = reader.seek(SeekFrom::Current(0))?;

                    let name_length = reader.read_u32::<LE>()?;
                    let mut buf = vec![0; name_length as usize];
                    reader.read_exact(&mut buf)?;
                    let name = String::from_utf8(buf)
                        .chain_err(|| "subfile name is not valid UTF-8")?;

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
    // interior mutability
    file: RefCell<File>,

    files: HashMap<String, usize>,
    dirs: HashMap<String, usize>,
}

impl Wad {
    pub fn open<P: AsRef<Path>>(wad_path: P) -> Result<Self> {
        let wad_path = wad_path.as_ref();

        let mut file = File::open(wad_path)?;
        let header = Header::read_from(&mut file)?;

        // construct the path-to-entry-index hashmaps
        let files = header
            .files
            .iter()
            .enumerate()
            .map(|(i, entry)| (entry.path.clone(), i))
            .collect();
        let dirs = header
            .dirs
            .iter()
            .enumerate()
            .map(|(i, entry)| (entry.path.clone(), i))
            .collect();

        Ok(Wad {
            header,
            wad_path: wad_path.to_path_buf(),
            file: RefCell::new(file),

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

    pub fn list_dir(
        &self,
        path: &'static str,
        only_files: bool,
    ) -> Result<impl Iterator<Item = String>> {
        let index = *self
            .dirs
            .get(path)
            .ok_or_else(|| ErrorKind::UnknownWadDir(path.to_string()))?;
        let entry = &self.header.dirs[index];

        let paths: Vec<_> = entry
            .subfiles
            .iter()
            .filter(|sf| !only_files || !sf.is_directory)
            .map(|sf| {
                let mut path = path.to_string();
                path.push_str("/");
                path.push_str(&sf.name);
                path
            })
            .collect();

        Ok(paths.into_iter())
    }

    /// Reads the entire file in the specified path, if any, and
    /// returns it in a buffer.
    pub fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        info!("reading {} from {}", path, self.wad_path.display());
        let index = *self
            .files
            .get(path)
            .ok_or_else(|| ErrorKind::UnknownWadFile(path.to_string()))?;
        let entry = &self.header.files[index];
        let mut file = self.file.borrow_mut();

        // read the data
        let offset = self.header.size + entry.offset;
        file.seek(SeekFrom::Start(offset))?;
        let mut buf = vec![0; entry.size as usize];
        file.read_exact(&mut buf)?;

        // we're done
        Ok(buf)
    }

    /// Injects a modified file into the WAD.
    pub fn inject_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        info!("injecting {} into {}", path, self.wad_path.display());
        let index = *self
            .files
            .get(path)
            .ok_or_else(|| ErrorKind::UnknownWadFile(path.to_string()))?;

        // reopen the file in write mode
        *self.file.borrow_mut() = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.wad_path)?;

        let result = self.inject_file_inner(index, data);

        // before checking for success, reopen in read mode
        *self.file.borrow_mut() = File::open(&self.wad_path)?;

        result
    }

    fn inject_file_inner(&mut self, index: usize, data: &[u8]) -> Result<()> {
        use byteorder::{LittleEndian as LE, WriteBytesExt};
        let mut file = self.file.borrow_mut();

        let header_entry = &mut self.header.files[index];
        let old_size = header_entry.size;
        let new_size = data.len() as u64;

        // seek to the offset offset
        file.seek(SeekFrom::Start(
            header_entry.entry_offset + 4 + header_entry.path.len() as u64,
        ))?;

        // write new size
        file.write_u64::<LE>(new_size)?;
        header_entry.size = new_size;

        if new_size <= old_size {
            // don't overwrite offset
            file.seek(SeekFrom::Start(self.header.size + header_entry.offset))?;
            file.write_all(data)?;
        } else {
            // overwrite offset and append data
            let total_size = file.metadata()?.len();
            let new_offset = total_size - self.header.size;
            file.write_u64::<LE>(new_offset)?;
            header_entry.offset = new_offset;

            file.seek(SeekFrom::End(0))?;
            file.set_len(total_size + new_size)?;

            file.write_all(data)?;
        }

        Ok(())
    }
}

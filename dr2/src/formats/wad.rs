pub mod header {
    use std::io::{prelude::*, SeekFrom, BufReader};
    use byteorder::{ByteOrder, ReadBytesExt, LittleEndian as LE};

    #[derive(Debug)]
    pub struct Header {
        pub version: [u32; 2],
        pub files: Vec<FileEntry>,
        pub dirs: Vec<DirEntry>,
        pub size: u64,
    }

    #[derive(Debug)]
    pub struct FileEntry {
        pub path: String,
        pub size: u64,
        pub offset: u64,
    }

    #[derive(Debug)]
    pub struct DirEntry {
        pub path: String,
        pub subfiles: Vec<SubfileEntry>,
    }

    #[derive(Debug)]
    pub struct SubfileEntry {
        pub name: String,
        pub is_directory: bool,
    }

    impl Header {
        pub fn read_from<R: Read + Seek>(reader: R) -> Option<Self> {
            // buffer i/o calls
            let mut reader = BufReader::new(reader);

            // store the offset at which the header begins (usually 0)
            let begin = reader.seek(SeekFrom::Current(0)).ok()?;

            // read the first 16 bytes
            let mut buf = vec![0; 16];
            reader.read(&mut buf).ok()?;

            // check if the magic bytes are present
            if &buf[0..4] != b"AGAR" {
                return None;
            }
            let version = [LE::read_u32(&buf[4..8]), LE::read_u32(&buf[8..12])];

            // now, read file metadata
            let file_count = reader.read_u32::<LE>().ok()?;
            let mut files = Vec::with_capacity(file_count as usize);
            for _ in 0..file_count {
                let path_length = reader.read_u32::<LE>().ok()?;
                let mut buf = vec![0; path_length as usize];
                reader.read(&mut buf).ok()?;
                let path = String::from_utf8(buf).ok()?;

                let size = reader.read_u64::<LE>().ok()?;
                let offset = reader.read_u64::<LE>().ok()?;

                files.push(FileEntry {
                    path,
                    size,
                    offset,
                });
            }

            // finally read directory metadata
            let dir_count = reader.read_u32::<LE>().ok()?;
            let mut dirs = Vec::with_capacity(dir_count as usize);
            for _ in 0..dir_count {
                let path_length = reader.read_u32::<LE>().ok()?;
                let mut buf = vec![0; path_length as usize];
                reader.read(&mut buf).ok()?;
                let path = String::from_utf8(buf).ok()?;

                let subfile_count = reader.read_u32::<LE>().ok()?;
                let mut subfiles = Vec::with_capacity(subfile_count as usize);

                for _ in 0..subfile_count {
                    let name_length = reader.read_u32::<LE>().ok()?;
                    let mut buf = vec![0; name_length as usize];
                    reader.read(&mut buf).ok()?;
                    let name = String::from_utf8(buf).ok()?;
                    
                    let is_directory = reader.read_u8().ok()? != 0;

                    subfiles.push(SubfileEntry {
                        name,
                        is_directory,
                    });
                }

                dirs.push(DirEntry {
                    path,
                    subfiles,
                });
            }

            // check the total header size
            let end = reader.seek(SeekFrom::Current(0)).ok()?;
            let size = end - begin;

            Some(Header {
                version,
                files,
                dirs,
                size,
            })
        }
    }
}

pub use header::Header;

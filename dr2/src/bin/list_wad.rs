use dr2::formats::wad;
use std::env;

fn main() {
    let wad_path = env::args_os().nth(1).expect("wad path not provided");
    let wad = wad::Wad::open(&wad_path).expect("unable to read wad file");
    let header = wad.header();

    for entry in &header.files {
        println!(
            "{} (offset {}, size {})",
            entry.path, entry.offset, entry.size
        );
    }
}

use std::env;
use dr2::formats::wad;

fn main() {
    let path = env::args_os().nth(1).unwrap();
    let _wad = wad::Wad::open(&path).unwrap();
}

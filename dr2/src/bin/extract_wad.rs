fn main() {
    let wad_path = std::env::args().nth(1).unwrap();
    let outdir = std::env::args().nth(2).unwrap();

    let wad = dr2::formats::wad::Wad::open(wad_path).unwrap();
    wad.extract_to(outdir).unwrap();
}

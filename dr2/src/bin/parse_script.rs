fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::read(filename).unwrap();
    let script = std::str::from_utf8(&file).unwrap();
    dr2::formats::lin::script_parser::parse(script).unwrap();
}

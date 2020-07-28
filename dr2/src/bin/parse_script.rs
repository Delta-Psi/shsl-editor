fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::read(filename).unwrap();
    let script = std::str::from_utf8(&file).unwrap();
    let script = dr2::formats::lin::script_parser::parse_script(script).unwrap();
    for instr in script.instrs {
        println!("{:?}", instr);
    }
}

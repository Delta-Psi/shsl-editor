use shsl_library::script::Script;
use std::io::prelude::*;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).unwrap();

    let script = Script::decode(&buf).unwrap();
    let decompiled = script.decompile();
    print!("{}", decompiled);
}

fn main() {
    use clap::{App, AppSettings, SubCommand, Arg};

    let app = App::new("SHSL Editor CLI")
        .version("0.1.0")
        .author("Delta-Psi")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("wad-list")
            .about("Lists all files present in a WAD file")
            .arg(Arg::with_name("WAD")
                .help("WAD file")
                .required(true)))
        .subcommand(SubCommand::with_name("wad-extract")
            .about("Extracts a single file from a WAD file")
            .arg(Arg::with_name("WAD")
                .help("WAD file")
                .required(true))
            .arg(Arg::with_name("PATH")
                .help("inner file path")
                .required(true))
            .arg(Arg::with_name("OUTDIR")
                .help("output directory")
                .default_value(".")))
        .subcommand(SubCommand::with_name("wad-inject")
            .about("Injects a modified file into a WAD file")
            .arg(Arg::with_name("WAD")
                .help("WAD file")
                .required(true))
            .arg(Arg::with_name("PATH")
                .help("inner file path")
                .required(true))
            .arg(Arg::with_name("INPUT")
                .help("modified file")
                .required(true)))
        .about("SDR2 PC modding tool");

    let matches = app.get_matches();

    if let (subcommand, Some(matches)) = matches.subcommand() {
        match subcommand {
            "wad-list" => {
                use dr2::formats::wad;

                let wad_path = matches.value_of("WAD").unwrap();
                let wad = wad::Wad::open(&wad_path).expect("could not load wad");

                for (path, _) in wad.files() {
                    println!("{}", path);
                }
            },

            "wad-extract" => {
                use std::io::prelude::*;
                use std::path::PathBuf;
                use dr2::formats::wad;

                let wad_path = matches.value_of("WAD").unwrap();
                let mut wad = wad::Wad::open(&wad_path).expect("could not load wad");

                let inner_path = matches.value_of("PATH").unwrap();
                let mut data = Vec::new();
                wad.read_file(&inner_path, &mut data).expect("could not read inner file");

                // extract filename
                let fname = inner_path.rsplitn(2, '/').nth(0).unwrap();

                let out_path = matches.value_of("OUTDIR").unwrap();
                let mut out_path = PathBuf::from(out_path);
                out_path.push(&fname);

                let mut output = std::fs::File::create(&out_path).expect("could not create output file");
                output.write_all(&data).expect("could not write to output file");
            },

            "wad-inject" => {
                use std::io::prelude::*;
                use dr2::formats::wad;

                let wad_path = matches.value_of("WAD").unwrap();
                let mut wad = wad::Wad::open(&wad_path).expect("could not load wad");

                let inner_path = matches.value_of("PATH").unwrap();

                let in_path = matches.value_of("INPUT").unwrap();
                let mut in_file = std::fs::File::open(&in_path).expect("coult not open input file");
                let mut data = Vec::new();
                in_file.read_to_end(&mut data).expect("could not read input file");

                wad.inject_file(&inner_path, &data).expect("could not inject data");
            }

            _ => unreachable!(),
        }
    }
}

fn main() {
    use clap::{App, AppSettings, SubCommand, Arg};

    let app = App::new("SHSL Editor CLI")
        .version("0.1.0")
        .author("Delta-Psi")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("extract")
            .about("Extracts the game data from dr2_data.wad and dr2_data_us.wad into a folder")
            .arg(Arg::with_name("DR2_DATA")
                .help("path to dr2_data.wad")
                .required(true))
            .arg(Arg::with_name("DR2_DATA_US")
                .help("path to dr2_data_us.wad")
                .required(true))
            .arg(Arg::with_name("OUTDIR")
                .help("output directory")
                .required(true)))
        .subcommand(SubCommand::with_name("inject")
            .arg(Arg::with_name("DR2_DATA")
                .help("path to dr2_data.wad")
                .required(true))
            .arg(Arg::with_name("DR2_DATA_US")
                .help("path to dr2_data_us.wad")
                .required(true))
            .arg(Arg::with_name("INDIR")
                .help("input directory")
                .required(true)))

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

        .subcommand(SubCommand::with_name("tga-to-png")
            .arg(Arg::with_name("INPUT")
                .required(true))
            .arg(Arg::with_name("OUTPUT")
                .required(true)))
        .subcommand(SubCommand::with_name("png-to-tga")
            .arg(Arg::with_name("INPUT")
                .required(true))
            .arg(Arg::with_name("OUTPUT")
                .required(true)))
        .about("SDR2 PC modding tool");

    let matches = app.get_matches();

    if let (subcommand, Some(matches)) = matches.subcommand() {
        match subcommand {
            "extract" => {
                let dr2_data_path = matches.value_of("DR2_DATA").unwrap();
                let dr2_data_us_path = matches.value_of("DR2_DATA_US").unwrap();
                let outdir = matches.value_of("OUTDIR").unwrap();

                extract(dr2_data_path, dr2_data_us_path, outdir);
            }

            "inject" => {
                let dr2_data_path = matches.value_of("DR2_DATA").unwrap();
                let dr2_data_us_path = matches.value_of("DR2_DATA_US").unwrap();
                let indir = matches.value_of("INDIR").unwrap();

                inject(dr2_data_path, dr2_data_us_path, indir);
            }

            "wad-list" => {
                use dr2::formats::wad;

                let wad_path = matches.value_of("WAD").unwrap();
                let wad = wad::Wad::open(&wad_path).expect("could not load wad");

                for path in wad.files().keys() {
                    println!("{}", path);
                }
            },

            "wad-extract" => {
                use std::io::prelude::*;
                use std::path::PathBuf;
                use dr2::formats::wad;

                let wad_path = matches.value_of("WAD").unwrap();
                let wad = wad::Wad::open(&wad_path).expect("could not load wad");

                let inner_path = matches.value_of("PATH").unwrap();
                let mut data = Vec::new();
                wad.read_file(&inner_path, &mut data).expect("could not read inner file");

                // extract filename
                let fname = inner_path.rsplitn(2, '/').next().unwrap();

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

            "tga-to-png" => {
                use std::io::prelude::*;
                use std::fs::File;
                use dr2::formats::tga::{self, TgaExt};

                let input_path = matches.value_of("INPUT").unwrap();
                let output_path = matches.value_of("OUTPUT").unwrap();

                let mut input = Vec::new();
                File::open(&input_path).expect("could not open input").read_to_end(&mut input).expect("count not read input");

                let image = tga::Tga::from_slice(&input).expect("could not parse tga");

                let mut output = File::create(&output_path).expect("count not create output");
                image.to_png(&mut output).expect("could not write png");
            },

            "png-to-tga" => {
                use std::fs::File;
                use dr2::formats::tga::{Tga, TgaExt};

                let input_path = matches.value_of("INPUT").unwrap();
                let output_path = matches.value_of("OUTPUT").unwrap();

                let mut input = File::open(&input_path).expect("could not open input");
                let mut data = Vec::new();
                Tga::from_png(&mut input, &mut data).expect("could not convert to png");

                Tga::from_bytes(&data).expect("invalid tga generated");

                std::fs::write(output_path, &data).expect("could not write output");
            },

            _ => unreachable!(),
        }
    }
}

pub fn extract(dr2_data_path: &str, dr2_data_us_path: &str, outdir: &str) {
    use dr2::game_data::{self, Data};

    let game_files = game_data::GameFiles::new(dr2_data_path, dr2_data_us_path).unwrap();
    game_data::GameData::extract(&game_files, outdir).unwrap();
}

pub fn inject(dr2_data_path: &str, dr2_data_us_path: &str, indir: &str) {
    use dr2::game_data::{self, Data};

    let mut game_files = game_data::GameFiles::new(dr2_data_path, dr2_data_us_path).unwrap();
    game_data::GameData::inject(&mut game_files, indir).unwrap();
}

#![windows_subsystem = "console"]

pub const VERSION: &'static str = "0.1.0";

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    use clap::{App, SubCommand, Arg};

    let app = App::new("SHSL Editor CLI")
        .version(VERSION)
        .author("Delta-Psi")
        //.setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("extract")
            .about("Extracts the game data into a folder")
            .arg(Arg::with_name("GAME_PATH")
                .help("path to the game files")
                .required(true))
            .arg(Arg::with_name("OUTDIR")
                .help("output directory")
                .required(true)))
        .subcommand(SubCommand::with_name("inject")
            .arg(Arg::with_name("GAME_PATH")
                .help("path to the game files")
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
        .subcommand(SubCommand::with_name("read-lin")
            .about("Reads a .lin file")
            .arg(Arg::with_name("LIN")
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
                let game_path = matches.value_of("GAME_PATH").unwrap();
                let outdir = matches.value_of("OUTDIR").unwrap();

                extract(game_path, outdir).unwrap();
            }

            "inject" => {
                let game_path = matches.value_of("GAME_PATH").unwrap();
                let indir = matches.value_of("INDIR").unwrap();

                inject(game_path, indir).unwrap();
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
                let data = wad.read_file(&inner_path).expect("could not read inner file");

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
                use dr2::formats::tga::{self, TgaExt};

                let input_path = matches.value_of("INPUT").unwrap();
                let output_path = matches.value_of("OUTPUT").unwrap();

                let input = std::fs::read(&input_path).expect("could not read input");
                let image = tga::Tga::from_slice(&input).expect("could not parse tga");

                let output = image.to_png().expect("could not write png");
                std::fs::write(&output_path, &output).expect("could not write to output");
            },

            "png-to-tga" => {
                use dr2::formats::tga::{Tga, TgaExt};

                let input_path = matches.value_of("INPUT").unwrap();
                let output_path = matches.value_of("OUTPUT").unwrap();

                let input = std::fs::read(&input_path).expect("could not read input");
                let data = Tga::from_png(&input).expect("could not convert to tga");

                Tga::from_bytes(&data).expect("invalid tga generated");

                std::fs::write(output_path, &data).expect("could not write output");
            },

            "read-lin" => {
                use dr2::formats::lin::Lin;

                let input_path = matches.value_of("LIN").unwrap();
                let input = std::fs::read(&input_path).expect("could not read input");

                let lin = Lin::from_bytes(&input).expect("could not read lin");
                println!("{:#?}", lin);
            }

            _ => unreachable!(),
        }
    } else {
        // interactive mod
        use std::io::prelude::*;

        println!("SHSL Editor CLI {} by Delta-Psi", VERSION);
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();

        println!("Type in one of the commands below.\nextract\ninject");
        let mut choice = String::new();
        stdin.read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice != "extract" && choice != "inject" {
            println!("invalid mode");
        } else {
            let result: dr2::errors::Result<()> = (|| {
                print!("Path of game data: ");
                stdout.flush().unwrap();
                let mut game_path = String::new();
                stdin.read_line(&mut game_path).unwrap();

                print!("Project directory: ");
                stdout.flush().unwrap();
                let mut projdir = String::new();
                stdin.read_line(&mut projdir).unwrap();

                if choice == "extract" {
                    extract(game_path.trim(),
                    projdir.trim())?;
                } else {
                    inject(game_path.trim(),
                    projdir.trim())?;
                }

                println!("Finished!");
                Ok(())
            })();
            match result {
                Ok(()) => (),
                Err(err) => println!("ERROR: {}", err),
            }
        }
        stdin.bytes().next().unwrap().unwrap();
    }
}

pub fn extract(game_path: &str, outdir: &str) -> dr2::errors::Result<()> {
    use dr2::game_data;

    let mut project = game_data::Project::create(outdir, Default::default())?;
    let game_files = game_data::GameFiles::load(game_path)?;
    game_data::extract(&mut project, &game_files)?;

    Ok(())
}

pub fn inject(game_path: &str, indir: &str) -> dr2::errors::Result<()> {
    use dr2::game_data;

    let mut project = game_data::Project::open(indir)?;
    let mut game_files = game_data::GameFiles::load(game_path)?;
    game_data::inject(&mut project, &mut game_files)?;

    Ok(())
}

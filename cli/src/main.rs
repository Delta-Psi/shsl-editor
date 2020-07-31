#![windows_subsystem = "console"]

pub const VERSION: &str = "0.1.0";

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    use clap::{App, AppSettings, Arg, SubCommand};

    let app = App::new("SHSL Editor CLI")
        .version(VERSION)
        .author("Delta-Psi")
        .about("SDR2 PC modding tool")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a blank project")
                .arg(
                    Arg::with_name("PROJECT")
                        .help("path to the new project (must not exist)")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extracts game data according to the project configuration")
                .arg(
                    Arg::with_name("PROJECT")
                        .help("path to the project")
                        .required(true),
                )
                .arg(
                    Arg::with_name("GAME_PATH")
                        .help("path to the game files")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("inject")
                .arg(
                    Arg::with_name("PROJECT")
                        .help("path to the project")
                        .required(true),
                )
                .arg(
                    Arg::with_name("GAME_PATH")
                        .help("path to the game files")
                        .required(true),
                ),
        );

    let matches = app.get_matches();

    if let (subcommand, Some(matches)) = matches.subcommand() {
        match subcommand {
            "new" => {
                let project_path = matches.value_of("PROJECT").unwrap();

                create(project_path).unwrap();
            }

            "extract" => {
                let project_path = matches.value_of("PROJECT").unwrap();
                let game_path = matches.value_of("GAME_PATH").unwrap();

                extract(game_path, project_path).unwrap();
            }

            "inject" => {
                let project_path = matches.value_of("PROJECT").unwrap();
                let game_path = matches.value_of("GAME_PATH").unwrap();

                inject(game_path, project_path).unwrap();
            }

            _ => unreachable!(),
        }
    }
}

pub fn create(project_path: &str) -> dr2::errors::Result<()> {
    use dr2::project::Project;
    
    Project::create(project_path, Default::default())?;

    Ok(())
}

pub fn extract(game_path: &str, project_path: &str) -> dr2::errors::Result<()> {
    use dr2::project::Project;
    use dr2::game_data;

    let mut project = Project::open(project_path)?;
    let game_files = game_data::GameFiles::load(game_path)?;
    game_data::extract(&mut project, &game_files)?;

    Ok(())
}

pub fn inject(game_path: &str, project_path: &str) -> dr2::errors::Result<()> {
    use dr2::project::Project;
    use dr2::game_data;

    let mut project = Project::open(project_path)?;
    let mut game_files = game_data::GameFiles::load(game_path)?;
    game_data::inject(&mut project, &mut game_files)?;

    Ok(())
}

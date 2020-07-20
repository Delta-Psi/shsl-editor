fn main() {
    let app = clap::App::new("SHSL Editor CLI")
        .version("0.1.0")
        .author("Delta-Psi")
        .about("SDR2 PC modding tool");

    app.get_matches();
}

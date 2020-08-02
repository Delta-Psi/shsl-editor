use dr2::formats::gmo::Gmo;

fn main() {
    let gmo_path = std::env::args().nth(1).unwrap();
    let gmo_data = std::fs::read(gmo_path).unwrap();
    let gmo = Gmo::from_bytes(&gmo_data).unwrap();

    println!("{:#x?}", gmo);
}

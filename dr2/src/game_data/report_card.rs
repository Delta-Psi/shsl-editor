use super::*;
use std::collections::BTreeMap;
use crate::formats::pak::Pak;
use crate::formats::tga::{Tga, TgaExt};
use serde::Serialize;
use log::info;

pub const STUDENT_COUNT: usize = 16;

#[derive(Serialize)]
pub struct ReportCard {
    pub name: String,
    pub height: String,
    pub weight: String,
    pub chest: String,
    pub blood_type: String,
    pub birthday: String,
    pub likes: String,
    pub dislikes: String,

    pub ultimate: [String; 3],

    pub fte_summaries: Option<[String; 5]>,
}

pub fn extract(files: &GameFiles, path: &Path) -> Result<()> {
    {
        let path = path.join("report_card.toml");

        let mut buf = Vec::new();
        files.dr2_data_us.wad.read_file("Dr2/data/us/bin/bin_progress_font_l.pak", &mut buf)?;
        let pak = Pak::from_bytes(&buf)?;

        let e8 = Pak::from_bytes(&pak.entries[8])?;
        let e9 = Pak::from_bytes(&pak.entries[9])?;
        let e16 = Pak::from_bytes(&pak.entries[16])?;

        let mut report_cards = BTreeMap::new();

        for i in 0..STUDENT_COUNT {
            use crate::decode_utf16;

            report_cards.insert(format!("{:02}", i), ReportCard {
                name: decode_utf16(&e16.entries[i]),
                height: decode_utf16(&e16.entries[STUDENT_COUNT + i]),
                weight: decode_utf16(&e16.entries[2*STUDENT_COUNT + i]),
                chest: decode_utf16(&e16.entries[3*STUDENT_COUNT + i]),
                blood_type: decode_utf16(&e16.entries[4*STUDENT_COUNT + i]),
                birthday: decode_utf16(&e16.entries[5*STUDENT_COUNT + i]),
                likes: decode_utf16(&e16.entries[6*STUDENT_COUNT + i]),
                dislikes: decode_utf16(&e16.entries[7*STUDENT_COUNT + i]),

                ultimate: [
                    decode_utf16(&e8.entries[i]),
                    decode_utf16(&e8.entries[STUDENT_COUNT + i]),
                    decode_utf16(&e8.entries[2*STUDENT_COUNT + i]),
                ],

                fte_summaries: match i {
                    0 => None, // gets nothing :'(
                    _ => Some([
                        decode_utf16(&e9.entries[(i-1)*5]),
                        decode_utf16(&e9.entries[(i-1)*5 + 1]),
                        decode_utf16(&e9.entries[(i-1)*5 + 2]),
                        decode_utf16(&e9.entries[(i-1)*5 + 3]),
                        decode_utf16(&e9.entries[(i-1)*5 + 4]),
                    ]),
                },
            });
        }

        info!("writing {}", path.display());
        std::fs::write(path, toml::to_string_pretty(&report_cards)?.as_bytes())?;
    }

    let path = path.join("report_card");
    {
        let path = path.join("pictures");
        std::fs::create_dir_all(&path)?;
        for i in 0..STUDENT_COUNT {
            let mut buf = Vec::new();
            files.dr2_data.wad.read_file(&format!("Dr2/data/all/cg/report/tsushimbo_chara_{:03}.tga", i), &mut buf)?;

            let image = Tga::from_bytes(&buf)?;

            let path = path.join(format!("{:02}.png", i));
            info!("writing {}", path.display());
            let mut file = std::fs::File::create(path)?;
            image.to_png(&mut file)?;
        }
    }

    Ok(())
}

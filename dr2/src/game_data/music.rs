//! TODO

pub const TRACK_COUNT: usize = 102;

pub struct Track {
    /// In seconds.
    pub loop_points: Option<(f32, f32)>,
}

impl Track {
    pub fn ogg_path(idx: usize) -> String {
        format!("Dr2/data/all/bgm/dr2_bgm_hca.awb.{:05}.ogg", idx)
    }
}

pub struct Music {
    pub tracks: Box<[Track; TRACK_COUNT]>,
}

/*
impl super::GameData for Music {
    fn extract(file: &GameFiles) -> Result<Self> {
    }

    fn inject(&self, files: &mut GameFiles) -> Result<()> {
    }
}
*/

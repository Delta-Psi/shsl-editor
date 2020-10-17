use byteorder::{ByteOrder, LE};

#[derive(Debug)]
pub struct Pak<'a> {
    pub entries: Vec<&'a [u8]>,
}

impl<'a> Pak<'a> {
    pub fn decode(data: &'a [u8]) -> Option<Self> {
        // read entry count
        let count = LE::read_u32(data.get(0..4)?) as usize;

        // read entry offset
        let mut offset_data = data.get(4..)?;
        let mut offsets = Vec::with_capacity(count);
        for _ in 0..count {
            let offset = LE::read_u32(offset_data);
            offsets.push(offset as usize);
            offset_data = offset_data.get(4..)?;
        }

        // ensure entry offsets are ascending and within bounds
        offsets.push(data.len());
        if offsets.windows(2).any(|w| w[1] <= w[0]) {
            return None;
        }

        let entries = (0..count)
            .map(|i| data.get(offsets[i] .. offsets[i+1]))
            .collect::<Option<_>>()?;

        Some(Self {
            entries,
        })
    }
}

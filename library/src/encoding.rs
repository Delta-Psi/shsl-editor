use byteorder::{ByteOrder, LE};

pub fn decode_utf16_le(data: &[u8]) -> Option<String> {
    // find bom
    if data.get(0..2)? != &[0xff, 0xfe] {
        return None;
    }
    let mut data = data.get(2..)?;

    let mut code_units = Vec::with_capacity(data.len() / 2);
    while !data.is_empty() {
        let code_unit = LE::read_u16(data);
        if code_unit == 0 {
            break;
        }
        code_units.push(code_unit);
        data = data.get(2..)?;
    }

    String::from_utf16(&code_units).ok()
}

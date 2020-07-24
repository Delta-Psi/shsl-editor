//! Adds some additional functionality to `tinytga`.
pub use tinytga::*;

use std::io::prelude::*;
use byteorder::{WriteBytesExt, LittleEndian as LE};

use crate::errors::*;

pub trait TgaExt<'a>: Sized {
    /// This method is preferred, as it returns an actually usable error type.
    fn from_bytes(data: &'a [u8]) -> Result<Self>;
    fn to_png(&self) -> Result<Vec<u8>>;
    /// Directly encodes and returns TGA data.
    fn from_png(data: &[u8]) -> Result<Vec<u8>>;
}

impl<'a> TgaExt<'a> for Tga<'a> {
    fn from_bytes(data: &'a [u8]) -> Result<Self> {
        Self::from_slice(data).map_err(|_| ErrorKind::TgaDecoding.into())
    }

    fn to_png(&self) -> Result<Vec<u8>> {
        let mut buf = std::io::Cursor::new(Vec::new());
        let mut encoder = png::Encoder::new(&mut buf, self.width() as u32, self.height() as u32);
        let mut pixel_data = self.pixel_data.to_vec();

        match self.header.image_type {
            ImageType::ColorMapped => {
                encoder.set_color(png::ColorType::Indexed);
                encoder.set_depth(png::BitDepth::Eight);

                let color_map = self.color_map.unwrap();

                match self.header.color_map_depth {
                    24 => {
                        let mut plte = color_map.to_vec();

                        // fix ordering (BGR to RGB)
                        for i in 0..self.header.color_map_len as usize {
                            plte.swap(3*i, 3*i+2);
                        }

                        encoder.set_palette(plte);
                    },
                    32 => {
                        // split the color map into PLTE (color) and tRNS (alpha) chunks
                        let mut plte = Vec::with_capacity(self.header.color_map_len as usize*3);
                        let mut trns = Vec::with_capacity(self.header.color_map_len as usize);

                        for i in 0..self.header.color_map_len as usize {
                            plte.push(color_map[4*i+2]);
                            plte.push(color_map[4*i+1]);
                            plte.push(color_map[4*i  ]);
                            trns.push(color_map[4*i+3]);
                        }

                        encoder.set_palette(plte);
                        encoder.set_trns(trns);
                    },

                    _ => unimplemented!(),
                }
            }

            _ => unimplemented!(),
        };

        let w = self.width() as usize;
        let h = self.height() as usize;
        match (self.header.image_descriptor & 0xf0) >> 4 {
            0b10 => (),
            0b01 => pixel_data.reverse(),
            0b00 => {
                // flip rows
                for row in 0..h/2 {
                    let opposite = h-1-row;
                    for col in 0..w {
                        pixel_data.swap(row*w+col, opposite*w+col);
                    }
                }
            },
            0b11 => {
                // flip columns
                for row in 0..h {
                    for col in 0..w/2 {
                        let opposite = w-1-col;
                        pixel_data.swap(row*w+col, row*w+opposite);
                    }
                }
            }

            _ => unimplemented!(),
        }

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&pixel_data[0..w*h*(self.header.pixel_depth/8) as usize])?;
        drop(writer);

        Ok(buf.into_inner())
    }

    fn from_png(data: &[u8]) -> Result<Vec<u8>> {
        let reader = std::io::Cursor::new(data);
        let mut decoder = png::Decoder::new(reader);
        decoder.set_transformations(png::Transformations::IDENTITY);
        let (info, mut reader) = decoder.read_info()?;

        let mut pixel_data = vec![0; info.buffer_size()];
        reader.next_frame(&mut pixel_data)?;

        let info = reader.info();

        let mut writer = std::io::Cursor::new(Vec::new());

        if info.interlaced {
            unimplemented!("interlaced png");
        }

        // write header
        writer.write_u8(0)?; // image ID field: blank
        
        let color_map;
        if info.color_type == png::ColorType::Indexed {
            writer.write_u8(1)?; // color map type: present
            writer.write_u8(1)?; // image type: colormapped
            writer.write_u16::<LE>(0)?; // color map offset

            let palette = info.palette.as_ref().unwrap();
            let color_map_len = palette.len()/3;
            if color_map_len > 256 {
                unimplemented!("multibyte palette indices");
            }
            writer.write_u16::<LE>(color_map_len as u16)?; // color map length

            if let Some(trns) = &info.trns {
                writer.write_u8(32)?; // color map depth

                // with transparency; encode as BGRA
                let mut buf = Vec::with_capacity(color_map_len*4);

                for i in 0..color_map_len {
                    buf.push(palette[3*i+2]);
                    buf.push(palette[3*i+1]);
                    buf.push(palette[3*i  ]);
                    buf.push(trns[i]);
                }

                color_map = Some(buf);
            } else {
                writer.write_u8(24)?; // color map depth

                // encode as BGR
                let mut buf = Vec::with_capacity(palette.len()*3);

                for i in 0..color_map_len {
                    buf.push(palette[3*i+2]);
                    buf.push(palette[3*i+1]);
                    buf.push(palette[3*i  ]);
                }

                color_map = Some(buf);
            }
        } else {
            unimplemented!();
        };

        writer.write_u16::<LE>(0)?; // x origin
        writer.write_u16::<LE>(0)?; // y origin
        writer.write_u16::<LE>(info.width as u16)?; // width
        writer.write_u16::<LE>(info.height as u16)?; // height

        // pixel depth
        writer.write_u8(if info.color_type == png::ColorType::Indexed {
            8
        } else {
            unimplemented!()
        })?;

        writer.write_u8(0b0010_0000)?; // image descriptor

        if let Some(color_map) = color_map {
            // write color map
            writer.write_all(&color_map)?;
        }

        // write pixel data
        writer.write_all(&pixel_data)?;

        Ok(writer.into_inner())
    } 
}

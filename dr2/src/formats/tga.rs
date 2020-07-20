//! Adds some additional functionality to `tinytga`.
pub use tinytga::*;

use std::io::prelude::*;

use error_chain::error_chain;
error_chain! {
    foreign_links {
        PngEncoding(png::EncodingError);
    }
}

pub trait TgaExt {
    fn to_png<W: Write>(&self, writer: W) -> Result<()>;
}

impl<'a> TgaExt for Tga<'a> {
    fn to_png<W: Write>(&self, writer: W) -> Result<()> {
        let mut encoder = png::Encoder::new(writer,
            self.width() as u32,
            self.height() as u32);

        match self.header.image_type {
            ImageType::ColorMapped => {
                encoder.set_color(png::ColorType::Indexed);
                let color_map = self.color_map.unwrap();

                // split the color map into PLTE (color) and tRNS (alpha) chunks
                match self.header.color_map_depth {
                    24 => {
                        encoder.set_palette(color_map.to_vec());
                    },
                    32 => {
                        let mut plte = Vec::with_capacity(self.header.color_map_len as usize*3);
                        let mut trns = Vec::with_capacity(self.header.color_map_len as usize);

                        for i in 0..self.header.color_map_len as usize {
                            plte.push(color_map[4*i+2]);
                            plte.push(color_map[4*i+1]);
                            plte.push(color_map[4*i+0]);
                            trns.push(color_map[4*i+3]);
                        }

                        encoder.set_palette(plte);
                        encoder.set_trns(trns);
                    },

                    _ => unimplemented!(),
                }

                if self.header.pixel_depth != 8 {
                    unimplemented!();
                }

                encoder.set_depth(png::BitDepth::Eight);
            }

            _ => unimplemented!(),
        };

        let w = self.width() as usize;
        let h = self.height() as usize;
        let mut pixel_data = self.pixel_data.to_vec();

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

        encoder.write_header()?.write_image_data(&pixel_data)?;

        Ok(())
    }
}

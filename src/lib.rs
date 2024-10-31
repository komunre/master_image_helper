pub mod image {
    use std::{error::Error, fs::File, usize};

    use png::{BitDepth, ColorType, DecodingError};

    pub struct ImageData {
        width: usize,
        height: usize,
        color: png::ColorType,
        bit_depth: png::BitDepth,

        pixels: Vec<u8>
    }

    #[derive(Debug, Copy, Clone)]
    pub struct PixelData {
        bit_depth: png::BitDepth,

        r: usize,
        g: usize,
        b: usize,
        a: usize,
    }

    impl PixelData {
        pub fn new(bit_depth: png::BitDepth, r: usize, g: usize, b: usize, a: usize) -> Self {
            PixelData {
                bit_depth,
                r,
                g,
                b,
                a
            }
        }

        pub fn r(&self) -> usize {
            self.r
        }

        pub fn g(&self) -> usize {
            self.g
        }

        pub fn b(&self) -> usize {
            self.b
        }

        pub fn a(&self) -> usize {
            self.a
        }

        pub fn bit_depth(&self) -> &png::BitDepth {
            &self.bit_depth
        }

        pub fn mut_r(&mut self) -> &mut usize {
            &mut self.r
        }

        pub fn mut_g(&mut self) -> &mut usize {
            &mut self.g
        }

        pub fn mut_b(&mut self) -> &mut usize {
            &mut self.b
        }

        pub fn mut_a(&mut self) -> &mut usize {
            &mut self.a
        }
    }

    impl ImageData {
        pub fn new(width: usize, height: usize, color: png::ColorType, bit_depth: png::BitDepth, pixels: Vec<u8>) -> Self {
            ImageData {
                width,
                height,
                color,
                bit_depth,
                pixels
            }
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.height
        }

        pub fn get_pixel_at(&self, x: usize, y: usize) -> PixelData {
            let bytes = f64::from(self.bit_depth as u32) / 8.0;
            let elements = match self.color {
                ColorType::Rgba => 4,
                ColorType::Rgb => 3,
                ColorType::GrayscaleAlpha => 2,
                ColorType::Grayscale => 1,
                ColorType::Indexed => 1
            } as u32;

            let index: usize = (f64::from((y * self.width + x) as u32 * elements) * bytes) as usize;

            if index > self.pixels.len() {
                return PixelData::new(BitDepth::Eight, 0, 0, 0, 0);
            }

            let pixel: PixelData;
            
            match self.bit_depth {
                BitDepth::Eight => {
                    match self.color {
                        ColorType::Rgba => {
                            pixel = PixelData::new(self.bit_depth, self.pixels[index].into(), self.pixels[index+1].into(), self.pixels[index+2].into(), self.pixels[index+3].into())
                        }
                        ColorType::Rgb => {
                            pixel = PixelData::new(self.bit_depth, self.pixels[index].into(), self.pixels[index + 1].into(), self.pixels[index + 2].into(), 1)
                        }
                        ColorType::Grayscale => {
                            pixel = PixelData::new(self.bit_depth, self.pixels[index].into(), 0, 0, 0)
                        }
                        ColorType::GrayscaleAlpha => {
                            pixel = PixelData::new(self.bit_depth, self.pixels[index].into(), 0, 0, self.pixels[index + 1].into())
                        }
                        ColorType::Indexed => {
                            pixel = PixelData::new(self.bit_depth, self.pixels[index].into(), 0, 0, 0)
                        }
                    }
                }
                BitDepth::Four => {
                    match self.color {
                        ColorType::Rgba => {
                            pixel = PixelData::new(self.bit_depth, (self.pixels[index] >> 4).into(), (self.pixels[index] << 4 >> 4).into(), (self.pixels[index + 1] >> 4).into(), (self.pixels[index + 1] >> 4 << 4).into())
                        }
                        ColorType::Rgb => {
                            pixel = PixelData::new(self.bit_depth, (self.pixels[index] >> 4).into(), (self.pixels[index] << 4 >> 4).into(), (self.pixels[index + 1] >> 4).into(), 0)
                        }
                        ColorType::Grayscale => {
                            pixel = PixelData::new(self.bit_depth, (self.pixels[index] >> 4).into(), 0, 0, 0)
                        }
                        ColorType::GrayscaleAlpha => {
                            pixel = PixelData::new(self.bit_depth, (self.pixels[index] >> 4).into(), 0, 0, (self.pixels[index] << 4 >> 4).into())
                        }
                        ColorType::Indexed => {
                            pixel = PixelData::new(self.bit_depth, (self.pixels[index] >> 4).into(), 0, 0, 0)
                        }
                    }
                }
                // TODO: Implement support for all bit depths
                _ => {
                    pixel = PixelData::new(BitDepth::Eight, 0, 0, 0, 0);
                }
            };

            pixel
        }
    }

    fn get_decoder(path: &str) -> Result<png::Decoder<File>, std::io::Error> {
        Ok(png::Decoder::new(File::open(path)?))
    }

    fn get_reader(decoder: png::Decoder<File>) -> Result<png::Reader<File>, DecodingError> {
        decoder.read_info()
    }

    fn get_image(mut reader: png::Reader<File>) -> Result<ImageData, DecodingError> {
        let mut buf = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut buf)?;

        let bytes = &buf[..info.buffer_size()]; // Get a splice of correct size. Returned frame might be smaller than output buffer.
        let (color_type, bit_depth) = reader.output_color_type();

        Ok(ImageData::new(info.width as usize, info.height as usize, color_type, bit_depth, Vec::from(bytes)))
    }

    pub fn read_image_from_file(path: &str) -> Result<ImageData, Box<dyn Error>> {
        let decoder = get_decoder(path)?;
        let reader = get_reader(decoder)?;
        Ok(get_image(reader)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

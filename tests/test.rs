#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    // benchmarking
    use std::time::Instant;
    // path resolving
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    // decoder function import
    use texture2ddecoder::*;
    // image saving
    extern crate image;
    // texture file decoder
    extern crate ddsfile;
    extern crate ktx2;

    // test functions
    #[test]
    fn test_ATC_RGB() {
        test_format("ATC_RGB", "dds", decode_atc_rgb4)
    }

    #[test]
    fn test_ATC_RGBA_Explicit() {
        test_format("ATC_RGBA_Explicit", "dds", decode_atc_rgba8)
    }

    #[test]
    fn test_ATC_RGBA_Interpolated() {
        test_format("ATC_RGBA_Interpolated", "dds", decode_atc_rgba8)
    }

    #[test]
    fn test_BC1() {
        test_format("BC1", "ktx2", decode_bc1)
    }

    #[test]
    fn test_BC3() {
        test_format("BC3", "ktx2", decode_bc3)
    }

    #[test]
    fn test_BC4() {
        test_format("BC4", "ktx2", decode_bc4)
    }

    #[test]
    fn test_BC5() {
        test_format("BC5", "ktx2", decode_bc5)
    }

    #[test]
    fn test_BC6H() {
        test_format("BC6H", "ktx2", decode_bc6)
    }

    #[test]
    fn test_BC7() {
        test_format("BC7", "ktx2", decode_bc7)
    }

    #[test]
    fn test_ETC1_RGB() {
        test_format("ETC1_RGB", "ktx2", decode_etc1)
    }

    #[test]
    fn test_ETC2_RGB() {
        test_format("ETC2_RGB", "ktx2", decode_etc2)
    }

    #[test]
    fn test_ETC2_RGBA() {
        test_format("ETC2_RGBA", "ktx2", decode_etc2a8)
    }

    #[test]
    fn test_ETC2_RGB_A1() {
        test_format("ETC2_RGB_A1", "ktx2", decode_etc2a1)
    }

    #[test]
    fn test_PVRTCI_2bpp_RGB() {
        test_format("PVRTCI_2bpp_RGB", "ktx2", decode_pvrtc_2bpp)
    }

    #[test]
    fn test_PVRTCI_2bpp_RGBA() {
        test_format("PVRTCI_2bpp_RGBA", "ktx2", decode_pvrtc_2bpp)
    }

    #[test]
    fn test_PVRTCI_4bpp_RGB() {
        test_format("PVRTCI_4bpp_RGB", "ktx2", decode_pvrtc_4bpp)
    }

    #[test]
    fn test_PVRTCI_4bpp_RGBA() {
        test_format("PVRTCI_4bpp_RGBA", "ktx2", decode_pvrtc_4bpp)
    }

    #[test]
    fn test_EAC_R11() {
        test_format("EAC_R11", "ktx2", decode_eacr)
    }

    #[test]
    fn test_EAC_RG11() {
        test_format("EAC_RG11", "ktx2", decode_eacrg)
    }

    // helper structs and functions
    struct Texture {
        width: u32,
        height: u32,
        data: Vec<u8>,
    }

    impl Texture {
        fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
            Self {
                width,
                height,
                data,
            }
        }

        fn from_file(fp: &str) -> Texture {
            let extension = Path::new(fp).extension().unwrap().to_str().unwrap();
            match extension {
                "ktx2" => Texture::from_ktx2_file(fp),
                "dds" => Texture::from_dds_file(fp),
                _ => panic!("Unsupported file format"),
            }
        }

        fn from_ktx2_file(fp: &str) -> Texture {
            let ktx2_data = fs::read(fp).unwrap();
            let reader = ktx2::Reader::new(ktx2_data).expect("Can't create reader"); // Crate instance of reader.

            // Get general texture information.
            let header = reader.header();

            // Read iterator over slices of each mipmap level.
            let levels = reader.levels().collect::<Vec<_>>();

            Texture::new(header.pixel_width, header.pixel_height, levels[0].to_vec())
        }

        fn from_dds_file(fp: &str) -> Texture {
            let dds_data = fs::read(fp).unwrap();
            let dds = ddsfile::Dds::read(&dds_data[..]).unwrap();

            Texture::new(dds.header.width, dds.header.height, dds.data)
        }

        fn _decode(&self, decode_func: fn(&[u8], usize, usize, &mut [u32])) -> Vec<u32> {
            let mut image: Vec<u32> = vec![0; (self.width * self.height) as usize];
            let start = Instant::now();
            decode_func(
                &self.data,
                self.width as usize,
                self.height as usize,
                &mut image,
            );
            let duration = start.elapsed();
            println!("Time elapsed in decoding is: {:?}", duration);
            image
        }
        fn save_as_image(&self, path: &str, decode_func: fn(&[u8], usize, usize, &mut [u32])) {
            let image = self._decode(decode_func);
            let image_buf = image
                .iter()
                .flat_map(|x| {
                    let v = x.to_le_bytes();
                    [v[2], v[1], v[0], v[3]]
                })
                .collect::<Vec<u8>>();

            image::save_buffer(
                Path::new(path),
                &image_buf,
                self.width,
                self.height,
                image::ColorType::Rgba8,
            )
            .unwrap();
        }
    }

    fn get_texture_fp(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/tests/textures");
        d.push(name);
        d.to_str().unwrap().to_string()
    }

    fn get_image_fp(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/tests/decompressed");
        fs::create_dir_all(&d).unwrap();
        d.push(name);
        d.to_str().unwrap().to_string()
    }

    fn test_format(
        name: &str,
        sample_extension: &str,
        decode_func: fn(&[u8], usize, usize, &mut [u32]),
    ) {
        println!("Testing {}", name);
        let src_fp = get_texture_fp(&format!("{}.{}", name, sample_extension));
        let dst_fp = get_image_fp(&format!("{}.png", name));

        let texture = Texture::from_file(&src_fp);
        texture.save_as_image(&dst_fp, decode_func);
    }
}

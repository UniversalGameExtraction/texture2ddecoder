#![allow(non_snake_case)]

type DecodeFunction = fn(&[u8], usize, usize, &mut [u32]) -> Result<(), &'static str>;

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

    use crate::DecodeFunction;
    // image saving
    extern crate image;
    // texture file decoder
    extern crate ddsfile;
    extern crate ktx2;

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
    fn test_ASTC_4x4() {
        test_format("ASTC_4x4", "ktx2", decode_astc_4_4)
    }

    #[test]
    fn test_ASTC_5x4() {
        test_format("ASTC_5x4", "ktx2", decode_astc_5_4)
    }

    #[test]
    fn test_ASTC_5x5() {
        test_format("ASTC_5x5", "ktx2", decode_astc_5_5)
    }

    #[test]
    fn test_ASTC_6x5() {
        test_format("ASTC_6x5", "ktx2", decode_astc_6_5)
    }

    #[test]
    fn test_ASTC_6x6() {
        test_format("ASTC_6x6", "ktx2", decode_astc_6_6)
    }

    #[test]
    fn test_ASTC_8x5() {
        test_format("ASTC_8x5", "ktx2", decode_astc_8_5)
    }

    #[test]
    fn test_ASTC_8x6() {
        test_format("ASTC_8x6", "ktx2", decode_astc_8_6)
    }

    #[test]
    fn test_ASTC_8x8() {
        test_format("ASTC_8x8", "ktx2", decode_astc_8_8)
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
        test_format("BC6H", "ktx2", decode_bc6_unsigned)
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
        test_format("ETC2_RGB", "ktx2", decode_etc2_rgb)
    }

    #[test]
    fn test_ETC2_RGBA() {
        test_format("ETC2_RGBA", "ktx2", decode_etc2_rgba8)
    }

    #[test]
    fn test_ETC2_RGB_A1() {
        test_format("ETC2_RGB_A1", "ktx2", decode_etc2_rgba1)
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

    #[test]
    fn test_CRUNCH_DXT1() {
        test_format("CRUNCH_DXT1", "crn", decode_crunch)
    }

    #[test]
    fn test_CRUNCH_DXT5() {
        test_format("CRUNCH_DXT5", "crn", decode_crunch)
    }

    #[test]
    fn test_CRUNCH_DXT5A() {
        test_format("CRUNCH_DXT5A", "crn", decode_crunch)
    }

    #[test]
    fn test_CRUNCH_DXN() {
        test_format("CRUNCH_DXN", "crn", decode_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_DXT1() {
        test_format("UNITYCRUNCH_DXT1", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_DXT5() {
        test_format("UNITYCRUNCH_DXT5", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_DXT5A() {
        test_format("UNITYCRUNCH_DXT5A", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_DXN() {
        test_format("UNITYCRUNCH_DXN", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_ETC1() {
        test_format("UNITYCRUNCH_ETC1", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_ETC1S() {
        test_format("UNITYCRUNCH_ETC1S", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_ETC2() {
        test_format("UNITYCRUNCH_ETC2", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_ETC2A() {
        test_format("UNITYCRUNCH_ETC2A", "crn", decode_unity_crunch)
    }

    #[test]
    fn test_UNITYCRUNCH_ETC2AS() {
        test_format("UNITYCRUNCH_ETC2AS", "crn", decode_unity_crunch)
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
                "ktx2" | "KTX2" => Texture::from_ktx2_file(fp),
                "dds" | "DDS" => Texture::from_dds_file(fp),
                "crn" | "CRN" => Texture::from_crn_file(fp),
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

        fn from_crn_file(fp: &str) -> Texture {
            let crn_data = fs::read(fp).unwrap();
            let mut tex_info = CrnTextureInfo::default();
            tex_info.crnd_get_texture_info(&crn_data, crn_data.len() as u32);
            Texture::new(
                core::cmp::max(1, tex_info.width),
                core::cmp::max(1, tex_info.height),
                crn_data,
            )
        }

        fn _decode(&self, decode_func: DecodeFunction) -> Vec<u32> {
            let mut image: Vec<u32> = vec![0; (self.width * self.height) as usize];
            let start = Instant::now();
            decode_func(
                &self.data,
                self.width as usize,
                self.height as usize,
                &mut image,
            )
            .unwrap();
            let duration = start.elapsed();
            println!("Time elapsed in decoding is: {:?}", duration);
            image
        }
        fn save_as_image(&self, path: &str, decode_func: DecodeFunction) {
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

    fn test_format(name: &str, sample_extension: &str, decode_func: DecodeFunction) {
        println!("Testing {}", name);
        let src_fp = get_texture_fp(&format!("{}.{}", name, sample_extension));
        let dst_fp = get_image_fp(&format!("{}.png", name));

        let texture = Texture::from_file(&src_fp);
        texture.save_as_image(&dst_fp, decode_func);
    }
}

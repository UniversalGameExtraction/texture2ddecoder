#[cfg(test)]
mod tests {
    use std::fs::DirEntry;

    // benchmarking
    use std::time::Instant;
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    // decoder function import
    use texture2ddecoder::*;
    // image saving
    extern crate image;
    // ktx decoder
    extern crate ktx2;
    // dds decoder
    extern crate ddsfile;

    fn decode_pvrtc_2bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
        decode_pvrtc(data, m_width, m_height, image, true);
    }
    fn decode_pvrtc_4bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
        decode_pvrtc(data, m_width, m_height, image, false);
    }

    struct Texture {
        width: u32,
        height: u32,
        data: Vec<u8>,
    }

    #[test]
    fn main() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/tests/textures");

        for file in fs::read_dir(d).unwrap() {
            let entry = file.unwrap();
            test_file(entry);
        }
    }

    fn test_file(entry: DirEntry) {
        // Crate instance of reader. This validates the header
        let binding = entry.path();
        let filename = Path::new(binding.to_str().unwrap()).file_name().unwrap();

        println!("Testing {}...", filename.to_str().unwrap());

        let binding = entry.path();
        let (texture_format, file_format) = filename.to_str().unwrap().split_once('.').unwrap();

        let file_parse_func = get_parse_func(file_format);
        let texture_decode_func = get_decode_func(texture_format);

        if file_parse_func.is_none() || texture_decode_func.is_none() {
            println!("... is not supported");
            return;
        }

        let texture = file_parse_func.unwrap()(binding.to_str().unwrap());

        let mut image: Vec<u32> = vec![0; texture.width as usize * texture.height as usize];

        let start = Instant::now();
        texture_decode_func.unwrap()(
            &texture.data,
            texture.width as usize,
            texture.height as usize,
            &mut image,
        );
        let duration = start.elapsed();
        println!(
            "{}.{:03} seconds",
            duration.as_secs(),
            duration.subsec_millis()
        );

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/tests/decompressed");
        fs::create_dir_all(&d).unwrap();
        d.push(format!("{}.png", texture_format));
        store_u32_bgra(
            &image,
            d.as_path().to_str().unwrap(),
            texture.width as usize,
            texture.height as usize,
        );
    }

    fn get_decode_func(format: &str) -> Option<fn(&[u8], usize, usize, &mut [u32])> {
        Some(match format {
            "ATC_RGB" => decode_atc_rgb4,
            "ATC_RGBA_Explicit" => decode_atc_rgba8,
            "ATC_RGBA_Interpolated" => decode_atc_rgba8,
            "BC1" => decode_bc1,
            "BC3" => decode_bc3,
            "BC4" => decode_bc4,
            "BC5" => decode_bc5,
            "BC6H" => decode_bc6,
            "BC7" => decode_bc7,
            "ETC1_RGB" => decode_etc1,
            "ETC2_RGB" => decode_etc2,
            "ETC2_RGBA" => decode_etc2a8,
            "ETC2_RGB_A1" => decode_etc2a1,
            "PVRTCI_2bpp_RGB" => decode_pvrtc_2bpp,
            "PVRTCI_2bpp_RGBA" => decode_pvrtc_2bpp,
            "PVRTCI_4bpp_RGB" => decode_pvrtc_4bpp,
            "PVRTCI_4bpp_RGBA" => decode_pvrtc_4bpp,
            "EAC_R11" => decode_eacr,
            "EAC_RG11" => decode_eacrg,
            _ => return None,
        })
    }

    fn get_parse_func(extension: &str) -> Option<fn(&str) -> Texture> {
        Some(match extension {
            "ktx2" => parse_ktx2,
            "dds" => parse_dds,
            _ => return None,
        })
    }

    fn parse_ktx2(fp: &str) -> Texture {
        let ktx2_data = fs::read(fp).unwrap();
        let reader = ktx2::Reader::new(ktx2_data).expect("Can't create reader"); // Crate instance of reader.

        // Get general texture information.
        let header = reader.header();

        // Read iterator over slices of each mipmap level.
        let levels = reader.levels().collect::<Vec<_>>();

        Texture {
            width: header.pixel_width,
            height: header.pixel_height,
            data: levels[0].to_vec(),
        }
    }

    fn parse_dds(fp: &str) -> Texture {
        let dds_data = fs::read(fp).unwrap();
        let dds = ddsfile::Dds::read(&dds_data[..]).unwrap();

        Texture {
            width: dds.header.width,
            height: dds.header.height,
            data: dds.data,
        }
    }

    fn store_u32_bgra(image: &[u32], path: &str, w: usize, h: usize) {
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
            w as u32,
            h as u32,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }
}

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

    fn decode_pvrtc_2bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
        decode_pvrtc(data, m_width, m_height, image, true);
    }
    fn decode_pvrtc_4bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
        decode_pvrtc(data, m_width, m_height, image, false);
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
        let ktx2_data = fs::read(entry.path()).unwrap();
        let reader = ktx2::Reader::new(ktx2_data).expect("Can't create reader"); // Crate instance of reader.

        // Get general texture information.
        let header = reader.header();

        // Read iterator over slices of each mipmap level.
        let levels = reader.levels().collect::<Vec<_>>();

        let binding = entry.path();
        let basename = Path::new(binding.to_str().unwrap())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split_once('.')
            .unwrap()
            .0;

        let image = to_image(
            basename,
            levels[0],
            header.pixel_width as usize,
            header.pixel_height as usize,
        );

        if image.is_none() {
            return;
        }
        let image = image.unwrap();

        println!("KTX2 file: {:?}", entry.path());
        println!("  Texture format: {:?}", header.format);
        println!(
            "  Texture size: {}x{}",
            header.pixel_width, header.pixel_height
        );
        println!("  Number of levels: {}", levels.len());
        println!("  Number of layers: {}", header.layer_count);
        println!("  Number of faces: {}", header.face_count);
        println!("  Basename: {}", basename);

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/tests/decompressed");
        fs::create_dir_all(&d).unwrap();
        d.push(format!("{}.png", basename));
        store_u32_bgra(
            &image,
            d.as_path().to_str().unwrap(),
            header.pixel_width as usize,
            header.pixel_height as usize,
        )
    }

    fn to_image(format: &str, data: &[u8], m_width: usize, m_height: usize) -> Option<Vec<u32>> {
        let mut image: Vec<u32> = vec![0; m_width * m_height];

        let func = match format {
            "BC1" => decode_bc1,
            "BC3" => decode_bc3,
            "BC4" => decode_bc4,
            "BC5" => decode_bc5,
            "BC6H" => decode_bc6,
            "BC7" => decode_bc7,
            "ETC2_RGB" => decode_etc2,
            "ETC2_RGBA" => decode_etc2a8,
            "ETC2_RGB_A1" => decode_etc2a1,
            "PVRTCI_2bpp_RGB" => decode_pvrtc_2bpp,
            "PVRTCI_2bpp_RGBA" => decode_pvrtc_2bpp,
            "PVRTCI_4bpp_RGB" => decode_pvrtc_4bpp,
            "PVRTCI_4bpp_RGBA" => decode_pvrtc_4bpp,
            "EAC_R11" => decode_eacr,
            "EAC_RG11" => decode_eacrg,
            _ => {
                println!("  Unsupported format: {:?}", format);
                return None;
            }
        };

        let start = Instant::now();
        func(data, m_width, m_height, &mut image);
        let duration = start.elapsed();
        println!(
            "{}.{:03} seconds",
            duration.as_secs(),
            duration.subsec_millis()
        );
        Some(image)
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

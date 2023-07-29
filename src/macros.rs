// macro to generate generic block decoder functions
macro_rules! block_decoder{
    ($name: expr, $block_width: expr, $block_height: expr, $raw_block_size: expr, $block_decode_func: expr) => {
        paste::item! {
            #[doc = "Decodes a " $name " encoded texture into an image"]
            pub fn [<decode_ $name>](data: &[u8], width: usize, height: usize, image: &mut [u32]) -> Result<(), &'static str> {
                const BLOCK_WIDTH: usize = $block_width;
                const BLOCK_HEIGHT: usize = $block_height;
                const BLOCK_SIZE: usize = BLOCK_WIDTH * BLOCK_HEIGHT;
                let num_blocks_x: usize = (width + BLOCK_WIDTH - 1) / BLOCK_WIDTH;
                let num_blocks_y: usize = (height + BLOCK_WIDTH - 1) / BLOCK_HEIGHT;
                let mut buffer: [u32; BLOCK_SIZE] = [crate::color::color(0,0,0,255); BLOCK_SIZE];

                if data.len() < num_blocks_x * num_blocks_y * $raw_block_size {
                    return Err("Not enough data to decode image!");
                }

                if image.len() < width * height {
                    return Err("Image buffer is too small!");
                }

                let mut data_offset = 0;
                (0..num_blocks_y).for_each(|by| {
                    (0..num_blocks_x).for_each(|bx| {
                        $block_decode_func(&data[data_offset..], &mut buffer);
                        crate::color::copy_block_buffer(
                            bx,
                            by,
                            width,
                            height,
                            BLOCK_WIDTH,
                            BLOCK_HEIGHT,
                            &buffer,
                            image,
                        );
                        data_offset += $raw_block_size;
                    });
                });
                Ok(())
            }

        }
    };
}
pub(crate) use block_decoder;

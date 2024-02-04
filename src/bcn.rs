use crate::macros::block_decoder;

pub(crate) mod bc1;
pub(crate) mod bc2;
pub(crate) mod bc3;
pub(crate) mod bc4;
pub(crate) mod bc5;
pub(crate) mod bc6;
pub(crate) mod bc7;
pub(crate) mod consts;

pub use bc1::decode_bc1_block;
pub use bc2::decode_bc2_block;
pub use bc3::decode_bc3_block;
pub use bc4::decode_bc4_block;
pub use bc5::decode_bc5_block;
pub use bc6::{decode_bc6_block, decode_bc6_block_signed, decode_bc6_block_unsigned};
pub use bc7::decode_bc7_block;

block_decoder!("bc1", 4, 4, 8, decode_bc1_block);
block_decoder!("bc2", 4, 4, 16, decode_bc2_block);
block_decoder!("bc3", 4, 4, 16, decode_bc3_block);
block_decoder!("bc4", 4, 4, 8, decode_bc4_block);
block_decoder!("bc5", 4, 4, 16, decode_bc5_block);
block_decoder!("bc6_signed", 4, 4, 16, decode_bc6_block_signed);
block_decoder!("bc6_unsigned", 4, 4, 16, decode_bc6_block_unsigned);
block_decoder!("bc7", 4, 4, 16, decode_bc7_block);

pub fn decode_bc6(
    data: &[u8],
    width: usize,
    height: usize,
    image: &mut [u32],
    signed: bool,
) -> Result<(), &'static str> {
    match signed {
        true => decode_bc6_signed(data, width, height, image),
        false => decode_bc6_unsigned(data, width, height, image),
    }
}

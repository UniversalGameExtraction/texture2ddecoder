use super::crn_consts::*;
use super::crn_unpacker::*;
use super::CrnFormat;
extern crate alloc;

// #[repr(C)]
// pub struct crn_file_info{
//     struct_size: u32,
//     actual_data_size: u32,
//     header_size: u32,
//     total_palette_size: u32,
//     tables_size: u32,
//     levels: u32,
//     level_compressed_size: [u32; cCRNMaxLevels as usize],
//     color_endpoint_palette_entries: u32,
//     color_selector_palette_entries: u32,
//     alpha_endpoint_palette_entries: u32,
//     alpha_selector_palette_entries: u32
// }

// impl crn_file_info{
//     pub fn default() -> crn_file_info{
//         return crn_file_info {
//             struct_size: core::mem::size_of::<crn_file_info>() as u32,
//             actual_data_size: 0,
//             header_size: 0,
//             total_palette_size: 0,
//             tables_size: 0,
//             levels: 0,
//             level_compressed_size: [0; cCRNMaxLevels as usize],
//             color_endpoint_palette_entries: 0,
//             color_selector_palette_entries: 0,
//             alpha_endpoint_palette_entries: 0,
//             alpha_selector_palette_entries: 0
//         }
//     }
// }

#[repr(C)]
pub struct CrnPackedUint<const N: usize> {
    pub buf: [u8; N],
}

// no-std, so we can not use std::ops
impl<const N: usize> CrnPackedUint<N> {
    pub fn assign_from_buffer(&mut self, other: &[u8]) {
        self.buf.copy_from_slice(&other[0..N])
    }

    pub fn cast_to_uint(&mut self) -> u32 {
        match N {
            1 => self.buf[0] as u32,
            2 => u16::from_be_bytes([self.buf[0], self.buf[1]]) as u32,
            3 => (self.buf[0] as u32) << 16 | u16::from_be_bytes([self.buf[1], self.buf[2]]) as u32,
            4 => u32::from_be_bytes([self.buf[0], self.buf[1], self.buf[2], self.buf[3]]),
            _ => panic!("Packed integer can hold a 4 byte buffer at max!"),
        }
    }
}

impl<const N: usize> Default for CrnPackedUint<N> {
    fn default() -> Self {
        CrnPackedUint { buf: [0; N] }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct CrnPalette {
    pub ofs: CrnPackedUint<3>,
    pub size: CrnPackedUint<3>,
    pub num: CrnPackedUint<2>,
}

impl CrnPalette {
    pub fn assign_from_buffer(&mut self, other: &[u8]) {
        self.ofs.assign_from_buffer(&other[0..]);
        self.size.assign_from_buffer(&other[3..]);
        self.num.assign_from_buffer(&other[6..]);
    }
}

#[derive(Default)]
#[repr(C)]
pub struct CrnHeader {
    pub sig: CrnPackedUint<2>,
    pub header_size: CrnPackedUint<2>,
    pub header_crc16: CrnPackedUint<2>,

    pub data_size: CrnPackedUint<4>,
    pub data_crc16: CrnPackedUint<2>,

    pub width: CrnPackedUint<2>,
    pub height: CrnPackedUint<2>,

    pub levels: CrnPackedUint<1>,
    pub faces: CrnPackedUint<1>,

    pub format: CrnPackedUint<1>,
    pub flags: CrnPackedUint<2>,

    pub reserved: CrnPackedUint<4>,
    pub userdata0: CrnPackedUint<4>,
    pub userdata1: CrnPackedUint<4>,

    pub color_endpoints: CrnPalette,
    pub color_selectors: CrnPalette,

    pub alpha_endpoints: CrnPalette,
    pub alpha_selectors: CrnPalette,

    pub tables_size: CrnPackedUint<2>,
    pub tables_ofs: CrnPackedUint<3>,

    pub level_ofs: alloc::vec::Vec<CrnPackedUint<4>>,
}

impl CrnHeader {
    // size of CrnHeader in C++, which uses a hacky struct definition which shouldn't be used in Rust
    pub const MIN_SIZE: u32 = 74;

    pub fn crnd_get_header(&mut self, p_data: &[u8], data_size: u32) -> bool {
        if data_size < CrnHeader::MIN_SIZE {
            return false;
        }
        *self = CrnHeader::default();
        self.sig.assign_from_buffer(&p_data[0..]);
        self.header_size.assign_from_buffer(&p_data[2..]);
        self.header_crc16.assign_from_buffer(&p_data[4..]);
        self.data_size.assign_from_buffer(&p_data[6..]);
        self.data_crc16.assign_from_buffer(&p_data[10..]);
        self.width.assign_from_buffer(&p_data[12..]);
        self.height.assign_from_buffer(&p_data[14..]);
        self.levels.assign_from_buffer(&p_data[16..]);
        self.faces.assign_from_buffer(&p_data[17..]);
        self.format.assign_from_buffer(&p_data[18..]);
        self.flags.assign_from_buffer(&p_data[19..]);
        self.reserved.assign_from_buffer(&p_data[21..]);
        self.userdata0.assign_from_buffer(&p_data[25..]);
        self.userdata1.assign_from_buffer(&p_data[29..]);
        self.color_endpoints.assign_from_buffer(&p_data[33..]);
        self.color_selectors.assign_from_buffer(&p_data[41..]);
        self.alpha_endpoints.assign_from_buffer(&p_data[49..]);
        self.alpha_selectors.assign_from_buffer(&p_data[57..]);
        self.tables_size.assign_from_buffer(&p_data[65..]);
        self.tables_ofs.assign_from_buffer(&p_data[67..]);
        self.level_ofs = alloc::vec![];
        for i in 0..self.levels.cast_to_uint() as usize {
            self.level_ofs.push(CrnPackedUint { buf: [0, 0, 0, 0] });
            self.level_ofs[i].assign_from_buffer(&p_data[70 + (i * 4)..]);
        }
        if self.sig.cast_to_uint() as u16 != CRNSIG_VALUE {
            return false;
        }
        if self.header_size.cast_to_uint() < CrnHeader::MIN_SIZE
            || data_size < self.data_size.cast_to_uint()
        {
            return false;
        }
        true
    }
}

// #[repr(C)]
// pub struct crn_level_info{
//     struct_size: u32,
//     width: u32,
//     height: u32,
//     faces: u32,
//     blocks_x: u32,
//     blocks_y: u32,
//     bytes_per_block: u32,
//     format: crn_format,
// }

// impl crn_level_info{
//     pub fn default() -> crn_level_info{
//         return crn_level_info {
//             struct_size: core::mem::size_of::<crn_level_info>() as u32,
// 			    width: 0,
// 			    height: 0,
// 			    faces: 0,
// 			    blocks_x: 0,
// 			    blocks_y: 0,
// 			    bytes_per_block: 0,
// 			    format: crn_format::cCRNFmtInvalid // Init as invalid?
//         }
//     }
// }

pub fn crnd_unpack_begin(p_data: &[u8], data_size: u32) -> Result<CrnUnpacker, &'static str> {
    if data_size < CRNHEADER_MIN_SIZE as u32 {
        return Err("Data size is below the minimum allowed.");
    }
    let mut p = CrnUnpacker::default();
    if !p.init(p_data, data_size) {
        return Err("Failed to initialize Crunch decompressor.");
    }
    Ok(p)
}

pub fn crnd_get_crn_format_bits_per_texel(fmt: &mut CrnFormat) -> Result<u32, &'static str> {
    match fmt {
        CrnFormat::Dxt1 | CrnFormat::Dxt5a | CrnFormat::Etc1 => Ok(4),

        CrnFormat::Dxt3
        | CrnFormat::CCrnfmtDxt5
        | CrnFormat::DxnXy
        | CrnFormat::DxnYx
        | CrnFormat::Dxt5CcxY
        | CrnFormat::Dxt5XGxR
        | CrnFormat::Dxt5XGbr
        | CrnFormat::Dxt5Agbr => Ok(8),

        _ => Err("Texture format is not supported."),
    }
}

pub fn crnd_get_bytes_per_dxt_block(fmt: &mut CrnFormat) -> Result<u32, &'static str> {
    Ok((crnd_get_crn_format_bits_per_texel(fmt)? << 4) >> 3)
}

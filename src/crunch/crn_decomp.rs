use super::CrnFormat;
use super::crn_consts::*;
use super::crn_unpacker::*;
extern crate alloc;

// #[repr(C)]
// pub struct crn_file_info{
//     m_struct_size: u32,
//     m_actual_data_size: u32,
//     m_header_size: u32,
//     m_total_palette_size: u32,
//     m_tables_size: u32,
//     m_levels: u32,
//     m_level_compressed_size: [u32; cCRNMaxLevels as usize],
//     m_color_endpoint_palette_entries: u32,
//     m_color_selector_palette_entries: u32,
//     m_alpha_endpoint_palette_entries: u32,
//     m_alpha_selector_palette_entries: u32
// }

// impl crn_file_info{
//     pub fn default() -> crn_file_info{
//         return crn_file_info { 
//             m_struct_size: core::mem::size_of::<crn_file_info>() as u32,
//             m_actual_data_size: 0,
//             m_header_size: 0,
//             m_total_palette_size: 0,
//             m_tables_size: 0,
//             m_levels: 0,
//             m_level_compressed_size: [0; cCRNMaxLevels as usize],
//             m_color_endpoint_palette_entries: 0,
//             m_color_selector_palette_entries: 0,
//             m_alpha_endpoint_palette_entries: 0,
//             m_alpha_selector_palette_entries: 0
//         }
//     }
// }

#[repr(C)]
pub struct CrnPackedUint<const N: usize>{
    pub m_buf: [u8; N]
}

// no-std, so we can not use std::ops
impl<const N: usize> CrnPackedUint<N>{
    pub fn assign_from_buffer(&mut self, other: &[u8]){
        self.m_buf.copy_from_slice(&other[0..N])
    }

    pub fn cast_to_uint(&mut self) -> u32{
        match N {
            1 => self.m_buf[0] as u32,
            2 => u16::from_be_bytes([self.m_buf[0], self.m_buf[1]]) as u32,
            3 => (self.m_buf[0] as u32) << 16 | u16::from_be_bytes([self.m_buf[1], self.m_buf[2]]) as u32,
            4 => u32::from_be_bytes([self.m_buf[0], self.m_buf[1], self.m_buf[2], self.m_buf[3]]),
            _ => panic!("Packed integer can hold a 4 byte buffer at max!")
        }
    }

}

impl<const N: usize> Default for CrnPackedUint<N>{
    fn default() -> Self {
        CrnPackedUint{
            m_buf: [0; N]
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct CrnPalette{
   pub m_ofs: CrnPackedUint<3>,
   pub m_size: CrnPackedUint<3>,
   pub m_num: CrnPackedUint<2>
}

impl CrnPalette{
    pub fn assign_from_buffer(&mut self, other: &[u8]){
        self.m_ofs.assign_from_buffer(&other[0..]);
        self.m_size.assign_from_buffer(&other[3..]);
        self.m_num.assign_from_buffer(&other[6..]);
    }
}

#[derive(Default)]
#[repr(C)]
pub struct CrnHeader{
    pub m_sig: CrnPackedUint<2>,
    pub m_header_size: CrnPackedUint<2>,
    pub m_header_crc16: CrnPackedUint<2>,

    pub m_data_size: CrnPackedUint<4>,
    pub m_data_crc16: CrnPackedUint<2>,

    pub m_width: CrnPackedUint<2>,
    pub m_height: CrnPackedUint<2>,

    pub m_levels: CrnPackedUint<1>,
    pub m_faces: CrnPackedUint<1>,

    pub m_format: CrnPackedUint<1>,
    pub m_flags: CrnPackedUint<2>,

    pub m_reserved: CrnPackedUint<4>,
    pub m_userdata0: CrnPackedUint<4>,
    pub m_userdata1: CrnPackedUint<4>,

    pub m_color_endpoints: CrnPalette,
    pub m_color_selectors: CrnPalette,

    pub m_alpha_endpoints: CrnPalette,
    pub m_alpha_selectors: CrnPalette,

    pub m_tables_size: CrnPackedUint<2>,
    pub m_tables_ofs: CrnPackedUint<3>,

    pub m_level_ofs: alloc::vec::Vec<CrnPackedUint<4>>
}

impl CrnHeader{
    pub fn crnd_get_header(&mut self, p_data: &[u8], data_size: u32) -> bool{
        if data_size < (core::mem::size_of::<CrnHeader>() - 8 + 4) as u32{
            return false;
        }
        *self = CrnHeader::default();
        self.m_sig.assign_from_buffer(&p_data[0..]);
        self.m_header_size.assign_from_buffer(&p_data[2..]);
        self.m_header_crc16.assign_from_buffer(&p_data[4..]);
        self.m_data_size.assign_from_buffer(&p_data[6..]);
        self.m_data_crc16.assign_from_buffer(&p_data[10..]);
        self.m_width.assign_from_buffer(&p_data[12..]);
        self.m_height.assign_from_buffer(&p_data[14..]);
        self.m_levels.assign_from_buffer(&p_data[16..]);
        self.m_faces.assign_from_buffer(&p_data[17..]);
        self.m_format.assign_from_buffer(&p_data[18..]);
        self.m_flags.assign_from_buffer(&p_data[19..]);
        self.m_reserved.assign_from_buffer(&p_data[21..]);
        self.m_userdata0.assign_from_buffer(&p_data[25..]);
        self.m_userdata1.assign_from_buffer(&p_data[29..]);
        self.m_color_endpoints.assign_from_buffer(&p_data[33..]);
        self.m_color_selectors.assign_from_buffer(&p_data[41..]);
        self.m_alpha_endpoints.assign_from_buffer(&p_data[49..]);
        self.m_alpha_selectors.assign_from_buffer(&p_data[57..]);
        self.m_tables_size.assign_from_buffer(&p_data[65..]);
        self.m_tables_ofs.assign_from_buffer(&p_data[67..]);
        self.m_level_ofs = alloc::vec![];
        for i in 0..self.m_levels.cast_to_uint() as usize{
            self.m_level_ofs.push(CrnPackedUint { m_buf: [0, 0, 0, 0] });
            self.m_level_ofs[i].assign_from_buffer(&p_data[70 + (i * 4)..]);
        }
        if self.m_sig.cast_to_uint() as u16 != C_CRNSIG_VALUE{
            return false;
        }
        if self.m_header_size.cast_to_uint() < core::mem::size_of::<CrnHeader>() as u32 || data_size < self.m_data_size.cast_to_uint(){
            return false;
        }
        true
    }
}

// #[repr(C)]
// pub struct crn_level_info{
//     m_struct_size: u32,
//     m_width: u32,
//     m_height: u32,
//     m_faces: u32,
//     m_blocks_x: u32,
//     m_blocks_y: u32,
//     m_bytes_per_block: u32,
//     m_format: crn_format,
// }

// impl crn_level_info{
//     pub fn default() -> crn_level_info{
//         return crn_level_info {
//             m_struct_size: core::mem::size_of::<crn_level_info>() as u32,
// 			m_width: 0,
// 			m_height: 0,
// 			m_faces: 0,
// 			m_blocks_x: 0,
// 			m_blocks_y: 0,
// 			m_bytes_per_block: 0,
// 			m_format: crn_format::cCRNFmtInvalid // Init as invalid?
//         }
//     }
// }

pub fn crnd_unpack_begin(p_data: &[u8], data_size: u32) -> Result<CrnUnpacker, &'static str>{
    if data_size < C_CRNHEADER_MIN_SIZE as u32{
        return Err("Data size is below the minimum allowed.");
    }
    let mut p = CrnUnpacker::default();
    if p.init(p_data, data_size) == false{
        return Err("Failed to initialize Crunch decompressor.");
    }
    Ok(p)
}

pub fn crnd_get_crn_format_bits_per_texel(fmt: &mut CrnFormat) -> Result<u32, &'static str>{
    match fmt {
        CrnFormat::CCrnfmtDxt1 |
        CrnFormat::CCrnfmtDxt5a |
        CrnFormat::CCrnfmtEtc1 => Ok(4),

        CrnFormat::CCrnfmtDxt3 |
        CrnFormat::CCrnfmtDxt5 |
        CrnFormat::CCrnfmtDxnXy |
        CrnFormat::CCrnfmtDxnYx |
        CrnFormat::CCrnfmtDxt5CcxY |
        CrnFormat::CCrnfmtDxt5XGxR |
        CrnFormat::CCrnfmtDxt5XGbr |
        CrnFormat::CCrnfmtDxt5Agbr => Ok(8),

        _ => Err("Texture format is not supported.")
    }
}

pub fn crnd_get_bytes_per_dxt_block(fmt: &mut CrnFormat) -> Result<u32, &'static str>{
    Ok((match crnd_get_crn_format_bits_per_texel(fmt){
        Ok(s) => s,
        Err(e) => return Err(e)
    } << 4) >> 3)
}

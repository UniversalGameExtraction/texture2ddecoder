#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::crnlib::*;
use super::crn_consts::*;
use super::crn_unpacker::*;
extern crate alloc;

#[repr(C)]
pub struct crn_texture_info {
    pub m_struct_size: u32,
    pub m_width: u32,
    pub m_height: u32,
    pub m_levels: u32,
    pub m_faces: u32,
    pub m_bytes_per_block: u32,
    pub m_userdata0: u32,
    pub m_userdata1: u32,
    pub m_format: crn_format
}

impl crn_texture_info{
    pub fn default() -> crn_texture_info{
        return crn_texture_info {
            m_struct_size: core::mem::size_of::<crn_texture_info>() as u32,
			m_width: 0,
			m_height: 0,
			m_levels: 0,
			m_faces: 0,
			m_bytes_per_block: 0,
			m_userdata0: 0,
			m_userdata1: 0,
			m_format: crn_format::cCRNFmtInvalid // Init as invalid?
        }
    }

    pub fn crnd_get_texture_info(&mut self, pData: &[u8], data_size: u32) -> bool{
        if data_size < core::mem::size_of::<crn_header>() as u32 {
            return false;
        }

        if self.m_struct_size != core::mem::size_of::<crn_texture_info>() as u32{
            return false;
        }

        let mut pHeader: crn_header = crn_header::default();
        let res: bool = pHeader.crnd_get_header(pData, data_size); 
        if res == false {
            return res;
        }

        self.m_width = pHeader.m_width.cast_to_uint();
        self.m_height = pHeader.m_height.cast_to_uint();
        self.m_levels = pHeader.m_levels.cast_to_uint();
        self.m_faces = pHeader.m_faces.cast_to_uint();
        self.m_format = match pHeader.m_format.cast_to_uint(){
            // -1 => crn_format::cCRNFmtInvalid,

            0 => crn_format::cCRNFmtDXT1,
        
            // 0 => crn_formatcCRNFmtFirstValid,
        
            // cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
            1 => crn_format::cCRNFmtDXT3,
        
            2 => crn_format::cCRNFmtDXT5,
        
            // Various DXT5 derivatives
            3 => crn_format::cCRNFmtDXT5_CCxY,    // Luma-chroma
            4 => crn_format::cCRNFmtDXT5_xGxR,    // Swizzled 2-component
            5 => crn_format::cCRNFmtDXT5_xGBR,    // Swizzled 3-component
            6 => crn_format::cCRNFmtDXT5_AGBR,    // Swizzled 4-component
        
            // ATI 3DC and X360 DXN
            7 => crn_format::cCRNFmtDXN_XY,
            8 => crn_format::cCRNFmtDXN_YX,
        
            // DXT5 alpha blocks only
            9 => crn_format::cCRNFmtDXT5A,
        
            10 => crn_format::cCRNFmtETC1,
            11 => crn_format::cCRNFmtETC2,
            12 => crn_format::cCRNFmtETC2A,
            13 => crn_format::cCRNFmtETC1S,
            14 => crn_format::cCRNFmtETC2AS,
        
            15 => crn_format::cCRNFmtTotal,
        
            0xFFFFFFFF => crn_format::cCRNFmtForceDWORD,

            _ => crn_format::cCRNFmtInvalid
        };
        if self.m_format == crn_format::cCRNFmtInvalid {
            return false;
        }
        if  (pHeader.m_format.cast_to_uint() == crn_format::cCRNFmtDXT1 as u32) ||
            (pHeader.m_format.cast_to_uint() == crn_format::cCRNFmtDXT5A as u32) ||
            (pHeader.m_format.cast_to_uint() == crn_format::cCRNFmtETC1 as u32) ||
            (pHeader.m_format.cast_to_uint() == crn_format::cCRNFmtETC2 as u32) ||
            (pHeader.m_format.cast_to_uint() == crn_format::cCRNFmtETC1S as u32) {
            self.m_bytes_per_block = 8;
        }else{
            self.m_bytes_per_block = 16;
        }
        self.m_userdata0 = pHeader.m_userdata0.cast_to_uint();
        self.m_userdata1 = pHeader.m_userdata1.cast_to_uint();
        return true;
    }
}

#[repr(C)]
pub struct crn_packed_uint<const N: usize>{
    m_buf: [u8; N]
}

// no-std, so we can not use std::ops
impl<const N: usize> crn_packed_uint<N>{
    pub fn assign(&mut self, other: crn_packed_uint<N>) -> &crn_packed_uint<N>{
        if self.m_buf != other.m_buf{ // this works?
            self.m_buf.copy_from_slice(&other.m_buf[0..N]);
        }
        return self;
    }

    pub fn assign_from_buffer(&mut self, other: &[u8]){
        self.m_buf.copy_from_slice(&other[0..N])
    }

    // Rust doesn't support function overloading.
    pub fn assign_val(&mut self, mut val: u32) -> &crn_packed_uint<N>{
        val <<= 8 * (4 - N);
        for i in 0..N{
            self.m_buf[i] = (val >> 24) as u8;
            val <<= 8; 
        }
        return self;
    }

    pub fn cast_to_uint(&mut self) -> u32{
        return match N {
            1 =>    self.m_buf[0] as u32,
            2 => (((self.m_buf[0] as u32) << 8 ) |   self.m_buf[1]  as u32) as u32,
            3 => (((self.m_buf[0] as u32) << 16) | ((self.m_buf[1]  as u32) << 8 ) | ( self.m_buf[2] as u32)) as u32,
            _ => (((self.m_buf[0] as u32) << 24) | ((self.m_buf[1]  as u32) << 16) | ((self.m_buf[2] as u32) << 8) | (self.m_buf[3] as u32)) as u32,
        }
    }

}

impl<const N: usize> Default for crn_packed_uint<N>{
    fn default() -> Self {
        return crn_packed_uint{
            m_buf: [0; N]
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct crn_palette{
   pub m_ofs: crn_packed_uint<3>,
   pub m_size: crn_packed_uint<3>,
   pub m_num: crn_packed_uint<2>
}

impl crn_palette{
    pub fn assign_from_buffer(&mut self, other: &[u8]){
        self.m_ofs.assign_from_buffer(&other[0..]);
        self.m_size.assign_from_buffer(&other[3..]);
        self.m_num.assign_from_buffer(&other[6..]);
    }
}

#[derive(Default)]
#[repr(C)]
pub struct crn_header{
    // enum { cCRNSigValue = ('H' << 8) | 'x' }
    // cCRNSigValue: u16,

    pub m_sig: crn_packed_uint<2>,
    pub m_header_size: crn_packed_uint<2>,
    pub m_header_crc16: crn_packed_uint<2>,

    pub m_data_size: crn_packed_uint<4>,
    pub m_data_crc16: crn_packed_uint<2>,

    pub m_width: crn_packed_uint<2>,
    pub m_height: crn_packed_uint<2>,

    pub m_levels: crn_packed_uint<1>,
    pub m_faces: crn_packed_uint<1>,

    pub m_format: crn_packed_uint<1>,
    pub m_flags: crn_packed_uint<2>,

    pub m_reserved: crn_packed_uint<4>,
    pub m_userdata0: crn_packed_uint<4>,
    pub m_userdata1: crn_packed_uint<4>,

    pub m_color_endpoints: crn_palette,
    pub m_color_selectors: crn_palette,

    pub m_alpha_endpoints: crn_palette,
    pub m_alpha_selectors: crn_palette,

    pub m_tables_size: crn_packed_uint<2>,
    pub m_tables_ofs: crn_packed_uint<3>,

    pub m_level_ofs: alloc::vec::Vec<crn_packed_uint<4>>
}

impl crn_header{
    pub fn crnd_get_header(&mut self, pData: &[u8], data_size: u32) -> bool{
        if data_size < (core::mem::size_of::<crn_header>() - 8 + 4) as u32{
            return false;
        }
        *self = crn_header::default();
        self.m_sig.assign_from_buffer(&pData[0..]);
        self.m_header_size.assign_from_buffer(&pData[2..]);
        self.m_header_crc16.assign_from_buffer(&pData[4..]);
        self.m_data_size.assign_from_buffer(&pData[6..]);
        self.m_data_crc16.assign_from_buffer(&pData[10..]);
        self.m_width.assign_from_buffer(&pData[12..]);
        self.m_height.assign_from_buffer(&pData[14..]);
        self.m_levels.assign_from_buffer(&pData[16..]);
        self.m_faces.assign_from_buffer(&pData[17..]);
        self.m_format.assign_from_buffer(&pData[18..]);
        self.m_flags.assign_from_buffer(&pData[19..]);
        self.m_reserved.assign_from_buffer(&pData[21..]);
        self.m_userdata0.assign_from_buffer(&pData[25..]);
        self.m_userdata1.assign_from_buffer(&pData[29..]);
        self.m_color_endpoints.assign_from_buffer(&pData[33..]);
        self.m_color_selectors.assign_from_buffer(&pData[41..]);
        self.m_alpha_endpoints.assign_from_buffer(&pData[49..]);
        self.m_alpha_selectors.assign_from_buffer(&pData[57..]);
        self.m_tables_size.assign_from_buffer(&pData[65..]);
        self.m_tables_ofs.assign_from_buffer(&pData[67..]);
        self.m_level_ofs = alloc::vec![];
        for i in 0..self.m_levels.cast_to_uint() as usize{
            self.m_level_ofs.push(crn_packed_uint { m_buf: [0, 0, 0, 0] });
            self.m_level_ofs[i].assign_from_buffer(&pData[70 + (i * 4)..]);
        }
        if self.m_sig.cast_to_uint() as u16 != cCRNSigValue{
            return false;
        }
        if self.m_header_size.cast_to_uint() < core::mem::size_of::<crn_header>() as u32 || data_size < self.m_data_size.cast_to_uint(){
            return false;
        }
        return true;
    }
}

pub fn crnd_unpack_begin(pData: &[u8], data_size: u32) -> Result<crn_unpacker, &'static str>{
    if data_size < cCRNHeaderMinSize{
        return Err("Data size is below the minimum allowed.");
    }
    let mut p = crn_unpacker::default();
    if p.init(pData, data_size) == false{
        return Err("Failed to initialize Crunch decompressor.");
    }
    return Ok(p);
}

pub fn crnd_get_crn_format_bits_per_texel(fmt: &mut crn_format) -> Result<u32, &'static str>{
    return match fmt {
        crn_format::cCRNFmtDXT1 |
        crn_format::cCRNFmtDXT5A |
        crn_format::cCRNFmtETC1 |
        crn_format::cCRNFmtETC2 |
        crn_format::cCRNFmtETC1S => Ok(4),

        crn_format::cCRNFmtDXT3 |
        crn_format::cCRNFmtDXT5 |
        crn_format::cCRNFmtDXN_XY |
        crn_format::cCRNFmtDXN_YX |
        crn_format::cCRNFmtDXT5_CCxY |
        crn_format::cCRNFmtDXT5_xGxR |
        crn_format::cCRNFmtDXT5_xGBR |
        crn_format::cCRNFmtDXT5_AGBR |
        crn_format::cCRNFmtETC2A |
        crn_format::cCRNFmtETC2AS => Ok(8),

        _ => Err("Texture format is not supported.")
    };
}

pub fn crnd_get_bytes_per_dxt_block(fmt: &mut crn_format) -> Result<u32, &'static str>{
    return Ok((match crnd_get_crn_format_bits_per_texel(fmt){
        Ok(s) => s,
        Err(e) => return Err(e)
    } << 4) >> 3);
}
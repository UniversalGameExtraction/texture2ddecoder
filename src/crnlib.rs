#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::crunch::crn_decomp::CrnHeader;

// Supported compressed pixel formats.
// Basically all the standard DX9 formats, with some swizzled DXT5 formats
// (most of them supported by ATI's Compressonator), along with some ATI/X360 GPU specific formats.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_format {
    cCRNFmtInvalid = -1,

    cCRNFmtDXT1 = 0,

    // cCRNFmtFirstValid = crn_format::cCRNFmtDXT1 as isize, // Rust doesn't allow same value enums, and as far as I see this is not used in the lib.

    // cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
    cCRNFmtDXT3,

    cCRNFmtDXT5,

    // Various DXT5 derivatives
    cCRNFmtDXT5_CCxY,    // Luma-chroma
    cCRNFmtDXT5_xGxR,    // Swizzled 2-component
    cCRNFmtDXT5_xGBR,    // Swizzled 3-component
    cCRNFmtDXT5_AGBR,    // Swizzled 4-component

    // ATI 3DC and X360 DXN
    cCRNFmtDXN_XY,
    cCRNFmtDXN_YX,

    // DXT5 alpha blocks only
    cCRNFmtDXT5A,

    cCRNFmtETC1,
    cCRNFmtETC2,
    cCRNFmtETC2A,
    cCRNFmtETC1S,
    cCRNFmtETC2AS,

    cCRNFmtTotal,

    cCRNFmtForceDWORD = 0xFFFFFFFF
}

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
        if data_size < core::mem::size_of::<CrnHeader>() as u32 {
            return false;
        }

        if self.m_struct_size != core::mem::size_of::<crn_texture_info>() as u32{
            return false;
        }

        let mut pHeader: CrnHeader = CrnHeader::default();
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
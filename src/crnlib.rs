use crate::crunch::crn_decomp::CrnHeader;

// Supported compressed pixel formats.
// Basically all the standard DX9 formats, with some swizzled DXT5 formats
// (most of them supported by ATI's Compressonator), along with some ATI/X360 GPU specific formats.
#[derive(PartialEq, PartialOrd)]
#[repr(u32)]
pub enum CrnFormat {
    CCrnfmtInvalid = 4294967295, // u32 -1,

    CCrnfmtDxt1 = 0,

    // cCRNFmtFirstValid = crn_format::cCRNFmtDXT1 as isize, // Rust doesn't allow same value enums, and as far as I see this is not used in the lib.

    // cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
    CCrnfmtDxt3,

    CCrnfmtDxt5,

    // Various DXT5 derivatives
    CCrnfmtDxt5CcxY,    // Luma-chroma
    CCrnfmtDxt5XGxR,    // Swizzled 2-component
    CCrnfmtDxt5XGbr,    // Swizzled 3-component
    CCrnfmtDxt5Agbr,    // Swizzled 4-component

    // ATI 3DC and X360 DXN
    CCrnfmtDxnXy,
    CCrnfmtDxnYx,

    // DXT5 alpha blocks only
    CCrnfmtDxt5a,

    CCrnfmtEtc1,
    CCrnfmtEtc2,
    CCrnfmtEtc2a,
    CCrnfmtEtc1s,
    CCrnfmtEtc2as,

    CCrnfmtTotal
}

#[repr(C)]
pub struct CrnTextureInfo {
    pub m_struct_size: u32,
    pub m_width: u32,
    pub m_height: u32,
    pub m_levels: u32,
    pub m_faces: u32,
    pub m_bytes_per_block: u32,
    pub m_userdata0: u32,
    pub m_userdata1: u32,
    pub m_format: CrnFormat
}

impl CrnTextureInfo{
    pub const fn default() -> Self{
        Self {
            m_struct_size: core::mem::size_of::<CrnTextureInfo>() as u32,
			m_width: 0,
			m_height: 0,
			m_levels: 0,
			m_faces: 0,
			m_bytes_per_block: 0,
			m_userdata0: 0,
			m_userdata1: 0,
			m_format: CrnFormat::CCrnfmtInvalid // Init as invalid?
        }
    }

    pub fn crnd_get_texture_info(&mut self, p_data: &[u8], data_size: u32) -> bool{
        if data_size < core::mem::size_of::<CrnHeader>() as u32 {
            return false;
        }

        if self.m_struct_size != core::mem::size_of::<CrnTextureInfo>() as u32{
            return false;
        }

        let mut p_header: CrnHeader = CrnHeader::default();
        let res: bool = p_header.crnd_get_header(p_data, data_size); 
        if !res {
            return res;
        }

        self.m_width = p_header.m_width.cast_to_uint();
        self.m_height = p_header.m_height.cast_to_uint();
        self.m_levels = p_header.m_levels.cast_to_uint();
        self.m_faces = p_header.m_faces.cast_to_uint();
        self.m_format = match p_header.m_format.cast_to_uint(){
            // -1 => crn_format::cCRNFmtInvalid,

            0 => CrnFormat::CCrnfmtDxt1,
        
            // 0 => crn_formatcCRNFmtFirstValid,
        
            // cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
            1 => CrnFormat::CCrnfmtDxt3,
        
            2 => CrnFormat::CCrnfmtDxt5,
        
            // Various DXT5 derivatives
            3 => CrnFormat::CCrnfmtDxt5CcxY,    // Luma-chroma
            4 => CrnFormat::CCrnfmtDxt5XGxR,    // Swizzled 2-component
            5 => CrnFormat::CCrnfmtDxt5XGbr,    // Swizzled 3-component
            6 => CrnFormat::CCrnfmtDxt5Agbr,    // Swizzled 4-component
        
            // ATI 3DC and X360 DXN
            7 => CrnFormat::CCrnfmtDxnXy,
            8 => CrnFormat::CCrnfmtDxnYx,
        
            // DXT5 alpha blocks only
            9 => CrnFormat::CCrnfmtDxt5a,
        
            10 => CrnFormat::CCrnfmtEtc1,
            11 => CrnFormat::CCrnfmtEtc2,
            12 => CrnFormat::CCrnfmtEtc2a,
            13 => CrnFormat::CCrnfmtEtc1s,
            14 => CrnFormat::CCrnfmtEtc2as,
        
            15 => CrnFormat::CCrnfmtTotal,

            _ => CrnFormat::CCrnfmtInvalid
        };
        if self.m_format == CrnFormat::CCrnfmtInvalid {
            return false;
        }
        if  (p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtDxt1 as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtDxt5a as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1 as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2 as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1s as u32) {
            self.m_bytes_per_block = 8;
        }else{
            self.m_bytes_per_block = 16;
        }
        self.m_userdata0 = p_header.m_userdata0.cast_to_uint();
        self.m_userdata1 = p_header.m_userdata1.cast_to_uint();
        true
    }
}
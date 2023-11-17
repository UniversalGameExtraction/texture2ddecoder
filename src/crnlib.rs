use crate::crunch::crn_decomp::CrnHeader;

// Supported compressed pixel formats.
// Basically all the standard DX9 formats, with some swizzled DXT5 formats
// (most of them supported by ATI's Compressonator), along with some ATI/X360 GPU specific formats.
#[derive(PartialEq, PartialOrd)]
#[repr(u32)]
pub enum CrnFormat {
    Invalid = 4294967295, // u32 -1,

    Dxt1 = 0,

    // cCRNFmtFirstValid = crn_format::cCRNFmtDXT1 as isize, // Rust doesn't allow same value enums, and as far as I see this is not used in the lib.

    // cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
    Dxt3,

    CCrnfmtDxt5,

    // Various DXT5 derivatives
    Dxt5CcxY,    // Luma-chroma
    Dxt5XGxR,    // Swizzled 2-component
    Dxt5XGbr,    // Swizzled 3-component
    Dxt5Agbr,    // Swizzled 4-component

    // ATI 3DC and X360 DXN
    DxnXy,
    DxnYx,

    // DXT5 alpha blocks only
    Dxt5a,

    Etc1,
    Etc2,
    Etc2a,
    Etc1s,
    Etc2as,

    Total
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
			m_format: CrnFormat::Invalid // Init as invalid?
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

            0 => CrnFormat::Dxt1,
        
            // 0 => crn_formatcCRNFmtFirstValid,
        
            // cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
            1 => CrnFormat::Dxt3,
        
            2 => CrnFormat::CCrnfmtDxt5,
        
            // Various DXT5 derivatives
            3 => CrnFormat::Dxt5CcxY,    // Luma-chroma
            4 => CrnFormat::Dxt5XGxR,    // Swizzled 2-component
            5 => CrnFormat::Dxt5XGbr,    // Swizzled 3-component
            6 => CrnFormat::Dxt5Agbr,    // Swizzled 4-component
        
            // ATI 3DC and X360 DXN
            7 => CrnFormat::DxnXy,
            8 => CrnFormat::DxnYx,
        
            // DXT5 alpha blocks only
            9 => CrnFormat::Dxt5a,
        
            10 => CrnFormat::Etc1,
            11 => CrnFormat::Etc2,
            12 => CrnFormat::Etc2a,
            13 => CrnFormat::Etc1s,
            14 => CrnFormat::Etc2as,
        
            15 => CrnFormat::Total,

            _ => CrnFormat::Invalid
        };
        if self.m_format == CrnFormat::Invalid {
            return false;
        }
        if  (p_header.m_format.cast_to_uint() == CrnFormat::Dxt1 as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::Dxt5a as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::Etc1 as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::Etc2 as u32) ||
            (p_header.m_format.cast_to_uint() == CrnFormat::Etc1s as u32) {
            self.m_bytes_per_block = 8;
        }else{
            self.m_bytes_per_block = 16;
        }
        self.m_userdata0 = p_header.m_userdata0.cast_to_uint();
        self.m_userdata1 = p_header.m_userdata1.cast_to_uint();
        true
    }
}
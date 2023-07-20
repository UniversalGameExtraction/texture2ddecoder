#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

// crnlib can compress to these file types.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_file_type{
    // .CRN
    cCRNFileTypeCRN = 0,
    
    // .DDS using regular DXT or clustered DXT
    cCRNFileTypeDDS,
    
    cCRNFileTypeForceDWORD = 0xFFFFFFFF
}

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

    cCRNFmtTotal,

    cCRNFmtForceDWORD = 0xFFFFFFFF
}

// Various library/file format limits.
// pub struct crn_limits {
    // Max. mipmap level resolution on any axis.
    pub const cCRNMaxLevelResolution: u32     = 4096;

    pub const cCRNMinPaletteSize: u32         = 8;
    pub const cCRNMaxPaletteSize: u32         = 8192;

    pub const cCRNMaxFaces: u32               = 6;
    pub const cCRNMaxLevels: u32              = 16;

    pub const cCRNMaxHelperThreads: u32       = 16;

    pub const cCRNMinQualityLevel: u32        = 0;
    pub const cCRNMaxQualityLevel: u32        = 255;
// }

// CRN/DDS compression flags.
// See the m_flags member in the crn_comp_params struct, below.
// pub enum crn_comp_flags{
    // Enables perceptual colorspace distance metrics if set.
    // Important: Be sure to disable this when compressing non-sRGB colorspace images, like normal maps!
    // Default: Set
    pub const cCRNCompFlagPerceptual: u32 = 1;

    // Enables (up to) 8x8 macroblock usage if set. If disabled, only 4x4 blocks are allowed.
    // Compression ratio will be lower when disabled, but may cut down on blocky artifacts because the process used to determine
    // where large macroblocks can be used without artifacts isn't perfect.
    // Default: Set.
    pub const cCRNCompFlagHierarchical: u32 = 2;

    // cCRNCompFlagQuick disables several output file optimizations - intended for things like quicker previews.
    // Default: Not set.
    pub const cCRNCompFlagQuick: u32 = 4;

    // DXT1: OK to use DXT1 alpha blocks for better quality or DXT1A transparency.
    // DXT5: OK to use both DXT5 block types.
    // Currently only used when writing to .DDS files, as .CRN uses only a subset of the possible DXTn block types.
    // Default: Set.
    pub const cCRNCompFlagUseBothBlockTypes: u32 = 8;

    // OK to use DXT1A transparent indices to encode black (assumes pixel shader ignores fetched alpha).
    // Currently only used when writing to .DDS files, .CRN never uses alpha blocks.
    // Default: Not set.
    pub const cCRNCompFlagUseTransparentIndicesForBlack: u32 = 16;

    // Disables endpoint caching, for more deterministic output.
    // Currently only used when writing to .DDS files.
    // Default: Not set.
    pub const cCRNCompFlagDisableEndpointCaching: u32 = 32;

    // If enabled, use the cCRNColorEndpointPaletteSize, etc. params to control the CRN palette sizes. Only useful when writing to .CRN files.
    // Default: Not set.
    pub const cCRNCompFlagManualPaletteSizes: u32 = 64;

    // If enabled, DXT1A alpha blocks are used to encode single bit transparency.
    // Default: Not set.
    pub const cCRNCompFlagDXT1AForTransparency: u32 = 128;

    // If enabled, the DXT1 compressor's color distance metric assumes the pixel shader will be converting the fetched RGB results to luma (Y part of YCbCr).
    // This increases quality when compressing grayscale images, because the compressor can spread the luma error amoung all three channels (i.e. it can generate blocks
    // with some chroma present if doing so will ultimately lead to lower luma error).
    // Only enable on grayscale source images.
    // Default: Not set.
    pub const cCRNCompFlagGrayscaleSampling: u32 = 256;

    // If enabled, debug information will be output during compression.
    // Default: Not set.
    pub const cCRNCompFlagDebugging: u32 = 0x80000000;

    pub const cCRNCompFlagForceDWORD: u32 = 0xFFFFFFFF;
// }

// Controls DXTn quality vs. speed control - only used when compressing to .DDS.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_dxt_quality {
   cCRNDXTQualitySuperFast,
   cCRNDXTQualityFast,
   cCRNDXTQualityNormal,
   cCRNDXTQualityBetter,
   cCRNDXTQualityUber,

   cCRNDXTQualityTotal,

   cCRNDXTQualityForceDWORD = 0xFFFFFFFF
}

// Which DXTn compressor to use when compressing to plain (non-clustered) .DDS.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_dxt_compressor_type {
   cCRNDXTCompressorCRN,      // Use crnlib's ETC1 or DXTc block compressor (default, highest quality, comparable or better than ati_compress or squish, and crnlib's ETC1 is a lot fasterw with similiar quality to Erricson's)
   cCRNDXTCompressorCRNF,     // Use crnlib's "fast" DXTc block compressor
   cCRNDXTCompressorRYG,      // Use RYG's DXTc block compressor (low quality, but very fast)

/* Seems to not be initialized, check later TODO */
// #if CRNLIB_SUPPORT_ATI_COMPRESS
//    cCRNDXTCompressorATI,
// #endif

// #if CRNLIB_SUPPORT_SQUISH
//    cCRNDXTCompressorSquish,
// #endif

   cCRNTotalDXTCompressors,

   cCRNDXTCompressorForceDWORD = 0xFFFFFFFF
}

// Progress callback function.
// Processing will stop prematurely (and fail) if the callback returns false.
// phase_index, total_phases - high level progress
// subphase_index, total_subphases - progress within current phase
type crn_progress_callback_func = *mut fn(phase_index: u32, total_phases: u32, subphase_index: u32, total_subphases: u32, pUser_data_ptr: *mut usize) -> u32;

// CRN/DDS compression parameters struct.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub struct crn_comp_params{
    m_size_of_obj: u32,
    m_file_type: crn_file_type, // Output file type: cCRNFileTypeCRN or cCRNFileTypeDDS.            
    m_faces: u32,               // 1 (2D map) or 6 (cubemap)
    m_width: u32,               // [1,cCRNMaxLevelResolution], non-power of 2 OK, non-square OK
    m_height: u32,              // [1,cCRNMaxLevelResolution], non-power of 2 OK, non-square OK
    m_levels: u32,              // [1,cCRNMaxLevelResolution], non-power of 2 OK, non-square OK
    m_format: crn_format,       // Output pixel format.        
    m_flags: u32,               // see crn_comp_flags enum
    
    // Array of pointers to 32bpp input images.
    m_pImages: [[*mut usize; cCRNMaxLevels as usize]; cCRNMaxFaces as usize],
    
    // Target bitrate - if non-zero, the compressor will use an interpolative search to find the
    // highest quality level that is <= the target bitrate. If it fails to find a bitrate high enough, it'll
    // try disabling adaptive block sizes (cCRNCompFlagHierarchical flag) and redo the search. This process can be pretty slow.    
    m_target_bitrate: f32,

    // Desired quality level.
    // Currently, CRN and DDS quality levels are not compatible with eachother from an image quality standpoint.
    m_quality_level: u32, // [cCRNMinQualityLevel, cCRNMaxQualityLevel]

    // DXTn compression parameters.
    m_dxt1a_alpha_threshold: u32,
    m_dxt_quality: crn_dxt_quality,
    m_dxt_compressor_type: crn_dxt_compressor_type,
    
    // Alpha channel's component. Defaults to 3.
    m_alpha_component: u32,

    // Various low-level CRN specific parameters.
    m_crn_adaptive_tile_color_psnr_derating: f32,
    m_crn_adaptive_tile_alpha_psnr_derating: f32,

    m_crn_color_endpoint_palette_size: u32, // [cCRNMinPaletteSize,cCRNMaxPaletteSize]
    m_crn_color_selector_palette_size: u32, // [cCRNMinPaletteSize,cCRNMaxPaletteSize]

    m_crn_alpha_endpoint_palette_size: u32, // [cCRNMinPaletteSize,cCRNMaxPaletteSize]
    m_crn_alpha_selector_palette_size: u32, // [cCRNMinPaletteSize,cCRNMaxPaletteSize]

    // Number of helper threads to create during compression. 0=no threading.
    m_num_helper_threads: u32,
    
    // CRN userdata0 and userdata1 members, which are written directly to the header of the output file.
    m_userdata0: u32,
    m_userdata1: u32,
    
    // User provided progress callback.
    m_pProgress_func: crn_progress_callback_func,
    m_pProgress_func_data: /* void* */ *mut usize
}


impl crn_comp_params{
    #[inline]
    pub fn default() -> Self{
        // Good to note here that equality of default object might not work since the lambda function might be different.
        return crn_comp_params {
            m_size_of_obj: core::mem::size_of::<crn_comp_params>() as u32,
            m_file_type: crn_file_type::cCRNFileTypeCRN,
            m_faces: 1,
            m_width: 0,
            m_height: 0,
            m_levels: 1,
            m_format: crn_format::cCRNFmtDXT1,
            m_flags: cCRNCompFlagPerceptual | cCRNCompFlagHierarchical | cCRNCompFlagUseBothBlockTypes,
      
            m_pImages: [[core::ptr::null_mut(); cCRNMaxLevels as usize]; cCRNMaxFaces as usize],
      
            m_target_bitrate: 0.0,
            m_quality_level: cCRNMaxQualityLevel,
            m_dxt1a_alpha_threshold: 128,
            m_dxt_quality: crn_dxt_quality::cCRNDXTQualityUber,
            m_dxt_compressor_type: crn_dxt_compressor_type::cCRNDXTCompressorCRN,
            m_alpha_component: 3,
      
            m_crn_adaptive_tile_color_psnr_derating: 2.0,
            m_crn_adaptive_tile_alpha_psnr_derating: 2.0,
            m_crn_color_endpoint_palette_size: 0,
            m_crn_color_selector_palette_size: 0,
            m_crn_alpha_endpoint_palette_size: 0,
            m_crn_alpha_selector_palette_size: 0,
      
            m_num_helper_threads: 0,
            m_userdata0: 0,
            m_userdata1: 0,
            m_pProgress_func: core::ptr::null_mut(),
            m_pProgress_func_data: core::ptr::null_mut(),
        }
    }

    // fn clear(mut self){
        // self = Self::default();
    // }

    #[inline]
    pub fn get_flag(self, flag: u32) -> bool{
        return (self.m_flags & flag) != 0;
    }

    #[inline]
    pub fn set_flag(&mut self, flag: u32, val: bool){
        self.m_flags &= !flag;
        if val {
            self.m_flags |= flag
        }
    }

   // Returns true if the input parameters are reasonable.
   #[inline]
    pub fn check(&mut self) -> bool {
        if   (self.m_file_type > crn_file_type::cCRNFileTypeDDS) ||
            ((self.m_quality_level < cCRNMinQualityLevel) || (self.m_quality_level > cCRNMaxQualityLevel)) ||
            (self.m_dxt1a_alpha_threshold > 255) ||
            ((self.m_faces != 1) && (self.m_faces != 6)) ||
            ((self.m_width < 1) || (self.m_width > cCRNMaxLevelResolution)) ||
            ((self.m_height < 1) || (self.m_height > cCRNMaxLevelResolution)) ||
            ((self.m_levels < 1) || (self.m_levels > cCRNMaxLevels)) ||
            ((self.m_format < crn_format::cCRNFmtDXT1) || (self.m_format >= crn_format::cCRNFmtTotal)) ||
            ((self.m_crn_color_endpoint_palette_size != 0) && ((self.m_crn_color_endpoint_palette_size < cCRNMinPaletteSize) || (self.m_crn_color_endpoint_palette_size > cCRNMaxPaletteSize))) ||
            ((self.m_crn_color_selector_palette_size != 0) && ((self.m_crn_color_selector_palette_size < cCRNMinPaletteSize) || (self.m_crn_color_selector_palette_size > cCRNMaxPaletteSize))) ||
            ((self.m_crn_alpha_endpoint_palette_size != 0) && ((self.m_crn_alpha_endpoint_palette_size < cCRNMinPaletteSize) || (self.m_crn_alpha_endpoint_palette_size > cCRNMaxPaletteSize))) ||
            ((self.m_crn_alpha_selector_palette_size != 0) && ((self.m_crn_alpha_selector_palette_size < cCRNMinPaletteSize) || (self.m_crn_alpha_selector_palette_size > cCRNMaxPaletteSize))) ||
            (self.m_alpha_component > 3) ||
            (self.m_num_helper_threads > cCRNMaxHelperThreads) ||
            (self.m_dxt_quality > crn_dxt_quality::cCRNDXTQualityUber) ||
            (self.m_dxt_compressor_type >= crn_dxt_compressor_type::cCRNTotalDXTCompressors)
        {
            return false;
        }
        return true;
    }

}

// Mipmap generator's mode.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_mip_mode{
    cCRNMipModeUseSourceOrGenerateMips,       // Use source texture's mipmaps if it has any, otherwise generate new mipmaps
    cCRNMipModeUseSourceMips,                 // Use source texture's mipmaps if it has any, otherwise the output has no mipmaps
    cCRNMipModeGenerateMips,                  // Always generate new mipmaps
    cCRNMipModeNoMips,                        // Output texture has no mipmaps

    cCRNMipModeTotal,

    cCRNModeForceDWORD = 0xFFFFFFFF
}

// Mipmap generator's filter kernel.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_mip_filter{
    cCRNMipFilterBox,
    cCRNMipFilterTent,
    cCRNMipFilterLanczos4,
    cCRNMipFilterMitchell,
    cCRNMipFilterKaiser,                      // Kaiser=default mipmap filter

    cCRNMipFilterTotal,

    cCRNMipFilterForceDWORD = 0xFFFFFFFF
}

#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_scale_mode{
    cCRNSMDisabled,
    cCRNSMAbsolute,
    cCRNSMRelative,
    cCRNSMLowerPow2,
    cCRNSMNearestPow2,
    cCRNSMNextPow2,

    cCRNSMTotal,

    cCRNSMForceDWORD = 0xFFFFFFFF
}

// Mipmap generator parameters.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub struct crn_mipmap_params{
    m_size_of_obj: u32,

    m_mode: crn_mip_mode,
    m_filter: crn_mip_filter,
 
    m_gamma_filtering: u32,
    m_gamma: f32,
 
    m_blurriness: f32,
 
    m_max_levels: u32,
    m_min_mip_size: u32,
 
    m_renormalize: u32,
    m_tiled: u32,
 
    m_scale_mode: crn_scale_mode,
    m_scale_x: f32,
    m_scale_y: f32,
 
    m_window_left: u32,
    m_window_top: u32,
    m_window_right: u32,
    m_window_bottom: u32,
 
    m_clamp_scale: u32,
    m_clamp_width: u32,
    m_clamp_height: u32,
}

impl crn_mipmap_params{
    pub fn default() -> Self{
        return crn_mipmap_params{
            m_size_of_obj: core::mem::size_of::<crn_mipmap_params>() as u32,
            m_mode: crn_mip_mode::cCRNMipModeUseSourceOrGenerateMips,
            m_filter: crn_mip_filter::cCRNMipFilterKaiser,
            m_gamma_filtering: 1,
            m_gamma: 2.2,
            // Default "blurriness" factor of .9 actually sharpens the output a little.
            m_blurriness: 0.9,
            m_renormalize: 0,
            m_tiled: 0,
            m_max_levels: cCRNMaxLevels,
            m_min_mip_size: 1,
    
            m_scale_mode: crn_scale_mode::cCRNSMDisabled,
            m_scale_x: 1.0,
            m_scale_y: 1.0,
    
            m_window_left: 0,
            m_window_top: 0,
            m_window_right: 0,
            m_window_bottom: 0,
    
            m_clamp_scale: 0,
            m_clamp_width: 0,
            m_clamp_height: 0,
        }
    }

    pub fn check() -> bool{
        return true;
    }
}


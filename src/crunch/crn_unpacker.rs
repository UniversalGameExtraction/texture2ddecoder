#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use super::crnlib::*;
use super::crn_utils::*;
use super::crn_consts::*;
use super::crn_static_huffman_data_model::*;
use super::crn_decomp::crn_header;
use super::crn_symbol_codec::symbol_codec;

macro_rules! CRND_HUFF_DECODE{
    ($codec: expr, $model: expr, $symbol: expr) => {
        $symbol = $codec.decode($model)
    };
}

macro_rules! WRITE_TO_INT_BUFFER {
    ($buf: expr, $index: expr, $val: expr) => {
        let t_index = ($index * 4) as usize;
        #[cfg(target_endian = "little")]
        let tiles = $val.to_le_bytes();
        #[cfg(target_endian = "big")]
        let tiles = $val.to_be_bytes();
        $buf[t_index] = tiles[0]; $buf[t_index + 1] = tiles[1]; $buf[t_index + 2] = tiles[2]; $buf[t_index + 3] = tiles[3]
    };
}

pub struct crn_unpacker<'slice>{
    pub m_magic: u32,
    pub m_pData: &'slice[u8],
    pub m_data_size: u32,
    pub m_tmp_header: crn_header,
    pub m_pHeader: crn_header,
    pub m_codec: symbol_codec<'slice>,
    pub m_chunk_encoding_dm: static_huffman_data_model,
    pub m_endpoint_delta_dm: [static_huffman_data_model; 2],
    pub m_selector_delta_dm: [static_huffman_data_model; 2],
    pub m_color_endpoints: alloc::vec::Vec<u32>,
    pub m_color_selectors: alloc::vec::Vec<u32>,
    pub m_alpha_endpoints: alloc::vec::Vec<u16>,
    pub m_alpha_selectors: alloc::vec::Vec<u16>,
}

impl<'slice> Default for crn_unpacker<'slice>{
    fn default() -> Self {
        return crn_unpacker {
            m_magic: cMagicValue,
            m_pData: <&[u8]>::default(),
            m_data_size: <u32>::default(),
            m_tmp_header: <crn_header>::default(),
            m_pHeader: <crn_header>::default(),
            m_codec: <symbol_codec<'slice>>::default(),
            m_chunk_encoding_dm: <static_huffman_data_model>::default(),
            m_endpoint_delta_dm: <[static_huffman_data_model; 2]>::default(),
            m_selector_delta_dm: <[static_huffman_data_model; 2]>::default(),
            m_color_endpoints: <alloc::vec::Vec<u32>>::default(),
            m_color_selectors: <alloc::vec::Vec<u32>>::default(),
            m_alpha_endpoints: <alloc::vec::Vec<u16>>::default(),
            m_alpha_selectors: <alloc::vec::Vec<u16>>::default(),
        }
    }
}

impl<'slice> crn_unpacker<'slice>{
    pub fn init(&mut self, pData: &'slice[u8], data_size: u32) -> bool{
        let res = self.m_pHeader.crnd_get_header(pData, data_size);
        if res == false {
            return res;
        }
        self.m_pData = pData;
        self.m_data_size = data_size;
        if self.init_tables() == false {
            return false;
        }
        if self.decode_palettes() == false {
            return false;
        }
        return true;
    }
    pub fn init_tables(&mut self) -> bool{
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_pData[self.m_pHeader.m_tables_ofs.cast_to_uint() as usize..], self.m_pHeader.m_tables_size.cast_to_uint());
        if res == false {
            return res;
        }
        res = self.m_codec.decode_receive_static_data_model(&mut self.m_chunk_encoding_dm);
        if res == false {
            return res;
        }
        if (self.m_pHeader.m_color_endpoints.m_num.cast_to_uint() == 0) && (self.m_pHeader.m_alpha_endpoints.m_num.cast_to_uint() == 0) {
            return false;
        }
        if self.m_pHeader.m_color_endpoints.m_num.cast_to_uint() != 0 {
            if  self.m_codec.decode_receive_static_data_model(&mut self.m_endpoint_delta_dm[0]) == false {return false;}
            if  self.m_codec.decode_receive_static_data_model(&mut self.m_selector_delta_dm[0]) == false {return false;}
        }
        if self.m_pHeader.m_alpha_endpoints.m_num.cast_to_uint() != 0 {
            if  self.m_codec.decode_receive_static_data_model(&mut self.m_endpoint_delta_dm[1]) == false {return false;}
            if  self.m_codec.decode_receive_static_data_model(&mut self.m_selector_delta_dm[1]) == false {return false;}
        }
        self.m_codec.stop_decoding();
        return true;
    }
    pub fn decode_palettes(&mut self) -> bool{
        if  self.m_pHeader.m_color_endpoints.m_num.cast_to_uint() != 0 {
           if  self.decode_color_endpoints() == false {return false;}
           if  self.decode_color_selectors() == false {return false;}
        }

        if  self.m_pHeader.m_alpha_endpoints.m_num.cast_to_uint() != 0 {
           if  self.decode_alpha_endpoints() == false {return false;}
           if  self.decode_alpha_selectors() == false {return false;}
        }

        return true;
    }
    pub fn decode_color_endpoints(&mut self) -> bool{
        let num_color_endpoints = self.m_pHeader.m_color_endpoints.m_num.cast_to_uint();
        self.m_color_endpoints.resize(num_color_endpoints as usize, 0);
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_pData[self.m_pHeader.m_color_endpoints.m_ofs.cast_to_uint() as usize..], self.m_pHeader.m_color_endpoints.m_size.cast_to_uint());
        if res == false {
            return res;
        }
        let mut dm = [static_huffman_data_model::default(), static_huffman_data_model::default()];
        for i in 0..2{
            res = self.m_codec.decode_receive_static_data_model(&mut dm[i]);
            if res == false {
                return res;
            }
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f): (u32, u32, u32, u32, u32, u32) = (0, 0, 0, 0, 0, 0);
        let mut pDst = &mut self.m_color_endpoints[0..];
        for _ in 0..num_color_endpoints{
            let (da, db, dc, dd, de, df): (u32, u32, u32, u32, u32, u32);
            CRND_HUFF_DECODE!(self.m_codec, &dm[0], da); a = (a + da) & 31;
            CRND_HUFF_DECODE!(self.m_codec, &dm[1], db); b = (b + db) & 63;
            CRND_HUFF_DECODE!(self.m_codec, &dm[0], dc); c = (c + dc) & 31;
            CRND_HUFF_DECODE!(self.m_codec, &dm[0], dd); d = (d + dd) & 31;
            CRND_HUFF_DECODE!(self.m_codec, &dm[1], de); e = (e + de) & 63;
            CRND_HUFF_DECODE!(self.m_codec, &dm[0], df); f = (f + df) & 31;
            if c_crnd_little_endian_platform {
                pDst[0] = c | (b << 5) | (a << 11) | (f << 16) | (e << 21) | (d << 27);
                pDst = &mut pDst[1..];
            }else{
                pDst[0] = f | (e << 5) | (d << 11) | (c << 16) | (b << 21) | (a << 27);
                pDst = &mut pDst[1..];
            }
        }
        self.m_codec.stop_decoding();
        return true;
    }
    pub fn decode_color_selectors(&mut self) -> bool{
        const cMaxSelectorValue: u32 = 3;
        const cMaxUniqueSelectorDeltas: u32 = cMaxSelectorValue * 2 + 1;
        let num_color_selectors = self.m_pHeader.m_color_selectors.m_num.cast_to_uint();
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_pData[(self.m_pHeader.m_color_selectors.m_ofs.cast_to_uint() as usize)..], self.m_pHeader.m_color_selectors.m_size.cast_to_uint());
        if res == false {
            return res;
        }
        let mut dm: static_huffman_data_model = static_huffman_data_model::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if res == false {
            return res;
        }
        let mut delta0 = [0; (cMaxUniqueSelectorDeltas * cMaxUniqueSelectorDeltas) as usize];
        let mut delta1 = [0; (cMaxUniqueSelectorDeltas * cMaxUniqueSelectorDeltas) as usize];
        let mut l: i32 = -(cMaxSelectorValue as i32);
        let mut m: i32 = -(cMaxSelectorValue as i32);
        for i in 0..(cMaxUniqueSelectorDeltas * cMaxUniqueSelectorDeltas) as usize{
            delta0[i] = l;
            delta1[i] = m;
            l += 1;
            if l > cMaxSelectorValue as i32 {
                l = -(cMaxSelectorValue as i32);
                m += 1;
            }
        }
        let mut cur = [0 as u32; 16];
        self.m_color_selectors.resize(num_color_selectors as usize, 0);
        let mut pDst = &mut self.m_color_selectors[0..];
        let pFrom_linear = &g_dxt1_from_linear[0..];
        for _ in 0..num_color_selectors as usize{
            for j in 0..8 as usize{
                let sym: u32;
                CRND_HUFF_DECODE!(self.m_codec, &dm, sym);
                cur[j*2+0] = ((delta0[sym as usize] + cur[j*2+0] as i32) & 3) as u32;
                cur[j*2+1] = ((delta1[sym as usize] + cur[j*2+1] as i32) & 3) as u32;
            }
            if c_crnd_little_endian_platform {
                pDst[0] =
                    ((pFrom_linear[cur[0 ] as usize] as u32)      ) | ((pFrom_linear[cur[1 ] as usize] as u32) <<  2) | ((pFrom_linear[cur[2 ] as usize] as u32) <<  4) | ((pFrom_linear[cur[3 ] as usize] as u32) <<  6) |
                    ((pFrom_linear[cur[4 ] as usize] as u32) <<  8) | ((pFrom_linear[cur[5 ] as usize] as u32) << 10) | ((pFrom_linear[cur[6 ] as usize] as u32) << 12) | ((pFrom_linear[cur[7 ] as usize] as u32) << 14) |
                    ((pFrom_linear[cur[8 ] as usize] as u32) << 16) | ((pFrom_linear[cur[9 ] as usize] as u32) << 18) | ((pFrom_linear[cur[10] as usize] as u32) << 20) | ((pFrom_linear[cur[11] as usize] as u32) << 22) |
                    ((pFrom_linear[cur[12] as usize] as u32) << 24) | ((pFrom_linear[cur[13] as usize] as u32) << 26) | ((pFrom_linear[cur[14] as usize] as u32) << 28) | ((pFrom_linear[cur[15] as usize] as u32) << 30);
                pDst = &mut pDst[1..];
            }else{
                pDst[0] =
                ((pFrom_linear[cur[8 ] as usize] as u32)      ) | ((pFrom_linear[cur[9 ] as usize] as u32) <<  2) | ((pFrom_linear[cur[10] as usize] as u32) <<  4) | ((pFrom_linear[cur[11] as usize] as u32) <<  6) |
                ((pFrom_linear[cur[12] as usize] as u32) <<  8) | ((pFrom_linear[cur[13] as usize] as u32) << 10) | ((pFrom_linear[cur[14] as usize] as u32) << 12) | ((pFrom_linear[cur[15] as usize] as u32) << 14) |
                ((pFrom_linear[cur[0 ] as usize] as u32) << 16) | ((pFrom_linear[cur[1 ] as usize] as u32) << 18) | ((pFrom_linear[cur[2 ] as usize] as u32) << 20) | ((pFrom_linear[cur[3 ] as usize] as u32) << 22) |
                ((pFrom_linear[cur[4 ] as usize] as u32) << 24) | ((pFrom_linear[cur[5 ] as usize] as u32) << 26) | ((pFrom_linear[cur[6 ] as usize] as u32) << 28) | ((pFrom_linear[cur[7 ] as usize] as u32) << 30);
                pDst = &mut pDst[1..];
            }
        }
        self.m_codec.stop_decoding();
        return true;
    }
    pub fn decode_alpha_endpoints(&mut self) -> bool{
        let num_alpha_endpoints = self.m_pHeader.m_alpha_endpoints.m_num.cast_to_uint();
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_pData[self.m_pHeader.m_alpha_endpoints.m_ofs.cast_to_uint() as usize..], self.m_pHeader.m_alpha_endpoints.m_size.cast_to_uint());
        if res == false {
            return res;
        }
        let mut dm = static_huffman_data_model::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if res == false {
            return res;
        }
        self.m_alpha_endpoints.resize(num_alpha_endpoints as usize, 0);
        let pDst: &mut [u16] = &mut self.m_alpha_endpoints[0..];
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        for i in 0..num_alpha_endpoints as usize{
            let sa = self.m_codec.decode(&dm);
            let sb = self.m_codec.decode(&dm);
            a = (sa + a) & 0xFF;
            b = (sb + b) & 0xFF;
            pDst[i] = (a | (b << 8)) as u16;
            // pDst = &mut pDst[1..];
        }
        self.m_codec.stop_decoding();
        return true;
    }
    pub fn decode_alpha_selectors(&mut self) -> bool{
        const cMaxSelectorValue: u32 = 7;
        const cMaxUniqueSelectorDeltas: u32 = cMaxSelectorValue * 2 + 1;
        let num_alpha_selectors = self.m_pHeader.m_alpha_selectors.m_num.cast_to_uint();
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_pData[self.m_pHeader.m_alpha_selectors.m_ofs.cast_to_uint() as usize..], self.m_pHeader.m_alpha_selectors.m_size.cast_to_uint());
        if res == false {
            return res;
        }
        let mut dm = static_huffman_data_model::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if res == false {
            return res;
        }
        let mut delta0 = [0; (cMaxUniqueSelectorDeltas * cMaxUniqueSelectorDeltas) as usize];
        let mut delta1 = [0; (cMaxUniqueSelectorDeltas * cMaxUniqueSelectorDeltas) as usize];
        let mut l: i32 = -(cMaxSelectorValue as i32);
        let mut m: i32 = -(cMaxSelectorValue as i32);
        for i in 0..(cMaxUniqueSelectorDeltas * cMaxUniqueSelectorDeltas) as usize{
            delta0[i] = l;
            delta1[i] = m;
            l += 1;
            if l > cMaxSelectorValue as i32 {
                l = -(cMaxSelectorValue as i32);
                m += 1;
            }
        }
        let mut cur = [0 as u32; 16];
        self.m_alpha_selectors.resize(num_alpha_selectors as usize * 3, 0);
        let mut pDst = &mut self.m_alpha_selectors[0..];
        let pFrom_linear = &g_dxt5_from_linear[0..];
        for _ in 0..num_alpha_selectors as usize{
            for j in 0..8 as usize{
                let sym: i32;
                sym = self.m_codec.decode(&dm) as i32;
                cur[j*2+0] = ((delta0[sym as usize] + cur[j*2+0] as i32) & 7) as u32;
                cur[j*2+1] = ((delta1[sym as usize] + cur[j*2+1] as i32) & 7) as u32;
            }
            pDst[0] = (((pFrom_linear[cur[0 ] as usize] as u32)     ) | ((pFrom_linear[cur[1 ] as usize] as u32) << 3) | ((pFrom_linear[cur[2 ] as usize] as u32) << 6) | 
                ((pFrom_linear[cur[3 ] as usize] as u32) << 9) | ((pFrom_linear[cur[4 ] as usize] as u32) << 12) | ((pFrom_linear[cur[5 ] as usize] as u32) << 15)) as u16;

            pDst[1] = (((pFrom_linear[cur[5 ] as usize] as u32) >> 1) | ((pFrom_linear[cur[6 ] as usize] as u32) << 2) | ((pFrom_linear[cur[7 ] as usize] as u32) << 5) |
                ((pFrom_linear[cur[8 ] as usize] as u32) << 8) | ((pFrom_linear[cur[9 ] as usize] as u32) << 11) | ((pFrom_linear[cur[10] as usize] as u32) << 14)) as u16;

            pDst[2] = (((pFrom_linear[cur[10] as usize] as u32) >> 2) | ((pFrom_linear[cur[11] as usize] as u32) << 1) | ((pFrom_linear[cur[12] as usize] as u32) << 4) |
                ((pFrom_linear[cur[13] as usize] as u32) << 7) | ((pFrom_linear[cur[14] as usize] as u32) << 10) | ((pFrom_linear[cur[15] as usize] as u32) << 13)) as u16;
            pDst = &mut pDst[3..];
        }
        self.m_codec.stop_decoding();
        return true;
    }
    pub fn crnd_unpack_level(&mut self, dst_size_in_bytes: u32, row_pitch_in_bytes: u32, level_index: u32) -> Result<alloc::vec::Vec<u8>, &'static str>{
        if (dst_size_in_bytes < 8) || (level_index >= cCRNMaxLevels) {
            return Err("Destination buffer size is too small.");
        }
        return self.unpack_level(dst_size_in_bytes, row_pitch_in_bytes, level_index);
    }
    pub fn unpack_level(&mut self, dst_size_in_bytes: u32, row_pitch_in_bytes: u32, level_index: u32) -> Result<alloc::vec::Vec<u8>, &'static str>{
        let cur_level_ofs = self.m_pHeader.m_level_ofs[level_index as usize].cast_to_uint();
        let mut next_level_ofs = self.m_data_size;
        if  (level_index + 1) < (self.m_pHeader.m_levels.cast_to_uint()) {
            next_level_ofs = self.m_pHeader.m_level_ofs[(level_index + 1) as usize].cast_to_uint();
        }
        if next_level_ofs <= cur_level_ofs {
            return Err("Level offset mismatch.");
        }
        return self.unpack_level_2(&self.m_pData[cur_level_ofs as usize..], next_level_ofs - cur_level_ofs, dst_size_in_bytes, row_pitch_in_bytes, level_index);
    }
    pub fn unpack_level_2(&mut self, pSrc: &'slice [u8], src_size_in_bytes: u32, dst_size_in_bytes: u32, mut row_pitch_in_bytes: u32, level_index: u32) -> Result<alloc::vec::Vec<u8>, &'static str>{
        let width: u32 = core::cmp::max(self.m_pHeader.m_width.cast_to_uint() >> level_index, 1);
        let height: u32 = core::cmp::max(self.m_pHeader.m_height.cast_to_uint() >> level_index, 1);
        let blocks_x: u32 = (width + 3) >> 2;
        let blocks_y: u32 = (height + 3) >> 2;
        let block_size: u32 = if  self.m_pHeader.m_format.cast_to_uint() == crn_format::cCRNFmtDXT1 as u32 || self.m_pHeader.m_format.cast_to_uint() == crn_format::cCRNFmtDXT5A as u32 {
            8
        }else{
            16
        };
        let minimal_row_pitch: u32 = block_size * blocks_x;
        if row_pitch_in_bytes == 0 {
            row_pitch_in_bytes = minimal_row_pitch;
        }else if row_pitch_in_bytes < minimal_row_pitch || (row_pitch_in_bytes & 3) != 0 {
            return Err("Crunch Row size is below the minimum allowed.");
        }
        let mut ret = alloc::vec![0 as u8; dst_size_in_bytes as usize];
        if dst_size_in_bytes < (row_pitch_in_bytes * blocks_y) {
            return Err("Destination buffer size is smaller than what expected to decompress.");
        }
        let chunks_x: u32 = (blocks_x + 1) >> 1;
        let chunks_y: u32 = (blocks_y + 1) >> 1;
        let mut res: bool = self.m_codec.start_decoding(&pSrc, src_size_in_bytes);
        if res == false{
            return Err("Failed to initialize the decoding process.");
        }
        let format = match self.m_pHeader.m_format.cast_to_uint() {
            0          => crn_format::cCRNFmtDXT1,
            1          => crn_format::cCRNFmtDXT3,
            2          => crn_format::cCRNFmtDXT5,
            3          => crn_format::cCRNFmtDXT5_CCxY,
            4          => crn_format::cCRNFmtDXT5_xGxR,
            5          => crn_format::cCRNFmtDXT5_xGBR,
            6          => crn_format::cCRNFmtDXT5_AGBR,
            7          => crn_format::cCRNFmtDXN_XY,
            8          => crn_format::cCRNFmtDXN_YX,
            9          => crn_format::cCRNFmtDXT5A,
            10         => crn_format::cCRNFmtETC1,
            11         => crn_format::cCRNFmtTotal,
            0xFFFFFFFF => crn_format::cCRNFmtForceDWORD,
            _          => crn_format::cCRNFmtInvalid
        };
        res = match format {
            crn_format::cCRNFmtDXT1 => self.unpack_dxt1(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y, chunks_x, chunks_y),

            crn_format::cCRNFmtDXT5 |
            crn_format::cCRNFmtDXT5_CCxY |
            crn_format::cCRNFmtDXT5_xGBR |
            crn_format::cCRNFmtDXT5_AGBR |
            crn_format::cCRNFmtDXT5_xGxR => self.unpack_dxt5(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y, chunks_x, chunks_y),
            
            crn_format::cCRNFmtDXT5A => self.unpack_dxt5a(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y, chunks_x, chunks_y),

            crn_format::cCRNFmtDXN_XY |
            crn_format::cCRNFmtDXN_YX => self.unpack_dxn(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y, chunks_x, chunks_y),

            _ => false
        };
        if res == false {
            return Err("Invalid or unsupported Crunch encoding format.");
        }
        self.m_codec.stop_decoding();
        return Ok(ret);
    }
    pub fn unpack_dxt1(&mut self, pDst: &mut [u8], row_pitch_in_bytes: u32, blocks_x: u32, blocks_y: u32, chunks_x: u32, chunks_y: u32) -> bool{
        let mut chunk_encoding_bits: u32 = 1;
        let num_color_endpoints: u32 = self.m_color_endpoints.len() as u32;
        let num_color_selectors: u32 = self.m_color_selectors.len() as u32;
        let mut prev_color_endpoint_index: u32 = 0;
        let mut prev_color_selector_index: u32 = 0;
        let num_faces: u32 = self.m_pHeader.m_faces.cast_to_uint();
        let row_pitch_in_dwords = row_pitch_in_bytes >> 2;
        let cBytesPerBlock: i32 = 8;
        for f in 0..num_faces as usize{
            let mut row_dst = f;
            for y in 0..chunks_y{
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item=i32>>;
                let block_delta: i32;
                if y&1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev()); 
                    block_delta = -cBytesPerBlock*2;
                    block_dst += (((chunks_x as i32) - 1) * cBytesPerBlock * 2) as usize;
                }else{
                    block_delta = cBytesPerBlock * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter{
                    let mut color_endpoints = [0 as u32; 4];
                    if chunk_encoding_bits == 1 {
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_chunk_encoding_dm, chunk_encoding_bits);
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;
                    let num_tiles = g_crnd_chunk_encoding_num_tiles[chunk_encoding_index as usize];
                    for i in 0..num_tiles as usize{
                        let delta: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_endpoint_delta_dm[0], delta);
                        prev_color_endpoint_index += delta;
                        limit(&mut prev_color_endpoint_index, num_color_endpoints);
                        color_endpoints[i] = self.m_color_endpoints[prev_color_endpoint_index as usize];
                    }
                    let pTile_indices = g_crnd_chunk_encoding_tiles[chunk_encoding_index as usize].m_tiles;
                    let skip_right_col = ((blocks_x & 1) == 1) && (x == (chunks_x as i32 - 1));
                    let mut pd_dst = block_dst >> 2;
                    if skip_bottom_row == false && skip_right_col == false {
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, color_endpoints[pTile_indices[0] as usize]);

                        let delta0: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[0], delta0);
                        prev_color_selector_index += delta0;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, self.m_color_selectors[prev_color_selector_index as usize]);
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 2, color_endpoints[pTile_indices[1] as usize]);
                        
                        let delta1: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[0], delta1);
                        prev_color_selector_index += delta1;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 3, self.m_color_selectors[prev_color_selector_index as usize]);
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0 + row_pitch_in_dwords as usize, color_endpoints[pTile_indices[2] as usize]);
                        
                        let delta2: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[0], delta2);
                        prev_color_selector_index += delta2;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1 + row_pitch_in_dwords as usize, self.m_color_selectors[prev_color_selector_index as usize]);
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 2 + row_pitch_in_dwords as usize, color_endpoints[pTile_indices[3] as usize]);

                        let delta3: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[0], delta3);
                        prev_color_selector_index += delta3;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(pDst, pd_dst + 3 + row_pitch_in_dwords as usize, self.m_color_selectors[prev_color_selector_index as usize]);
                    }else{
                        for by in 0..2{
                            pd_dst = block_dst + (row_pitch_in_bytes * by) as usize;
                            pd_dst >>= 2;
                            for bx in 0..2{
                                let delta: u32;
                                CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[0], delta);
                                prev_color_selector_index += delta;
                                limit(&mut prev_color_selector_index, num_color_selectors);
                                if (((bx != 0) && skip_right_col) || ((by != 0) && skip_bottom_row)) == false {
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, color_endpoints[pTile_indices[(bx + by * 2) as usize] as usize]);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, self.m_color_selectors[prev_color_selector_index as usize]);
                                }
                                pd_dst += 2;
                            }
                        }
                    }
                    block_dst = (block_dst as i32 + block_delta) as usize;
                }
                row_dst += (row_pitch_in_bytes * 2) as usize;
            }
        }
        return true;
    }
    pub fn unpack_dxt5(&mut self, pDst: &mut [u8], row_pitch_in_bytes: u32, blocks_x: u32, blocks_y: u32, chunks_x: u32, chunks_y: u32) -> bool{
        let mut chunk_encoding_bits: u32 = 1;
        let num_color_endpoints: u32 = self.m_color_endpoints.len() as u32;
        let num_color_selectors: u32 = self.m_color_selectors.len() as u32;
        let num_alpha_endpoints: u32 = self.m_alpha_endpoints.len() as u32;
        let num_alpha_selectors: u32 = self.m_pHeader.m_alpha_selectors.m_num.cast_to_uint();
        let mut prev_color_endpoint_index: u32 = 0;
        let mut prev_color_selector_index: u32 = 0;
        let mut prev_alpha_endpoint_index: u32 = 0;
        let mut prev_alpha_selector_index: u32 = 0;
        let num_faces = self.m_pHeader.m_faces.cast_to_uint();
        let cBytesPerBlock: i32 = 16;
        for f in 0..num_faces as usize{
            let mut row_dst = f;
            for y in 0..chunks_y{
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item=i32>>;
                let block_delta: i32;
                if y&1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev()); 
                    block_delta = -cBytesPerBlock*2;
                    block_dst += (((chunks_x as i32) - 1) * cBytesPerBlock * 2) as usize;
                }else{
                    block_delta = cBytesPerBlock * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter{
                    let mut color_endpoints = [0 as u32; 4];
                    let mut alpha_endpoints = [0 as u32; 4];
                    if  chunk_encoding_bits == 1 {
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_chunk_encoding_dm, chunk_encoding_bits);
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index: u32 = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;
                    let num_tiles = g_crnd_chunk_encoding_num_tiles[chunk_encoding_index as usize] as u32;
                    let pTile_indices = g_crnd_chunk_encoding_tiles[chunk_encoding_index as usize].m_tiles;
                    let skip_right_col = (blocks_x & 1) != 0 && (x == ((chunks_x as i32) - 1));
                    for i in 0..num_tiles{
                        let delta: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_endpoint_delta_dm[1], delta);
                        prev_alpha_endpoint_index += delta;
                        limit(&mut prev_alpha_endpoint_index, num_alpha_endpoints);
                        alpha_endpoints[i as usize] = self.m_alpha_endpoints[prev_alpha_endpoint_index as usize] as u32;
                    }
                    
                    for i in 0..num_tiles{
                        let delta: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_endpoint_delta_dm[0], delta);
                        prev_color_endpoint_index += delta;
                        limit(&mut prev_color_endpoint_index, num_color_endpoints);
                        color_endpoints[i as usize] = self.m_color_endpoints[prev_color_endpoint_index as usize] as u32;
                    }

                    let mut pd_dst = block_dst;
                    pd_dst >>= 2;
                    for by in 0..2{
                        for bx in 0..2{
                            let delta0: u32;
                            CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[1], delta0);
                            prev_alpha_selector_index += delta0;
                            limit(&mut prev_alpha_selector_index, num_alpha_selectors);
                            let delta1: u32;
                            CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[0], delta1);
                            prev_color_selector_index += delta1;
                            limit(&mut prev_color_selector_index, num_color_selectors);
                            if  (((bx != 0) && skip_right_col) || ((by != 0) && skip_bottom_row)) == false {
                                let tile_index: u32 = pTile_indices[bx + by * 2] as u32;
                                let pAlpha_selectors = &self.m_alpha_selectors[(prev_alpha_selector_index * 3) as usize..];
                                #[cfg(target_endian = "big")]{
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, ((alpha_endpoints[tile_index as usize] << 16) | pAlpha_selectors[0] as u32));
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, (((pAlpha_selectors[1] as u32) << 16) | (pAlpha_selectors[2] as u32)) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 2, (color_endpoints[tile_index as usize]));
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 3, (self.m_color_selectors[prev_color_selector_index as usize]));
                                }
                                #[cfg(target_endian = "little")]{
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, (alpha_endpoints[tile_index as usize] | ((pAlpha_selectors[0] as u32) << 16) as u32) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, ((pAlpha_selectors[1] as u32) | ((pAlpha_selectors[2] as u32) << 16)) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 2, (color_endpoints[tile_index as usize])); 
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 3, (self.m_color_selectors[prev_color_selector_index as usize]));
                                }
                            }
                            pd_dst += 4;
                        }
                        pd_dst <<= 2;
                        pd_dst = ((pd_dst as i32) + (-cBytesPerBlock * 2 + row_pitch_in_bytes as i32)) as usize;
                        pd_dst >>= 2;
                    }
                    block_dst = ((block_dst as i32) + block_delta) as usize;
                }
                row_dst += (row_pitch_in_bytes * 2) as usize;
            }
        }
        return true;
    }
    pub fn unpack_dxt5a(&mut self, pDst: &mut [u8], row_pitch_in_bytes: u32, blocks_x: u32, blocks_y: u32, chunks_x: u32, chunks_y: u32) -> bool{
        let mut chunk_encoding_bits: u32 = 1;
        let num_alpha_endpoints: u32 = self.m_alpha_endpoints.len() as u32;
        let num_alpha_selectors: u32 = self.m_pHeader.m_alpha_selectors.m_num.cast_to_uint();
        let mut prev_alpha0_endpoint_index: u32 = 0;
        let mut prev_alpha0_selector_index: u32 = 0;
        let num_faces = self.m_pHeader.m_faces.cast_to_uint();
        let cBytesPerBlock = 8;
        for f in 0..num_faces as usize{
            let mut row_dst = f;
            for y in 0..chunks_y{
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item=i32>>;
                let block_delta: i32;
                if y&1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev()); 
                    block_delta = -cBytesPerBlock*2;
                    block_dst += (((chunks_x as i32) - 1) * cBytesPerBlock * 2) as usize;
                }else{
                    block_delta = cBytesPerBlock * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter{
                    let mut alpha0_endpoints = [0 as u32; 4];
                    if  chunk_encoding_bits == 1 {
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_chunk_encoding_dm, chunk_encoding_bits);
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index: u32 = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;
                    let num_tiles = g_crnd_chunk_encoding_num_tiles[chunk_encoding_index as usize] as u32;
                    let pTile_indices = g_crnd_chunk_encoding_tiles[chunk_encoding_index as usize].m_tiles;
                    let skip_right_col = (blocks_x & 1) != 0 && (x == ((chunks_x as i32) - 1));
                    for i in 0..num_tiles{
                        let delta: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_endpoint_delta_dm[1], delta);
                        prev_alpha0_endpoint_index += delta;
                        limit(&mut prev_alpha0_endpoint_index, num_alpha_endpoints);
                        alpha0_endpoints[i as usize] = self.m_alpha_endpoints[prev_alpha0_endpoint_index as usize] as u32;
                    }
                    let mut pd_dst = block_dst;
                    pd_dst >>= 2;
                    for by in 0..2{
                        for bx in 0..2{
                            let delta: u32;
                            CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[1], delta);
                            prev_alpha0_selector_index += delta;
                            limit(&mut prev_alpha0_selector_index, num_alpha_selectors);
                            if  (((bx != 0) && skip_right_col) || ((by != 0) && skip_bottom_row)) == false {
                                let tile_index: u32 = pTile_indices[bx + by * 2] as u32;
                                let pAlpha0_selectors = &self.m_alpha_selectors[(prev_alpha0_selector_index * 3) as usize..];
                                #[cfg(target_endian = "big")]{
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, ((alpha0_endpoints[tile_index as usize] << 16) | pAlpha0_selectors[0] as u32));
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, (((pAlpha0_selectors[1] as u32) << 16) | (pAlpha0_selectors[2] as u32)) as u32);
                                }
                                #[cfg(target_endian = "little")]{
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, (alpha0_endpoints[tile_index as usize] | ((pAlpha0_selectors[0] as u32) << 16) as u32) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, ((pAlpha0_selectors[1] as u32) | ((pAlpha0_selectors[2] as u32) << 16)) as u32);
                                }
                            }
                            pd_dst += 2;
                        }
                        pd_dst <<= 2;
                        pd_dst = ((pd_dst as i32) + (-cBytesPerBlock * 2 + row_pitch_in_bytes as i32)) as usize;
                        pd_dst >>= 2;
                    }
                    block_dst = ((block_dst as i32) + block_delta) as usize;
                }
                row_dst += (row_pitch_in_bytes * 2) as usize;
            }
        }
        return true;
    }
    pub fn unpack_dxn(&mut self, pDst: &mut [u8], row_pitch_in_bytes: u32, blocks_x: u32, blocks_y: u32, chunks_x: u32, chunks_y: u32) -> bool{
        let mut chunk_encoding_bits: u32 = 1;
        let num_alpha_endpoints: u32 = self.m_alpha_endpoints.len() as u32;
        let num_alpha_selectors: u32 = self.m_pHeader.m_alpha_selectors.m_num.cast_to_uint();
        let mut prev_alpha0_endpoint_index: u32 = 0;
        let mut prev_alpha0_selector_index: u32 = 0;
        let mut prev_alpha1_endpoint_index: u32 = 0;
        let mut prev_alpha1_selector_index: u32 = 0;
        let num_faces: u32 = self.m_pHeader.m_faces.cast_to_uint();
        let cBytesPerBlock: i32 = 16;
        for f in 0..num_faces as usize{
            let mut row_dst = f;
            for y in 0..chunks_y{
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item=i32>>;
                let block_delta: i32;
                if y&1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev()); 
                    block_delta = -cBytesPerBlock*2;
                    block_dst += (((chunks_x as i32) - 1) * cBytesPerBlock * 2) as usize;
                }else{
                    block_delta = cBytesPerBlock * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter{
                    let mut alpha0_endpoints = [0 as u32; 4];
                    let mut alpha1_endpoints = [0 as u32; 4];
                    if  chunk_encoding_bits == 1 {
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_chunk_encoding_dm, chunk_encoding_bits);
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index: u32 = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;
                    let num_tiles = g_crnd_chunk_encoding_num_tiles[chunk_encoding_index as usize] as u32;
                    let pTile_indices = g_crnd_chunk_encoding_tiles[chunk_encoding_index as usize].m_tiles;
                    let skip_right_col = (blocks_x & 1) != 0 && (x == ((chunks_x as i32) - 1));
                    for i in 0..num_tiles{
                        let delta: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_endpoint_delta_dm[1], delta);
                        prev_alpha0_endpoint_index += delta;
                        limit(&mut prev_alpha0_endpoint_index, num_alpha_endpoints);
                        alpha0_endpoints[i as usize] = self.m_alpha_endpoints[prev_alpha0_endpoint_index as usize] as u32;
                    }
                    for i in 0..num_tiles{
                        let delta: u32;
                        CRND_HUFF_DECODE!(self.m_codec, &self.m_endpoint_delta_dm[1], delta);
                        prev_alpha1_endpoint_index += delta;
                        limit(&mut prev_alpha1_endpoint_index, num_alpha_endpoints);
                        alpha1_endpoints[i as usize] = self.m_alpha_endpoints[prev_alpha1_endpoint_index as usize] as u32;
                    }
                    let mut pd_dst = block_dst;
                    pd_dst >>= 2;
                    for by in 0..2{
                        for bx in 0..2{
                            let delta0: u32;
                            CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[1], delta0);
                            prev_alpha0_selector_index += delta0;
                            limit(&mut prev_alpha0_selector_index, num_alpha_selectors);
                            let delta1: u32;
                            CRND_HUFF_DECODE!(self.m_codec, &self.m_selector_delta_dm[1], delta1);
                            prev_alpha1_selector_index += delta1;
                            limit(&mut prev_alpha1_selector_index, num_alpha_selectors);
                            if (((bx != 0) && skip_right_col) || ((by != 0) && skip_bottom_row)) == false {
                                let tile_index: u32 = pTile_indices[bx + by * 2] as u32;
                                let pAlpha0_selectors = &self.m_alpha_selectors[(prev_alpha0_selector_index * 3) as usize..];
                                let pAlpha1_selectors = &self.m_alpha_selectors[(prev_alpha1_selector_index * 3) as usize..];
                                #[cfg(target_endian = "big")]{
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, ((alpha0_endpoints[tile_index as usize] << 16) | pAlpha0_selectors[0] as u32));
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, (((pAlpha0_selectors[1] as u32) << 16) | (pAlpha0_selectors[2] as u32)) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 2, ((alpha1_endpoints[tile_index as usize] << 16) | pAlpha1_selectors[0] as u32));
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 3, (((pAlpha1_selectors[1] as u32) << 16) | (pAlpha1_selectors[2] as u32)) as u32);
                                }
                                #[cfg(target_endian = "little")]{
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 0, (alpha0_endpoints[tile_index as usize] | ((pAlpha0_selectors[0] as u32) << 16) as u32) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 1, ((pAlpha0_selectors[1] as u32) | ((pAlpha0_selectors[2] as u32) << 16)) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 2, (alpha1_endpoints[tile_index as usize] | ((pAlpha1_selectors[0] as u32) << 16) as u32) as u32);
                                    WRITE_TO_INT_BUFFER!(pDst, pd_dst + 3, ((pAlpha1_selectors[1] as u32) | ((pAlpha1_selectors[2] as u32) << 16)) as u32);
                                }
                            }
                            pd_dst += 4;
                        }
                        pd_dst <<= 2;
                        pd_dst = ((pd_dst as i32) + (-cBytesPerBlock * 2 + row_pitch_in_bytes as i32)) as usize;
                        pd_dst >>= 2;
                    }
                    block_dst = ((block_dst as i32) + block_delta) as usize;
                }
                row_dst += (row_pitch_in_bytes * 2) as usize;
            }
        }
        return true;
    }
}
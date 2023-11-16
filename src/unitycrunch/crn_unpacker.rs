use crate::crnlib::CrnFormat;
use crate::crunch::crn_consts::*;
use crate::crunch::crn_static_huffman_data_model::*;
use crate::crunch::crn_symbol_codec::symbol_codec;
use crate::crunch::crn_decomp::CrnHeader;
use crate::macros::*;
extern crate alloc;

#[derive(Default, Clone)]
#[repr(C)]
pub struct BlockBufferElement {
    endpoint_reference: u16,
    color_endpoint_index: u16,
    alpha0_endpoint_index: u16,
    alpha1_endpoint_index: u16
}

pub struct CrnUnpacker<'slice>{
    pub m_magic: u32,
    pub m_p_data: &'slice[u8],
    pub m_data_size: u32,
    pub m_p_header: CrnHeader,
    pub m_codec: symbol_codec<'slice>,
    pub m_reference_encoding_dm: StaticHuffmanDataModel,
    pub m_endpoint_delta_dm: [StaticHuffmanDataModel; 2],
    pub m_selector_delta_dm: [StaticHuffmanDataModel; 2],
    pub m_color_endpoints: alloc::vec::Vec<u32>,
    pub m_color_selectors: alloc::vec::Vec<u32>,
    pub m_alpha_endpoints: alloc::vec::Vec<u16>,
    pub m_alpha_selectors: alloc::vec::Vec<u16>,
    pub m_block_buffer: alloc::vec::Vec<BlockBufferElement>
}

impl<'slice> Default for CrnUnpacker<'slice>{
    fn default() -> Self {
        CrnUnpacker {
            m_magic: C_MAGIC_VALUE,
            m_p_data: <&[u8]>::default(),
            m_data_size: <u32>::default(),
            m_p_header: <CrnHeader>::default(),
            m_codec: <symbol_codec<'slice>>::default(),
            m_reference_encoding_dm: <StaticHuffmanDataModel>::default(),
            m_endpoint_delta_dm: <[StaticHuffmanDataModel; 2]>::default(),
            m_selector_delta_dm: <[StaticHuffmanDataModel; 2]>::default(),
            m_color_endpoints: <alloc::vec::Vec<u32>>::default(),
            m_color_selectors: <alloc::vec::Vec<u32>>::default(),
            m_alpha_endpoints: <alloc::vec::Vec<u16>>::default(),
            m_alpha_selectors: <alloc::vec::Vec<u16>>::default(),
            m_block_buffer: <alloc::vec::Vec<BlockBufferElement>>::default()
        }
    }
}

impl<'slice> CrnUnpacker<'slice>{
    pub fn init(&mut self, p_data: &'slice[u8], data_size: u32) -> bool{
        let res = self.m_p_header.crnd_get_header(p_data, data_size);
        if !res {
            return res;
        }
        self.m_p_data = p_data;
        self.m_data_size = data_size;
        if !self.init_tables() {
            return false;
        }
        if !self.decode_palettes() {
            return false;
        }
        true
    }
    pub fn init_tables(&mut self) -> bool{
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_p_data[self.m_p_header.m_tables_ofs.cast_to_uint() as usize..], self.m_p_header.m_tables_size.cast_to_uint());
        if !res {
            return res;
        }
        res = self.m_codec.decode_receive_static_data_model(&mut self.m_reference_encoding_dm);
        if !res {
            return res;
        }
        if (self.m_p_header.m_color_endpoints.m_num.cast_to_uint() == 0) && (self.m_p_header.m_alpha_endpoints.m_num.cast_to_uint() == 0) {
            return false;
        }
        if self.m_p_header.m_color_endpoints.m_num.cast_to_uint() != 0 {
            if  !self.m_codec.decode_receive_static_data_model(&mut self.m_endpoint_delta_dm[0]) {return false;}
            if  !self.m_codec.decode_receive_static_data_model(&mut self.m_selector_delta_dm[0]) {return false;}
        }
        if self.m_p_header.m_alpha_endpoints.m_num.cast_to_uint() != 0 {
            if  !self.m_codec.decode_receive_static_data_model(&mut self.m_endpoint_delta_dm[1]) {return false;}
            if  !self.m_codec.decode_receive_static_data_model(&mut self.m_selector_delta_dm[1]) {return false;}
        }
        self.m_codec.stop_decoding();
        true
    }
    pub fn decode_palettes(&mut self) -> bool{
        if self.m_p_header.m_color_endpoints.m_num.cast_to_uint() != 0 {
           if !self.decode_color_endpoints() {return false;}
           if !self.decode_color_selectors() {return false;}
        }

        if self.m_p_header.m_alpha_endpoints.m_num.cast_to_uint() != 0 {
            if !self.decode_alpha_endpoints() {return false;}

            if self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2as as u32 {
                if !self.decode_alpha_selectors_etcs() {return false;}
            }else if self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2a as u32{
                if !self.decode_alpha_selectors_etc() {return false;}
            }else if !self.decode_alpha_selectors() {return false;}
        }

        true
    }
    pub fn decode_color_endpoints(&mut self) -> bool{
        let num_color_endpoints = self.m_p_header.m_color_endpoints.m_num.cast_to_uint();
        let has_etc_color_blocks: bool =    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1 as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2 as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2a as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1s as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2as as u32;

        let has_subblocks: bool =   self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1 as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2 as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2a as u32;
        self.m_color_endpoints.resize(num_color_endpoints as usize, 0);
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_p_data[self.m_p_header.m_color_endpoints.m_ofs.cast_to_uint() as usize..], self.m_p_header.m_color_endpoints.m_size.cast_to_uint());
        if !res {
            return res;
        }
        let mut dm = [StaticHuffmanDataModel::default(), StaticHuffmanDataModel::default()];
        let range: usize = if has_etc_color_blocks {1} else {2};
        for i in 0..range{
            res = self.m_codec.decode_receive_static_data_model(&mut (dm[i]));
            if !res {
                return res;
            }
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f): (u32, u32, u32, u32, u32, u32) = (0, 0, 0, 0, 0, 0);
        let mut p_dst = &mut self.m_color_endpoints[0..];
        for _ in 0..num_color_endpoints{
            if has_etc_color_blocks {
                for b in [0, 8, 16, 24]{
                    a += match self.m_codec.decode(&dm[0]) {
						Ok(s) => s,
						Err(_) => return false
					} << b;
                }
                a &= 0x1F1F1F1F;
                if has_subblocks {
                    p_dst[0] = a;
                }else{
                    p_dst[0] = (a & 0x07000000) << 5 | (a & 0x07000000) << 2 | 0x02000000 | (a & 0x001F1F1F) << 3;
                }
                p_dst = &mut p_dst[1..];
            }else{
                let (da, db, dc, dd, de, df): (u32, u32, u32, u32, u32, u32);
                CRND_HUFF_DECODE!(self.m_codec, &dm[0], da); a = (a + da) & 31;
                CRND_HUFF_DECODE!(self.m_codec, &dm[1], db); b = (b + db) & 63;
                CRND_HUFF_DECODE!(self.m_codec, &dm[0], dc); c = (c + dc) & 31;
                CRND_HUFF_DECODE!(self.m_codec, &dm[0], dd); d = (d + dd) & 31;
                CRND_HUFF_DECODE!(self.m_codec, &dm[1], de); e = (e + de) & 63;
                CRND_HUFF_DECODE!(self.m_codec, &dm[0], df); f = (f + df) & 31;
                p_dst[0] = c | (b << 5) | (a << 11) | (f << 16) | (e << 21) | (d << 27);
                p_dst = &mut p_dst[1..];
            }
        }
        self.m_codec.stop_decoding();
        true
    }
    pub fn decode_color_selectors(&mut self) -> bool{
        let has_etc_color_blocks: bool =    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1 as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2 as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2a as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1s as u32 ||
                                            self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2as as u32;

        let has_subblocks: bool =   self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1 as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2 as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2a as u32;
        let mut res: bool;
        // Return value here is ignored in the original code.
        res = self.m_codec.start_decoding(&self.m_p_data[(self.m_p_header.m_color_selectors.m_ofs.cast_to_uint() as usize)..], self.m_p_header.m_color_selectors.m_size.cast_to_uint());
        if !res {
            return res;
        }
        let mut dm: StaticHuffmanDataModel = StaticHuffmanDataModel::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        if has_subblocks{
            self.m_color_selectors.resize((self.m_p_header.m_color_selectors.m_num.cast_to_uint() as usize) << 1, 0);
        }else{
            self.m_color_selectors.resize(self.m_p_header.m_color_selectors.m_num.cast_to_uint() as usize, 0);
        }
        let mut s: u32 = 0;
        for i in 0..self.m_p_header.m_color_selectors.m_num.cast_to_uint() as usize{
            for j in [0, 4, 8, 12, 16, 20, 24, 28]{
                s ^= match self.m_codec.decode(&dm) {
                    Ok(s) => s,
                    Err(_) => return false
                } << j;
            }
            if has_etc_color_blocks{
                let selector = (!s & 0xAAAAAAAA) | (!(s ^ s >> 1) & 0x55555555);
                let mut t: i32 = 8;
                for h in 0..4{
                    for w in 0..4{
                        if has_subblocks{
                            let s0 = selector >> (w << 3 | h << 1);
                            self.m_color_selectors[i << 1] |= ((s0 >> 1 & 1) | (s0 & 1) << 16) << (t & 15);
                        }
                        let s1 = selector >> (h << 3 | w << 1);
                        if has_subblocks {
                            self.m_color_selectors[i << 1 | 1] |= ((s1 >> 1 & 1) | (s1 & 1) << 16) << (t & 15);
                        }else{
                            self.m_color_selectors[i] |= ((s1 >> 1 & 1) | (s1 & 1) << 16) << (t & 15);
                        }
                        t += 4;
                    }
                    t -= 15;
                }
            }else{
                self.m_color_selectors[i] = ((s ^ s << 1) & 0xAAAAAAAA) | (s >> 1 & 0x55555555);
            }
        }
        self.m_codec.stop_decoding();
        true
    }
    pub fn decode_alpha_endpoints(&mut self) -> bool{
        let num_alpha_endpoints = self.m_p_header.m_alpha_endpoints.m_num.cast_to_uint();
        let mut res: bool;
        res = self.m_codec.start_decoding(&self.m_p_data[self.m_p_header.m_alpha_endpoints.m_ofs.cast_to_uint() as usize..], self.m_p_header.m_alpha_endpoints.m_size.cast_to_uint());
        if !res {
            return res;
        }
        let mut dm = StaticHuffmanDataModel::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        self.m_alpha_endpoints.resize(num_alpha_endpoints as usize, 0);
        let p_dst: &mut [u16] = &mut self.m_alpha_endpoints[0..];
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        for i in 0..num_alpha_endpoints as usize{
            let sa = match self.m_codec.decode(&dm) {
                    Ok(s) => s,
                    Err(_) => return false
                };
            let sb = match self.m_codec.decode(&dm) {
                    Ok(s) => s,
                    Err(_) => return false
                };
            a = (sa + a) & 0xFF;
            b = (sb + b) & 0xFF;
            p_dst[i] = (a | (b << 8)) as u16;
        }
        self.m_codec.stop_decoding();
        true
    }
    pub fn decode_alpha_selectors(&mut self) -> bool{
        let mut res = self.m_codec.start_decoding(&self.m_p_data[self.m_p_header.m_alpha_selectors.m_ofs.cast_to_uint() as usize..], self.m_p_header.m_alpha_selectors.m_size.cast_to_uint());
        if !res {
            return res;
        }
        let mut dm = StaticHuffmanDataModel::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        self.m_alpha_selectors.resize((self.m_p_header.m_alpha_selectors.m_num.cast_to_uint() as usize) * 3, 0);
        let mut dxt5_from_linear = [0_u8; 64];
        for i in 0..64{
            dxt5_from_linear[i] = G_DXT5_FROM_LINEAR[i & 7] | G_DXT5_FROM_LINEAR[i >> 3] << 3;
        }
        let mut s0_linear: u32 = 0;
        let mut s1_linear: u32 = 0;
        let mut i: usize = 0;
        while i < self.m_alpha_selectors.len(){
            let mut s0: u32 = 0;
            let mut s1: u32 = 0;
            for j in [0, 6, 12, 18] {
                s0_linear ^= match self.m_codec.decode(&dm) {
                    Ok(s) => s,
                    Err(_) => return false
                } << j;
                s0 |= (dxt5_from_linear[(s0_linear >> j & 0x3F) as usize] as u32) << j;
            }
            for j in [0, 6, 12, 18] {
                s1_linear ^= match self.m_codec.decode(&dm) {
                    Ok(s) => s,
                    Err(_) => return false
                } << j;
                s1 |= (dxt5_from_linear[(s1_linear >> j & 0x3F) as usize] as u32) << j;
            }
            self.m_alpha_selectors[i] = s0 as u16;
            i += 1;
            self.m_alpha_selectors[i] = (s0 >> 16 | s1 << 8) as u16;
            i += 1;
            self.m_alpha_selectors[i] = (s1 >> 8) as u16;
            i += 1;
        }
        self.m_codec.stop_decoding();
        true
    }
    pub fn decode_alpha_selectors_etc(&mut self) -> bool{
        let mut res = self.m_codec.start_decoding(&self.m_p_data[self.m_p_header.m_alpha_selectors.m_ofs.cast_to_uint() as usize..], self.m_p_header.m_alpha_selectors.m_size.cast_to_uint());
        if !res {
            return res;
        }
        let mut dm = StaticHuffmanDataModel::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        // + 1 here because in the C++ code it goes out of bounds by 1 byte at max.
        self.m_alpha_selectors.resize((self.m_p_header.m_alpha_selectors.m_num.cast_to_uint() as usize) * 6 + 1, 0);
        let mut s_linear = [0_u8; 8];
        let mut data_pos: usize = 0;
        let mut i: usize = 0;
        // - 1 because we added one before.
        while i < self.m_alpha_selectors.len() - 1{
            let mut s_group: u32 = 0;
            for p in 0..16{
                if p & 1 == 1 {
                    s_group >>= 3;
                }else{
                    s_linear[p >> 1] ^= match self.m_codec.decode(&dm) {
						Ok(s) => s,
						Err(_) => return false
					} as u8;
                    s_group = s_linear[p >> 1] as u32;
                }
                let mut s: u8 = (s_group & 7) as u8;
                if s <= 3{
                    s = 3 - s;
                }
                let mut d = (3 * (p + 1)) as i32;
                let mut byte_offset = d >> 3;
                let mut bit_offset = d & 7;
                WRITE_OR_U8_INTO_U16_BUFFER!(self.m_alpha_selectors, data_pos + byte_offset as usize, (((s as u16) << (8 - bit_offset)) & 0xFF));
                if bit_offset < 3 {
                    WRITE_OR_U8_INTO_U16_BUFFER!(self.m_alpha_selectors, data_pos + (byte_offset as usize) - 1, (s >> bit_offset) as u16);
                }
                d += 9 * ((p as i32 & 3) - (p as i32 >> 2));
                byte_offset = d >> 3;
                bit_offset = d & 7;
                WRITE_OR_U8_INTO_U16_BUFFER!(self.m_alpha_selectors, data_pos + (byte_offset as usize) + 6, ((s as u16) << (8 - bit_offset)) & 0xFF);
                if bit_offset < 3 {
                    WRITE_OR_U8_INTO_U16_BUFFER!(self.m_alpha_selectors, data_pos + (byte_offset as usize) + 5, (s >> bit_offset) as u16);
                }
            }
            i += 6;
            data_pos += 12;
        }
        true
    }
    pub fn decode_alpha_selectors_etcs(&mut self) -> bool {
        let mut res = self.m_codec.start_decoding(&self.m_p_data[self.m_p_header.m_alpha_selectors.m_ofs.cast_to_uint() as usize..], self.m_p_header.m_alpha_selectors.m_size.cast_to_uint());
        if !res {
            return res;
        }
        let mut dm = StaticHuffmanDataModel::default();
        res = self.m_codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        self.m_alpha_selectors.resize(((self.m_p_header.m_alpha_selectors.m_num.cast_to_uint() as usize) * 3) + 1, 0);
        let mut s_linear = [0_u8; 8];
        let mut i: usize = 0;
        while i < ((self.m_alpha_selectors.len() - 1) << 1){
            let mut s_group: u32 = 0;
            for p in 0..16{
                if p & 1 == 1 {
                    s_group >>= 3;
                }else{
                    s_linear[p >> 1] ^= match self.m_codec.decode(&dm) {
						Ok(s) => s,
						Err(_) => return false
					} as u8;
                    s_group = s_linear[p >> 1] as u32;
                }
                let mut s: u8 = (s_group & 7) as u8;
                if s <= 3{
                    s = 3 - s;
                }
                let d = (3 * ((p as i32) + 1) + 9 * (((p as i32) & 3) - ((p as i32) >> 2))) as i16;
                let byte_offset = d >> 3;
                let bit_offset = d & 7;
                WRITE_OR_U8_INTO_U16_BUFFER!(self.m_alpha_selectors, i + byte_offset as usize, (((s as u16) << (8 - bit_offset)) & 0xFF));
                if bit_offset < 3 {
                    WRITE_OR_U8_INTO_U16_BUFFER!(self.m_alpha_selectors, i + (byte_offset as usize) - 1, (s >> bit_offset) as u16);
                }
            }
            i += 6;
        }
        true
    }
    pub fn crnd_unpack_level(&mut self, dst_size_in_bytes: u32, row_pitch_in_bytes: u32, level_index: u32) -> Result<alloc::vec::Vec<u8>, &'static str>{
        if (dst_size_in_bytes < 8) || (level_index >= C_CRNMAX_LEVELS) {
            return Err("Destination buffer size is too small.");
        }
        self.unpack_level(dst_size_in_bytes, row_pitch_in_bytes, level_index)
    }
    pub fn unpack_level(&mut self, dst_size_in_bytes: u32, row_pitch_in_bytes: u32, level_index: u32) -> Result<alloc::vec::Vec<u8>, &'static str>{
        let cur_level_ofs = self.m_p_header.m_level_ofs[level_index as usize].cast_to_uint();
        let mut next_level_ofs = self.m_data_size;
        if  (level_index + 1) < (self.m_p_header.m_levels.cast_to_uint()) {
            next_level_ofs = self.m_p_header.m_level_ofs[(level_index + 1) as usize].cast_to_uint();
        }
        if next_level_ofs <= cur_level_ofs {
            return Err("Level offset mismatch.");
        }
        self.unpack_level_2(&self.m_p_data[cur_level_ofs as usize..], next_level_ofs - cur_level_ofs, dst_size_in_bytes, row_pitch_in_bytes, level_index)
    }
    pub fn unpack_level_2(&mut self, p_src: &'slice [u8], src_size_in_bytes: u32, dst_size_in_bytes: u32, mut row_pitch_in_bytes: u32, level_index: u32) -> Result<alloc::vec::Vec<u8>, &'static str>{
        let width: u32 = core::cmp::max(self.m_p_header.m_width.cast_to_uint() >> level_index, 1);
        let height: u32 = core::cmp::max(self.m_p_header.m_height.cast_to_uint() >> level_index, 1);
        let blocks_x: u32 = (width + 3) >> 2;
        let blocks_y: u32 = (height + 3) >> 2;
        let block_size: u32 = if    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtDxt1  as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtDxt5a as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1  as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc2  as u32 ||
                                    self.m_p_header.m_format.cast_to_uint() == CrnFormat::CCrnfmtEtc1s as u32 {
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
        let mut ret = alloc::vec![0_u8; dst_size_in_bytes as usize];
        if dst_size_in_bytes < (row_pitch_in_bytes * blocks_y) {
            return Err("Destination buffer size is smaller than what expected to decompress.");
        }
        let res: bool = self.m_codec.start_decoding(&p_src, src_size_in_bytes);
        if !res{
            return Err("Failed to initialize the decoding process.");
        }
        let format = match self.m_p_header.m_format.cast_to_uint() {
            0          => CrnFormat::CCrnfmtDxt1,
            1          => CrnFormat::CCrnfmtDxt3,
            2          => CrnFormat::CCrnfmtDxt5,
            3          => CrnFormat::CCrnfmtDxt5CcxY,
            4          => CrnFormat::CCrnfmtDxt5XGxR,
            5          => CrnFormat::CCrnfmtDxt5XGbr,
            6          => CrnFormat::CCrnfmtDxt5Agbr,
            7          => CrnFormat::CCrnfmtDxnXy,
            8          => CrnFormat::CCrnfmtDxnYx,
            9          => CrnFormat::CCrnfmtDxt5a,
            10         => CrnFormat::CCrnfmtEtc1,
            11         => CrnFormat::CCrnfmtEtc2,
            12         => CrnFormat::CCrnfmtEtc2a,
            13         => CrnFormat::CCrnfmtEtc1s,            
            14         => CrnFormat::CCrnfmtEtc2as,
            15         => CrnFormat::CCrnfmtTotal,
            0xFFFFFFFF => CrnFormat::CCrnfmtForceDword,
            _          => CrnFormat::CCrnfmtInvalid
        };
        let unpack_res = match format {
            CrnFormat::CCrnfmtDxt1 |
            CrnFormat::CCrnfmtEtc1s => self.unpack_dxt1(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y),

            CrnFormat::CCrnfmtDxt5 |
            CrnFormat::CCrnfmtDxt5CcxY |
            CrnFormat::CCrnfmtDxt5XGbr |
            CrnFormat::CCrnfmtDxt5Agbr |
            CrnFormat::CCrnfmtDxt5XGxR |
            CrnFormat::CCrnfmtEtc2as => self.unpack_dxt5(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y),
            
            CrnFormat::CCrnfmtDxt5a => self.unpack_dxt5a(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y),
            
            CrnFormat::CCrnfmtDxnXy |
            CrnFormat::CCrnfmtDxnYx => self.unpack_dxn(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y),
            
            CrnFormat::CCrnfmtEtc1 |
            CrnFormat::CCrnfmtEtc2 => self.unpack_etc1(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y),

            CrnFormat::CCrnfmtEtc2a => self.unpack_etc2a(&mut ret, row_pitch_in_bytes, blocks_x, blocks_y),

            _ => return Err("Invalid format for unpacking.")
        };
        match unpack_res{
            Ok(unpack_res) => unpack_res,
            Err(unpack_res) => return Err(unpack_res)
        };
        self.m_codec.stop_decoding();
        Ok(ret)
    }
    pub fn unpack_dxt1(&mut self, p_dst: &mut [u8], output_pitch_in_bytes: u32, output_width: u32, output_height: u32) -> Result<bool, &'static str>{
        let num_color_endpoints: u32 = self.m_color_endpoints.len() as u32;
        let width: u32 = (output_width + 1) & !1;
        let height: u32 = (output_height + 1) & !1;
        let delta_pitch_in_dwords: i32 = ((output_pitch_in_bytes >> 2) - (width << 1)) as i32;
        if self.m_block_buffer.len() < width as usize{
            self.m_block_buffer.resize(width as usize, BlockBufferElement::default());
        }
        let mut color_endpoint_index: usize = 0;
        let mut reference_group: u8 = 0;
        for f in 0..self.m_p_header.m_faces.cast_to_uint() as usize{
            let mut data_pos: usize = f;
            for y in 0..height{
                let mut visible = y < output_height;
                for x in 0..width as usize{
                    visible = visible && x < output_width as usize;
                    if (y & 1) == 0 && (x & 1) == 0 {
                        reference_group = match self.m_codec.decode(&self.m_reference_encoding_dm) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT1 Texture.")
                        } as u8;
                    }
                    let buffer = &mut self.m_block_buffer[x];
                    let endpoint_reference: u8;
                    if y&1 == 1{
                        endpoint_reference = buffer.endpoint_reference as u8;
                    }else{
                        endpoint_reference = reference_group & 3;
                        reference_group >>= 2;
                        buffer.endpoint_reference = (reference_group & 3) as u16;
                        reference_group >>= 2;
                    }
                    if endpoint_reference == 0 {
                        color_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[0]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT1 Texture.")
                        } as usize;
                        if color_endpoint_index >= num_color_endpoints as usize{
                            color_endpoint_index -= num_color_endpoints as usize;
                        }
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                    }else if endpoint_reference == 1 {
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                    }else{
                        color_endpoint_index = buffer.color_endpoint_index as usize;
                    }
                    let color_selector_index = match self.m_codec.decode(&self.m_selector_delta_dm[0]) {
                        Ok(s) => s,
                        Err(_) => return Err("Failed to decode DXT1 Texture.")
                    } as usize;
                    if visible {
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 0, self.m_color_endpoints[color_endpoint_index]);
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 1, self.m_color_selectors[color_selector_index]);
                    }
                    data_pos += 2;
                }
                data_pos += delta_pitch_in_dwords as usize;
            }
        }
        Ok(true)
    }
    pub fn unpack_dxt5(&mut self, p_dst: &mut [u8], output_pitch_in_bytes: u32, output_width: u32, output_height: u32) -> Result<bool, &'static str>{
        let num_color_endpoints: u32 = self.m_color_endpoints.len() as u32;
        let num_alpha_endpoints: u32 = self.m_alpha_endpoints.len() as u32;
        let width: u32 = (output_width + 1) & !1;
        let height: u32 = (output_height + 1) & !1;
        let delta_pitch_in_dwords: i32 = ((output_pitch_in_bytes >> 2) - (width << 2)) as i32;
        if self.m_block_buffer.len() < width as usize{
            self.m_block_buffer.resize(width as usize, BlockBufferElement::default());
        }
        let mut color_endpoint_index: usize = 0;
        let mut alpha0_endpoint_index: usize = 0;
        let mut reference_group: u8 = 0;
        for f in 0..self.m_p_header.m_faces.cast_to_uint() as usize{
            let mut data_pos: usize = f;
            for y in 0..height{
                let mut visible = y < output_height;
                for x in 0..width as usize{
                    visible = visible && x < output_width as usize;
                    if (y & 1) == 0 && (x & 1) == 0 {
                        reference_group = match self.m_codec.decode(&self.m_reference_encoding_dm) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5 Texture.")
                        } as u8;
                    }
                    let buffer = &mut self.m_block_buffer[x];
                    let endpoint_reference: u8;
                    if y&1 == 1{
                        endpoint_reference = buffer.endpoint_reference as u8;
                    }else{
                        endpoint_reference = reference_group & 3;
                        reference_group >>= 2;
                        buffer.endpoint_reference = (reference_group & 3) as u16;
                        reference_group >>= 2;
                    }
                    if endpoint_reference == 0 {
                        color_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[0]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5 Texture.")
                        } as usize;
                        if color_endpoint_index >= num_color_endpoints as usize{
                            color_endpoint_index -= num_color_endpoints as usize;
                        }
                        buffer.color_endpoint_index = color_endpoint_index as u16;

                        alpha0_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5 Texture.")
                        } as usize;
                        if alpha0_endpoint_index >= num_alpha_endpoints as usize{
                            alpha0_endpoint_index -= num_alpha_endpoints as usize;
                        }
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    }else if endpoint_reference == 1 {
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    }else{
                        color_endpoint_index = buffer.color_endpoint_index as usize;
                        alpha0_endpoint_index = buffer.alpha0_endpoint_index as usize;
                    }
                    let color_selector_index = match self.m_codec.decode(&self.m_selector_delta_dm[0]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5 Texture.")
                        } as usize;
                    let alpha0_selector_index = match self.m_codec.decode(&self.m_selector_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5 Texture.")
                        } as usize;
                    if visible {
                        let p_alpha0_selectors = &self.m_alpha_selectors[alpha0_selector_index * 3..];
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 0, self.m_alpha_endpoints[alpha0_endpoint_index] as u32 | ((p_alpha0_selectors[0] as u32) << 16));
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 1, (p_alpha0_selectors[1] as u32) | ((p_alpha0_selectors[2] as u32) << 16));
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 2, self.m_color_endpoints[color_endpoint_index]);
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 3, self.m_color_selectors[color_selector_index]);
                    }
                    data_pos += 4;
                }
                data_pos += delta_pitch_in_dwords as usize;
            }
        }
        Ok(true)
    }
    pub fn unpack_dxt5a(&mut self, p_dst: &mut [u8], output_pitch_in_bytes: u32, output_width: u32, output_height: u32) -> Result<bool, &'static str>{
        let num_alpha_endpoints: u32 = self.m_alpha_endpoints.len() as u32;
        let width: u32 = (output_width + 1) & !1;
        let height: u32 = (output_height + 1) & !1;
        let delta_pitch_in_dwords: i32 = ((output_pitch_in_bytes >> 2) - (width << 1)) as i32;
        if self.m_block_buffer.len() < width as usize{
            self.m_block_buffer.resize(width as usize, BlockBufferElement::default());
        }
        let mut alpha0_endpoint_index: usize = 0;
        let mut reference_group: u8 = 0;
        for f in 0..self.m_p_header.m_faces.cast_to_uint() as usize{
            let mut data_pos: usize = f;
            for y in 0..height{
                let mut visible = y < output_height;
                for x in 0..width as usize{
                    visible = visible && x < output_width as usize;
                    if (y & 1) == 0 && (x & 1) == 0 {
                        reference_group = match self.m_codec.decode(&self.m_reference_encoding_dm) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5A Texture.")
                        } as u8;
                    }
                    let buffer = &mut self.m_block_buffer[x];
                    let endpoint_reference: u8;
                    if y&1 == 1{
                        endpoint_reference = buffer.endpoint_reference as u8;
                    }else{
                        endpoint_reference = reference_group & 3;
                        reference_group >>= 2;
                        buffer.endpoint_reference = (reference_group & 3) as u16;
                        reference_group >>= 2;
                    }
                    if endpoint_reference == 0 {
                        alpha0_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5A Texture.")
                        } as usize;
                        if alpha0_endpoint_index >= num_alpha_endpoints as usize{
                            alpha0_endpoint_index -= num_alpha_endpoints as usize;
                        }
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    }else if endpoint_reference == 1 {
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    }else{
                        alpha0_endpoint_index = buffer.alpha0_endpoint_index as usize;
                    }
                    let alpha0_selector_index = match self.m_codec.decode(&self.m_selector_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXT5A Texture.")
                        } as usize;
                    if visible {
                        let p_alpha0_selectors = &self.m_alpha_selectors[alpha0_selector_index * 3..];
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 0, self.m_alpha_endpoints[alpha0_endpoint_index] as u32 | ((p_alpha0_selectors[0] as u32) << 16));
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 1, (p_alpha0_selectors[1] as u32) | ((p_alpha0_selectors[2] as u32) << 16));
                    }
                    data_pos += 2;
                }
                data_pos += delta_pitch_in_dwords as usize;
            }
        }
        Ok(true)
    }
    pub fn unpack_dxn(&mut self, p_dst: &mut [u8], output_pitch_in_bytes: u32, output_width: u32, output_height: u32) -> Result<bool, &'static str>{
        let num_alpha_endpoints: u32 = self.m_alpha_endpoints.len() as u32;
        let width: u32 = (output_width + 1) & !1;
        let height: u32 = (output_height + 1) & !1;
        let delta_pitch_in_dwords: i32 = ((output_pitch_in_bytes >> 2) - (width << 2)) as i32;
        if self.m_block_buffer.len() < width as usize{
            self.m_block_buffer.resize(width as usize, BlockBufferElement::default());
        }
        let mut alpha0_endpoint_index: usize = 0;
        let mut alpha1_endpoint_index: usize = 0;
        let mut reference_group: u8 = 0;
        for f in 0..self.m_p_header.m_faces.cast_to_uint() as usize{
            let mut data_pos: usize = f;
            for y in 0..height{
                let mut visible = y < output_height;
                for x in 0..width as usize{
                    visible = visible && x < output_width as usize;
                    if (y & 1) == 0 && (x & 1) == 0 {
                        reference_group = match self.m_codec.decode(&self.m_reference_encoding_dm) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXN Texture.")
                        } as u8;
                    }
                    let buffer = &mut self.m_block_buffer[x];
                    let endpoint_reference: u8;
                    if y&1 == 1{
                        endpoint_reference = buffer.endpoint_reference as u8;
                    }else{
                        endpoint_reference = reference_group & 3;
                        reference_group >>= 2;
                        buffer.endpoint_reference = (reference_group & 3) as u16;
                        reference_group >>= 2;
                    }
                    if endpoint_reference == 0 {
                        alpha0_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXN Texture.")
                        } as usize;
                        if alpha0_endpoint_index >= num_alpha_endpoints as usize{
                            alpha0_endpoint_index -= num_alpha_endpoints as usize;
                        }
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;

                        alpha1_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXN Texture.")
                        } as usize;
                        if alpha1_endpoint_index >= num_alpha_endpoints as usize{
                            alpha1_endpoint_index -= num_alpha_endpoints as usize;
                        }
                        buffer.alpha1_endpoint_index = alpha1_endpoint_index as u16;
                    }else if endpoint_reference == 1 {
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                        buffer.alpha1_endpoint_index = alpha1_endpoint_index as u16;
                    }else{
                        alpha0_endpoint_index = buffer.alpha0_endpoint_index as usize;
                        alpha1_endpoint_index = buffer.alpha1_endpoint_index as usize;
                    }
                    let alpha0_selector_index = match self.m_codec.decode(&self.m_selector_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXN Texture.")
                        } as usize;
                    let alpha1_selector_index = match self.m_codec.decode(&self.m_selector_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode DXN Texture.")
                        } as usize;
                    if visible {
                        let p_alpha0_selectors = &self.m_alpha_selectors[alpha0_selector_index * 3..];
                        let p_alpha1_selectors = &self.m_alpha_selectors[alpha1_selector_index * 3..];
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 0, self.m_alpha_endpoints[alpha0_endpoint_index] as u32 | ((p_alpha0_selectors[0] as u32) << 16));
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 1, (p_alpha0_selectors[1] as u32) | ((p_alpha0_selectors[2] as u32) << 16));
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 2, self.m_alpha_endpoints[alpha1_endpoint_index] as u32 | ((p_alpha1_selectors[0] as u32) << 16));
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 3, (p_alpha1_selectors[1] as u32) | ((p_alpha1_selectors[2] as u32) << 16));
                    }
                    data_pos += 4;
                }
                data_pos += delta_pitch_in_dwords as usize;
            }
        }
        Ok(true)
    }
    pub fn unpack_etc1(&mut self, p_dst: &mut [u8], output_pitch_in_bytes: u32, output_width: u32, output_height: u32) -> Result<bool, &'static str>{
        let num_color_endpoints: u32 = self.m_color_endpoints.len() as u32;
        let width: u32 = (output_width + 1) & !1;
        let height: u32 = (output_height + 1) & !1;
        let delta_pitch_in_dwords: i32 = ((output_pitch_in_bytes >> 2) - (width << 1)) as i32;
        if self.m_block_buffer.len() < (width << 1) as usize{
            self.m_block_buffer.resize((width << 1) as usize, BlockBufferElement::default());
        }
        let mut color_endpoint_index: usize = 0;
        let mut diagonal_color_endpoint_index: usize = 0;
        let mut reference_group: u8;
        for f in 0..self.m_p_header.m_faces.cast_to_uint() as usize{
            let mut data_pos: usize = f;
            for y in 0..height{
                let mut visible = y < output_height;
                for x in 0..width as usize{
                    visible = visible && x < output_width as usize;
                    let buffer = &mut self.m_block_buffer[x << 1];
                    let mut endpoint_reference: u8;
                    let mut block_endpoint = [0_u8; 4];
                    if y&1 == 1{
                        endpoint_reference = buffer.endpoint_reference as u8;
                    }else{
                        reference_group = match self.m_codec.decode(&self.m_reference_encoding_dm) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode ETC1 Texture.")
                        } as u8;
                        endpoint_reference = (reference_group & 3) | (reference_group >> 2 & 12);
                        buffer.endpoint_reference = ((reference_group >> 2 & 3) | (reference_group >> 4 & 12)) as u16;
                    }
                    if (endpoint_reference & 3) == 0 {
                        color_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[0]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode ETC1 Texture.")
                        } as usize;
                        if color_endpoint_index >= num_color_endpoints as usize {
                            color_endpoint_index -= num_color_endpoints as usize;
                        }
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                    }else if (endpoint_reference & 3) == 1{
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                    }else if (endpoint_reference & 3) == 3{
                        color_endpoint_index = diagonal_color_endpoint_index;
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                    }else{
                        color_endpoint_index = buffer.color_endpoint_index as usize;
                    }
                    endpoint_reference >>= 2;
                    let e0 = self.m_color_endpoints[color_endpoint_index].to_le_bytes();
                    let selector_index: usize = match self.m_codec.decode(&self.m_selector_delta_dm[0]) {
                        Ok(s) => s,
                        Err(_) => return Err("Failed to decode ETC1 Texture.")
                    } as usize;
                    if endpoint_reference != 0 {
                        color_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[0]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode ETC1 Texture.")
                        } as usize;
                        if color_endpoint_index >= num_color_endpoints as usize {
                            color_endpoint_index -= num_color_endpoints as usize;
                        }
                    }
                    diagonal_color_endpoint_index = self.m_block_buffer[x << 1 | 1].color_endpoint_index as usize;
                    self.m_block_buffer[x << 1 | 1].color_endpoint_index = color_endpoint_index as u16;
                    let e1 = self.m_color_endpoints[color_endpoint_index].to_le_bytes();
                    if visible {
                        let flip: u8 = endpoint_reference >> 1 ^ 1;
                        let mut diff: u8 = 1;
                        for c in 0..3_usize{
                            if diff == 0 {
                                break;
                            }
                            if e0[c] + 3 >= e1[c] && e1[c] + 4 >= e0[c]{
                                diff = diff
                            }else{
                                diff = 0;
                            }
                        }
                        for c in 0..3_usize{
                            if diff != 0 {
                                block_endpoint[c] = e0[c] << 3 | (((e1[c] as i32) - (e0[c] as i32)) & 7) as u8;
                            }else{
                                block_endpoint[c] = (e0[c] << 3 & 0xF0) | e1[c] >> 1;
                            }
                        }
                        block_endpoint[3] = e0[3] << 5 | e1[3] << 2 | diff << 1 | flip;
                        p_dst[data_pos * 4 + 0] = block_endpoint[0];
                        p_dst[data_pos * 4 + 1] = block_endpoint[1];
                        p_dst[data_pos * 4 + 2] = block_endpoint[2];
                        p_dst[data_pos * 4 + 3] = block_endpoint[3];
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 1, self.m_color_selectors[selector_index << 1 | (flip as usize)]);
                    }
                    data_pos += 2;
                }
                data_pos += delta_pitch_in_dwords as usize;
            }
        }
        Ok(true)
    }
    pub fn unpack_etc2a(&mut self, p_dst: &mut [u8], output_pitch_in_bytes: u32, output_width: u32, output_height: u32) -> Result<bool, &'static str>{
        let num_color_endpoints: u32 = self.m_color_endpoints.len() as u32;
        let num_alpha_endpoints: u32 = self.m_alpha_endpoints.len() as u32;
        let width: u32 = (output_width + 1) & !1;
        let height: u32 = (output_height + 1) & !1;
        let delta_pitch_in_dwords: i32 = ((output_pitch_in_bytes >> 2) - (width << 2)) as i32;
        if self.m_block_buffer.len() < (width << 1) as usize{
            self.m_block_buffer.resize((width << 1) as usize, BlockBufferElement::default());
        }
        let mut color_endpoint_index: usize = 0;
        let mut alpha0_endpoint_index: usize = 0;
        let mut diagonal_color_endpoint_index: usize = 0;
        let mut diagonal_alpha0_endpoint_index: usize = 0;
        let mut reference_group: u8;
        for f in 0..self.m_p_header.m_faces.cast_to_uint() as usize{
            let mut data_pos: usize = f;
            for y in 0..height{
                let mut visible = y < output_height;
                for x in 0..width as usize{
                    visible = visible && x < output_width as usize;
                    let buffer = &mut self.m_block_buffer[x << 1];
                    let mut endpoint_reference: u8;
                    let mut block_endpoint = [0_u8; 4];
                    if y&1 == 1{
                        endpoint_reference = buffer.endpoint_reference as u8;
                    }else{
                        reference_group = match self.m_codec.decode(&self.m_reference_encoding_dm) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode ETC2 Texture.")
                        } as u8;
                        endpoint_reference = (reference_group & 3) | (reference_group >> 2 & 12);
                        buffer.endpoint_reference = ((reference_group >> 2 & 3) | (reference_group >> 4 & 12)) as u16;
                    }
                    if (endpoint_reference & 3) == 0 {
                        color_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[0]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode ETC2 Texture.")
                        } as usize;
                        if color_endpoint_index >= num_color_endpoints as usize {
                            color_endpoint_index -= num_color_endpoints as usize;
                        }
                        alpha0_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[1]) {
                            Ok(s) => s,
                            Err(_) => return Err("Failed to decode ETC2 Texture.")
                        } as usize;
                        if alpha0_endpoint_index >= num_alpha_endpoints as usize{
                            alpha0_endpoint_index -= num_alpha_endpoints as usize;
                        }
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    }else if (endpoint_reference & 3) == 1{
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    }else if (endpoint_reference & 3) == 3{
                        color_endpoint_index = diagonal_color_endpoint_index;
                        buffer.color_endpoint_index = color_endpoint_index as u16;
                        alpha0_endpoint_index = diagonal_alpha0_endpoint_index;
                        buffer.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    }else{
                        color_endpoint_index = buffer.color_endpoint_index as usize;
                        alpha0_endpoint_index = buffer.alpha0_endpoint_index as usize;
                    }
                    endpoint_reference >>= 2;
                    let e0 = self.m_color_endpoints[color_endpoint_index].to_le_bytes();
                    let color_selector_index: usize = match self.m_codec.decode(&self.m_selector_delta_dm[0]) {
						Ok(s) => s,
						Err(_) => return Err("Failed to decode ETC2 Texture.")
					} as usize;
                    let alpha0_selector_index: usize = match self.m_codec.decode(&self.m_selector_delta_dm[1]) {
						Ok(s) => s,
						Err(_) => return Err("Failed to decode ETC2 Texture.")
					} as usize;
                    if endpoint_reference != 0 {
                        color_endpoint_index += match self.m_codec.decode(&self.m_endpoint_delta_dm[0]) {
						Ok(s) => s,
						Err(_) => return Err("Failed to decode ETC2 Texture.")
					} as usize;
                        if color_endpoint_index >= num_color_endpoints as usize {
                            color_endpoint_index -= num_color_endpoints as usize;
                        }
                    }
                    let e1 = self.m_color_endpoints[color_endpoint_index].to_le_bytes();
                    diagonal_color_endpoint_index = self.m_block_buffer[x << 1 | 1].color_endpoint_index as usize;
                    diagonal_alpha0_endpoint_index = self.m_block_buffer[x << 1 | 1].alpha0_endpoint_index as usize;
                    self.m_block_buffer[x << 1 | 1].color_endpoint_index = color_endpoint_index as u16;
                    self.m_block_buffer[x << 1 | 1].alpha0_endpoint_index = alpha0_endpoint_index as u16;
                    if visible {
                        let flip = endpoint_reference >> 1 ^ 1;
                        let mut diff = 1_u8;
                        for c in 0..3_usize{
                            if diff == 0 {
                                break;
                            }
                            if e0[c] + 3 >= e1[c] && e1[c] + 4 >= e0[c]{
                                diff = diff
                            }else{
                                diff = 0;
                            }
                        }
                        for c in 0..3_usize{
                            if diff != 0 {
                                block_endpoint[c] = e0[c] << 3 | (((e1[c] as i32) - (e0[c] as i32)) & 7) as u8;
                            }else{
                                block_endpoint[c] = (e0[c] << 3 & 0xF0) | e1[c] >> 1;
                            }
                        }
                        block_endpoint[3] = e0[3] << 5 | e1[3] << 2 | diff << 1 | flip;
                        let p_alpha0_selectors: &[u16];
                        if flip != 0 {
                            p_alpha0_selectors = &self.m_alpha_selectors[alpha0_selector_index * 6 + 3..];
                        }else{
                            p_alpha0_selectors = &self.m_alpha_selectors[alpha0_selector_index * 6 + 0..];
                        }
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 0, self.m_alpha_endpoints[alpha0_endpoint_index] as u32 | ((p_alpha0_selectors[0] as u32) << 16));
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 1, (p_alpha0_selectors[1] as u32) | ((p_alpha0_selectors[2] as u32) << 16));
                        p_dst[(data_pos + 2) * 4 + 0] = block_endpoint[0];
                        p_dst[(data_pos + 2) * 4 + 1] = block_endpoint[1];
                        p_dst[(data_pos + 2) * 4 + 2] = block_endpoint[2];
                        p_dst[(data_pos + 2) * 4 + 3] = block_endpoint[3];
                        WRITE_TO_INT_BUFFER!(p_dst, data_pos + 3, self.m_color_selectors[color_selector_index << 1 | (flip as usize)]);
                    }
                    data_pos += 4;
                }
                data_pos += delta_pitch_in_dwords as usize;
            }
        }
        Ok(true)
    }
}

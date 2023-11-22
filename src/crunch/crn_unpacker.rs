use super::crn_consts::*;
use super::crn_decomp::CrnHeader;
use super::crn_static_huffman_data_model::*;
use super::crn_symbol_codec::symbol_codec;
use super::crn_utils::*;
use super::CrnFormat;
use crate::macros::*;
extern crate alloc;

pub struct CrnUnpacker<'slice> {
    pub magic: u32,
    pub p_data: &'slice [u8],
    pub data_size: u32,
    pub tmp_header: CrnHeader,
    pub p_header: CrnHeader,
    pub codec: symbol_codec<'slice>,
    pub chunk_encoding_dm: StaticHuffmanDataModel,
    pub endpoint_delta_dm: [StaticHuffmanDataModel; 2],
    pub selector_delta_dm: [StaticHuffmanDataModel; 2],
    pub color_endpoints: alloc::vec::Vec<u32>,
    pub color_selectors: alloc::vec::Vec<u32>,
    pub alpha_endpoints: alloc::vec::Vec<u16>,
    pub alpha_selectors: alloc::vec::Vec<u16>,
}

impl<'slice> Default for CrnUnpacker<'slice> {
    fn default() -> Self {
        CrnUnpacker {
            magic: MAGIC_VALUE,
            p_data: <&[u8]>::default(),
            data_size: <u32>::default(),
            tmp_header: <CrnHeader>::default(),
            p_header: <CrnHeader>::default(),
            codec: <symbol_codec<'slice>>::default(),
            chunk_encoding_dm: <StaticHuffmanDataModel>::default(),
            endpoint_delta_dm: <[StaticHuffmanDataModel; 2]>::default(),
            selector_delta_dm: <[StaticHuffmanDataModel; 2]>::default(),
            color_endpoints: <alloc::vec::Vec<u32>>::default(),
            color_selectors: <alloc::vec::Vec<u32>>::default(),
            alpha_endpoints: <alloc::vec::Vec<u16>>::default(),
            alpha_selectors: <alloc::vec::Vec<u16>>::default(),
        }
    }
}

impl<'slice> CrnUnpacker<'slice> {
    pub fn init(&mut self, p_data: &'slice [u8], data_size: u32) -> bool {
        let res = self.p_header.crnd_get_header(p_data, data_size);
        if !res {
            return res;
        }
        self.p_data = p_data;
        self.data_size = data_size;
        if !self.init_tables() {
            return false;
        }
        if !self.decode_palettes() {
            return false;
        }
        true
    }
    pub fn init_tables(&mut self) -> bool {
        let mut res: bool;
        res = self.codec.start_decoding(
            &self.p_data[self.p_header.tables_ofs.cast_to_uint() as usize..],
            self.p_header.tables_size.cast_to_uint(),
        );
        if !res {
            return res;
        }
        res = self
            .codec
            .decode_receive_static_data_model(&mut self.chunk_encoding_dm);
        if !res {
            return res;
        }
        if (self.p_header.color_endpoints.num.cast_to_uint() == 0)
            && (self.p_header.alpha_endpoints.num.cast_to_uint() == 0)
        {
            return false;
        }
        if self.p_header.color_endpoints.num.cast_to_uint() != 0 {
            if !self
                .codec
                .decode_receive_static_data_model(&mut self.endpoint_delta_dm[0])
            {
                return false;
            }
            if !self
                .codec
                .decode_receive_static_data_model(&mut self.selector_delta_dm[0])
            {
                return false;
            }
        }
        if self.p_header.alpha_endpoints.num.cast_to_uint() != 0 {
            if !self
                .codec
                .decode_receive_static_data_model(&mut self.endpoint_delta_dm[1])
            {
                return false;
            }
            if !self
                .codec
                .decode_receive_static_data_model(&mut self.selector_delta_dm[1])
            {
                return false;
            }
        }
        self.codec.stop_decoding();
        true
    }
    pub fn decode_palettes(&mut self) -> bool {
        if self.p_header.color_endpoints.num.cast_to_uint() != 0 {
            if !self.decode_color_endpoints() {
                return false;
            }
            if !self.decode_color_selectors() {
                return false;
            }
        }

        if self.p_header.alpha_endpoints.num.cast_to_uint() != 0 {
            if !self.decode_alpha_endpoints() {
                return false;
            }
            if !self.decode_alpha_selectors() {
                return false;
            }
        }

        true
    }
    pub fn decode_color_endpoints(&mut self) -> bool {
        let num_color_endpoints = self.p_header.color_endpoints.num.cast_to_uint();
        self.color_endpoints.resize(num_color_endpoints as usize, 0);
        let mut res: bool;

        res = self.codec.start_decoding(
            &self.p_data[self.p_header.color_endpoints.ofs.cast_to_uint() as usize..],
            self.p_header.color_endpoints.size.cast_to_uint(),
        );
        if !res {
            return res;
        }

        let mut dm = [
            StaticHuffmanDataModel::default(),
            StaticHuffmanDataModel::default(),
        ];
        for dm_item in dm.iter_mut().take(2) {
            res = self.codec.decode_receive_static_data_model(dm_item);
            if !res {
                return false;
            }
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f): (u32, u32, u32, u32, u32, u32) =
            (0, 0, 0, 0, 0, 0);
        let mut p_dst = &mut self.color_endpoints[0..];
        for _ in 0..num_color_endpoints {
            let (da, db, dc, dd, de, df): (u32, u32, u32, u32, u32, u32);
            CRND_HUFF_DECODE!(self.codec, &dm[0], da);
            a = (a + da) & 31;
            CRND_HUFF_DECODE!(self.codec, &dm[1], db);
            b = (b + db) & 63;
            CRND_HUFF_DECODE!(self.codec, &dm[0], dc);
            c = (c + dc) & 31;
            CRND_HUFF_DECODE!(self.codec, &dm[0], dd);
            d = (d + dd) & 31;
            CRND_HUFF_DECODE!(self.codec, &dm[1], de);
            e = (e + de) & 63;
            CRND_HUFF_DECODE!(self.codec, &dm[0], df);
            f = (f + df) & 31;
            if CRND_LITTLE_ENDIAN_PLATFORM {
                p_dst[0] = c | (b << 5) | (a << 11) | (f << 16) | (e << 21) | (d << 27);
                p_dst = &mut p_dst[1..];
            } else {
                p_dst[0] = f | (e << 5) | (d << 11) | (c << 16) | (b << 21) | (a << 27);
                p_dst = &mut p_dst[1..];
            }
        }
        self.codec.stop_decoding();
        true
    }
    pub fn decode_color_selectors(&mut self) -> bool {
        const MAX_SELECTOR_VALUE: u32 = 3;
        const MAX_UNIQUE_SELECTOR_DELTAS: usize = (MAX_SELECTOR_VALUE as usize) * 2 + 1;
        let num_color_selectors = self.p_header.color_selectors.num.cast_to_uint();
        let mut res: bool;
        res = self.codec.start_decoding(
            &self.p_data[(self.p_header.color_selectors.ofs.cast_to_uint() as usize)..],
            self.p_header.color_selectors.size.cast_to_uint(),
        );
        if !res {
            return res;
        }
        let mut dm: StaticHuffmanDataModel = StaticHuffmanDataModel::default();
        res = self.codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        let mut delta0 = [0; (MAX_UNIQUE_SELECTOR_DELTAS * MAX_UNIQUE_SELECTOR_DELTAS)];
        let mut delta1 = [0; (MAX_UNIQUE_SELECTOR_DELTAS * MAX_UNIQUE_SELECTOR_DELTAS)];
        let mut l: i32 = -(MAX_SELECTOR_VALUE as i32);
        let mut m: i32 = -(MAX_SELECTOR_VALUE as i32);
        for i in 0..(MAX_UNIQUE_SELECTOR_DELTAS * MAX_UNIQUE_SELECTOR_DELTAS) {
            delta0[i] = l;
            delta1[i] = m;
            l += 1;
            if l > MAX_SELECTOR_VALUE as i32 {
                l = -(MAX_SELECTOR_VALUE as i32);
                m += 1;
            }
        }
        let mut cur = [0_u32; 16];
        self.color_selectors.resize(num_color_selectors as usize, 0);
        let mut p_dst = &mut self.color_selectors[0..];
        let p_from_linear = &DXT1_FROM_LINEAR[0..];
        for _ in 0..num_color_selectors as usize {
            for j in 0..8_usize {
                let sym: u32 = match self.codec.decode(&dm) {
                    Ok(s) => s,
                    Err(_) => return false,
                };
                cur[j * 2] = ((delta0[sym as usize] + cur[j * 2] as i32) & 3) as u32;
                cur[j * 2 + 1] = ((delta1[sym as usize] + cur[j * 2 + 1] as i32) & 3) as u32;
            }
            if CRND_LITTLE_ENDIAN_PLATFORM {
                p_dst[0] = (p_from_linear[cur[0] as usize] as u32)
                    | ((p_from_linear[cur[1] as usize] as u32) << 2)
                    | ((p_from_linear[cur[2] as usize] as u32) << 4)
                    | ((p_from_linear[cur[3] as usize] as u32) << 6)
                    | ((p_from_linear[cur[4] as usize] as u32) << 8)
                    | ((p_from_linear[cur[5] as usize] as u32) << 10)
                    | ((p_from_linear[cur[6] as usize] as u32) << 12)
                    | ((p_from_linear[cur[7] as usize] as u32) << 14)
                    | ((p_from_linear[cur[8] as usize] as u32) << 16)
                    | ((p_from_linear[cur[9] as usize] as u32) << 18)
                    | ((p_from_linear[cur[10] as usize] as u32) << 20)
                    | ((p_from_linear[cur[11] as usize] as u32) << 22)
                    | ((p_from_linear[cur[12] as usize] as u32) << 24)
                    | ((p_from_linear[cur[13] as usize] as u32) << 26)
                    | ((p_from_linear[cur[14] as usize] as u32) << 28)
                    | ((p_from_linear[cur[15] as usize] as u32) << 30);
                p_dst = &mut p_dst[1..];
            } else {
                p_dst[0] = (p_from_linear[cur[8] as usize] as u32)
                    | ((p_from_linear[cur[9] as usize] as u32) << 2)
                    | ((p_from_linear[cur[10] as usize] as u32) << 4)
                    | ((p_from_linear[cur[11] as usize] as u32) << 6)
                    | ((p_from_linear[cur[12] as usize] as u32) << 8)
                    | ((p_from_linear[cur[13] as usize] as u32) << 10)
                    | ((p_from_linear[cur[14] as usize] as u32) << 12)
                    | ((p_from_linear[cur[15] as usize] as u32) << 14)
                    | ((p_from_linear[cur[0] as usize] as u32) << 16)
                    | ((p_from_linear[cur[1] as usize] as u32) << 18)
                    | ((p_from_linear[cur[2] as usize] as u32) << 20)
                    | ((p_from_linear[cur[3] as usize] as u32) << 22)
                    | ((p_from_linear[cur[4] as usize] as u32) << 24)
                    | ((p_from_linear[cur[5] as usize] as u32) << 26)
                    | ((p_from_linear[cur[6] as usize] as u32) << 28)
                    | ((p_from_linear[cur[7] as usize] as u32) << 30);
                p_dst = &mut p_dst[1..];
            }
        }
        self.codec.stop_decoding();
        true
    }
    pub fn decode_alpha_endpoints(&mut self) -> bool {
        let num_alpha_endpoints = self.p_header.alpha_endpoints.num.cast_to_uint();
        let mut res: bool;
        res = self.codec.start_decoding(
            &self.p_data[self.p_header.alpha_endpoints.ofs.cast_to_uint() as usize..],
            self.p_header.alpha_endpoints.size.cast_to_uint(),
        );
        if !res {
            return res;
        }
        let mut dm = StaticHuffmanDataModel::default();
        res = self.codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        self.alpha_endpoints.resize(num_alpha_endpoints as usize, 0);
        let p_dst: &mut [u16] = &mut self.alpha_endpoints[0..];
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        for p_dst_i in p_dst.iter_mut().take(num_alpha_endpoints as usize) {
            let sa = match self.codec.decode(&dm) {
                Ok(s) => s,
                Err(_) => return false,
            };
            let sb = match self.codec.decode(&dm) {
                Ok(s) => s,
                Err(_) => return false,
            };
            a = (sa + a) & 0xFF;
            b = (sb + b) & 0xFF;
            *p_dst_i = (a | (b << 8)) as u16;
        }
        self.codec.stop_decoding();
        true
    }
    pub fn decode_alpha_selectors(&mut self) -> bool {
        const MAX_SELECTOR_VALUE: u32 = 7;
        const MAX_UNIQUE_SELECTOR_DELTAS: usize = (MAX_SELECTOR_VALUE as usize) * 2 + 1;
        let num_alpha_selectors = self.p_header.alpha_selectors.num.cast_to_uint();
        let mut res: bool;
        res = self.codec.start_decoding(
            &self.p_data[self.p_header.alpha_selectors.ofs.cast_to_uint() as usize..],
            self.p_header.alpha_selectors.size.cast_to_uint(),
        );
        if !res {
            return res;
        }
        let mut dm = StaticHuffmanDataModel::default();
        res = self.codec.decode_receive_static_data_model(&mut dm);
        if !res {
            return res;
        }
        let mut delta0 = [0; (MAX_UNIQUE_SELECTOR_DELTAS * MAX_UNIQUE_SELECTOR_DELTAS)];
        let mut delta1 = [0; (MAX_UNIQUE_SELECTOR_DELTAS * MAX_UNIQUE_SELECTOR_DELTAS)];
        let mut l: i32 = -(MAX_SELECTOR_VALUE as i32);
        let mut m: i32 = -(MAX_SELECTOR_VALUE as i32);
        for i in 0..(MAX_UNIQUE_SELECTOR_DELTAS * MAX_UNIQUE_SELECTOR_DELTAS) {
            delta0[i] = l;
            delta1[i] = m;
            l += 1;
            if l > MAX_SELECTOR_VALUE as i32 {
                l = -(MAX_SELECTOR_VALUE as i32);
                m += 1;
            }
        }
        let mut cur = [0_u32; 16];
        self.alpha_selectors
            .resize(num_alpha_selectors as usize * 3, 0);
        let mut p_dst = &mut self.alpha_selectors[0..];
        let p_from_linear = &DXT5_FROM_LINEAR[0..];
        for _ in 0..num_alpha_selectors as usize {
            for j in 0..8_usize {
                let sym: i32 = match self.codec.decode(&dm) {
                    Ok(s) => s,
                    Err(_) => return false,
                } as i32;
                cur[j * 2] = ((delta0[sym as usize] + cur[j * 2] as i32) & 7) as u32;
                cur[j * 2 + 1] = ((delta1[sym as usize] + cur[j * 2 + 1] as i32) & 7) as u32;
            }
            p_dst[0] = ((p_from_linear[cur[0] as usize] as u32)
                | ((p_from_linear[cur[1] as usize] as u32) << 3)
                | ((p_from_linear[cur[2] as usize] as u32) << 6)
                | ((p_from_linear[cur[3] as usize] as u32) << 9)
                | ((p_from_linear[cur[4] as usize] as u32) << 12)
                | ((p_from_linear[cur[5] as usize] as u32) << 15)) as u16;

            p_dst[1] = (((p_from_linear[cur[5] as usize] as u32) >> 1)
                | ((p_from_linear[cur[6] as usize] as u32) << 2)
                | ((p_from_linear[cur[7] as usize] as u32) << 5)
                | ((p_from_linear[cur[8] as usize] as u32) << 8)
                | ((p_from_linear[cur[9] as usize] as u32) << 11)
                | ((p_from_linear[cur[10] as usize] as u32) << 14)) as u16;

            p_dst[2] = (((p_from_linear[cur[10] as usize] as u32) >> 2)
                | ((p_from_linear[cur[11] as usize] as u32) << 1)
                | ((p_from_linear[cur[12] as usize] as u32) << 4)
                | ((p_from_linear[cur[13] as usize] as u32) << 7)
                | ((p_from_linear[cur[14] as usize] as u32) << 10)
                | ((p_from_linear[cur[15] as usize] as u32) << 13)) as u16;
            p_dst = &mut p_dst[3..];
        }
        self.codec.stop_decoding();
        true
    }
    pub fn crnd_unpack_level(
        &mut self,
        dst_size_in_bytes: u32,
        row_pitch_in_bytes: u32,
        level_index: u32,
    ) -> Result<alloc::vec::Vec<u8>, &'static str> {
        if (dst_size_in_bytes < 8) || (level_index >= CRNMAX_LEVELS) {
            return Err("Destination buffer size is too small.");
        }
        self.unpack_level(dst_size_in_bytes, row_pitch_in_bytes, level_index)
    }
    pub fn unpack_level(
        &mut self,
        dst_size_in_bytes: u32,
        row_pitch_in_bytes: u32,
        level_index: u32,
    ) -> Result<alloc::vec::Vec<u8>, &'static str> {
        let cur_level_ofs = self.p_header.level_ofs[level_index as usize].cast_to_uint();
        let mut next_level_ofs = self.data_size;
        if (level_index + 1) < (self.p_header.levels.cast_to_uint()) {
            next_level_ofs = self.p_header.level_ofs[(level_index + 1) as usize].cast_to_uint();
        }
        if next_level_ofs <= cur_level_ofs {
            return Err("Level offset mismatch.");
        }
        self.unpack_level_2(
            &self.p_data[cur_level_ofs as usize..],
            next_level_ofs - cur_level_ofs,
            dst_size_in_bytes,
            row_pitch_in_bytes,
            level_index,
        )
    }
    pub fn unpack_level_2(
        &mut self,
        p_src: &'slice [u8],
        src_size_in_bytes: u32,
        dst_size_in_bytes: u32,
        mut row_pitch_in_bytes: u32,
        level_index: u32,
    ) -> Result<alloc::vec::Vec<u8>, &'static str> {
        let width: u32 = core::cmp::max(self.p_header.width.cast_to_uint() >> level_index, 1);
        let height: u32 = core::cmp::max(self.p_header.height.cast_to_uint() >> level_index, 1);
        let blocks_x: u32 = (width + 3) >> 2;
        let blocks_y: u32 = (height + 3) >> 2;
        let block_size: u32 = if self.p_header.format.cast_to_uint() == CrnFormat::Dxt1 as u32
            || self.p_header.format.cast_to_uint() == CrnFormat::Dxt5a as u32
        {
            8
        } else {
            16
        };
        let minimal_row_pitch: u32 = block_size * blocks_x;
        if row_pitch_in_bytes == 0 {
            row_pitch_in_bytes = minimal_row_pitch;
        } else if row_pitch_in_bytes < minimal_row_pitch || (row_pitch_in_bytes & 3) != 0 {
            return Err("Crunch Row size is below the minimum allowed.");
        }
        let mut ret = alloc::vec![0_u8; dst_size_in_bytes as usize];
        if dst_size_in_bytes < (row_pitch_in_bytes * blocks_y) {
            return Err("Destination buffer size is smaller than what expected to decompress.");
        }
        let chunks_x: u32 = (blocks_x + 1) >> 1;
        let chunks_y: u32 = (blocks_y + 1) >> 1;
        let res: bool = self.codec.start_decoding(p_src, src_size_in_bytes);
        if !res {
            return Err("Failed to initialize the decoding process.");
        }
        let format = match self.p_header.format.cast_to_uint() {
            0 => CrnFormat::Dxt1,
            1 => CrnFormat::Dxt3,
            2 => CrnFormat::CCrnfmtDxt5,
            3 => CrnFormat::Dxt5CcxY,
            4 => CrnFormat::Dxt5XGxR,
            5 => CrnFormat::Dxt5XGbr,
            6 => CrnFormat::Dxt5Agbr,
            7 => CrnFormat::DxnXy,
            8 => CrnFormat::DxnYx,
            9 => CrnFormat::Dxt5a,
            10 => CrnFormat::Etc1,
            11 => CrnFormat::Total,
            _ => CrnFormat::Invalid,
        };
        let unpack_res = match format {
            CrnFormat::Dxt1 => self.unpack_dxt1(
                &mut ret,
                row_pitch_in_bytes,
                blocks_x,
                blocks_y,
                chunks_x,
                chunks_y,
            ),

            CrnFormat::CCrnfmtDxt5
            | CrnFormat::Dxt5CcxY
            | CrnFormat::Dxt5XGbr
            | CrnFormat::Dxt5Agbr
            | CrnFormat::Dxt5XGxR => self.unpack_dxt5(
                &mut ret,
                row_pitch_in_bytes,
                blocks_x,
                blocks_y,
                chunks_x,
                chunks_y,
            ),

            CrnFormat::Dxt5a => self.unpack_dxt5a(
                &mut ret,
                row_pitch_in_bytes,
                blocks_x,
                blocks_y,
                chunks_x,
                chunks_y,
            ),

            CrnFormat::DxnXy | CrnFormat::DxnYx => self.unpack_dxn(
                &mut ret,
                row_pitch_in_bytes,
                blocks_x,
                blocks_y,
                chunks_x,
                chunks_y,
            ),

            _ => return Err("Invalid format for unpacking."),
        };
        match unpack_res {
            Ok(unpack_res) => unpack_res,
            Err(unpack_res) => return Err(unpack_res),
        };
        self.codec.stop_decoding();
        Ok(ret)
    }
    pub fn unpack_dxt1(
        &mut self,
        p_dst: &mut [u8],
        row_pitch_in_bytes: u32,
        blocks_x: u32,
        blocks_y: u32,
        chunks_x: u32,
        chunks_y: u32,
    ) -> Result<bool, &'static str> {
        let mut chunk_encoding_bits: u32 = 1;
        let num_color_endpoints: u32 = self.color_endpoints.len() as u32;
        let num_color_selectors: u32 = self.color_selectors.len() as u32;
        let mut prev_color_endpoint_index: u32 = 0;
        let mut prev_color_selector_index: u32 = 0;
        let num_faces: u32 = self.p_header.faces.cast_to_uint();
        let row_pitch_in_dwords = row_pitch_in_bytes >> 2;
        let c_bytes_per_block: i32 = 8;
        for f in 0..num_faces as usize {
            let mut row_dst = f;
            for y in 0..chunks_y {
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item = i32>>;
                let block_delta: i32;
                if y & 1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev());
                    block_delta = -c_bytes_per_block * 2;
                    block_dst += (((chunks_x as i32) - 1) * c_bytes_per_block * 2) as usize;
                } else {
                    block_delta = c_bytes_per_block * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter {
                    let mut color_endpoints = [0_u32; 4];
                    if chunk_encoding_bits == 1 {
                        chunk_encoding_bits = match self.codec.decode(&self.chunk_encoding_dm) {
                            Ok(chunk_encoding_bits) => chunk_encoding_bits,
                            Err(_) => return Err("Failed to decord DXT1 Texture"),
                        };
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;

                    let num_tiles = CRND_CHUNK_ENCODING_NUM_TILES[chunk_encoding_index as usize];
                    for color_endpoint in color_endpoints.iter_mut().take(num_tiles as usize) {
                        let delta: u32 = match self.codec.decode(&self.endpoint_delta_dm[0]) {
                            Ok(delta) => delta,
                            Err(_) => return Err("Failed to decord DXT1 Texture"),
                        };
                        prev_color_endpoint_index += delta;
                        limit(&mut prev_color_endpoint_index, num_color_endpoints);
                        *color_endpoint = self.color_endpoints[prev_color_endpoint_index as usize];
                    }

                    let p_tile_indices =
                        CRND_CHUNK_ENCODING_TILES[chunk_encoding_index as usize].tiles;
                    let skip_right_col = ((blocks_x & 1) == 1) && (x == (chunks_x as i32 - 1));
                    let mut pd_dst = block_dst >> 2;
                    if !skip_bottom_row && !skip_right_col {
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst,
                            color_endpoints[p_tile_indices[0] as usize]
                        );

                        let delta0: u32 = match self.codec.decode(&self.selector_delta_dm[0]) {
                            Ok(delta0) => delta0,
                            Err(_) => return Err("Failed to decord DXT1 Texture"),
                        };
                        prev_color_selector_index += delta0;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst + 1,
                            self.color_selectors[prev_color_selector_index as usize]
                        );
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst + 2,
                            color_endpoints[p_tile_indices[1] as usize]
                        );

                        let delta1: u32 = match self.codec.decode(&self.selector_delta_dm[0]) {
                            Ok(delta1) => delta1,
                            Err(_) => return Err("Failed to decord DXT1 Texture"),
                        };
                        prev_color_selector_index += delta1;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst + 3,
                            self.color_selectors[prev_color_selector_index as usize]
                        );
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst + row_pitch_in_dwords as usize,
                            color_endpoints[p_tile_indices[2] as usize]
                        );

                        let delta2: u32 = match self.codec.decode(&self.selector_delta_dm[0]) {
                            Ok(delta2) => delta2,
                            Err(_) => return Err("Failed to decord DXT1 Texture"),
                        };
                        prev_color_selector_index += delta2;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst + 1 + row_pitch_in_dwords as usize,
                            self.color_selectors[prev_color_selector_index as usize]
                        );
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst + 2 + row_pitch_in_dwords as usize,
                            color_endpoints[p_tile_indices[3] as usize]
                        );

                        let delta3: u32 = match self.codec.decode(&self.selector_delta_dm[0]) {
                            Ok(delta3) => delta3,
                            Err(_) => return Err("Failed to decord DXT1 Texture"),
                        };
                        prev_color_selector_index += delta3;
                        limit(&mut prev_color_selector_index, num_color_selectors);
                        WRITE_TO_INT_BUFFER!(
                            p_dst,
                            pd_dst + 3 + row_pitch_in_dwords as usize,
                            self.color_selectors[prev_color_selector_index as usize]
                        );
                    } else {
                        for by in 0..2 {
                            pd_dst = block_dst + (row_pitch_in_bytes * by) as usize;
                            pd_dst >>= 2;
                            for bx in 0..2 {
                                let delta: u32 = match self.codec.decode(&self.selector_delta_dm[0])
                                {
                                    Ok(delta) => delta,
                                    Err(_) => return Err("Failed to decord DXT1 Texture"),
                                };
                                prev_color_selector_index += delta;
                                limit(&mut prev_color_selector_index, num_color_selectors);
                                if !(((bx != 0) && skip_right_col)
                                    || ((by != 0) && skip_bottom_row))
                                {
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst,
                                        color_endpoints
                                            [p_tile_indices[(bx + by * 2) as usize] as usize]
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 1,
                                        self.color_selectors[prev_color_selector_index as usize]
                                    );
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
        Ok(true)
    }
    pub fn unpack_dxt5(
        &mut self,
        p_dst: &mut [u8],
        row_pitch_in_bytes: u32,
        blocks_x: u32,
        blocks_y: u32,
        chunks_x: u32,
        chunks_y: u32,
    ) -> Result<bool, &'static str> {
        let mut chunk_encoding_bits: u32 = 1;
        let num_color_endpoints: u32 = self.color_endpoints.len() as u32;
        let num_color_selectors: u32 = self.color_selectors.len() as u32;
        let num_alpha_endpoints: u32 = self.alpha_endpoints.len() as u32;
        let num_alpha_selectors: u32 = self.p_header.alpha_selectors.num.cast_to_uint();
        let mut prev_color_endpoint_index: u32 = 0;
        let mut prev_color_selector_index: u32 = 0;
        let mut prev_alpha_endpoint_index: u32 = 0;
        let mut prev_alpha_selector_index: u32 = 0;
        let num_faces = self.p_header.faces.cast_to_uint();
        let c_bytes_per_block: i32 = 16;
        for f in 0..num_faces as usize {
            let mut row_dst = f;
            for y in 0..chunks_y {
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item = i32>>;
                let block_delta: i32;
                if y & 1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev());
                    block_delta = -c_bytes_per_block * 2;
                    block_dst += (((chunks_x as i32) - 1) * c_bytes_per_block * 2) as usize;
                } else {
                    block_delta = c_bytes_per_block * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter {
                    let mut color_endpoints = [0_u32; 4];
                    let mut alpha_endpoints = [0_u32; 4];
                    if chunk_encoding_bits == 1 {
                        chunk_encoding_bits = match self.codec.decode(&self.chunk_encoding_dm) {
                            Ok(chunk_encoding_bits) => chunk_encoding_bits,
                            Err(_) => return Err("Failed to decord DXT5 Texture"),
                        };
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index: u32 = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;
                    let num_tiles =
                        CRND_CHUNK_ENCODING_NUM_TILES[chunk_encoding_index as usize] as u32;
                    let p_tile_indices =
                        CRND_CHUNK_ENCODING_TILES[chunk_encoding_index as usize].tiles;
                    let skip_right_col = (blocks_x & 1) != 0 && (x == ((chunks_x as i32) - 1));
                    for i in 0..num_tiles {
                        let delta: u32 = match self.codec.decode(&self.endpoint_delta_dm[1]) {
                            Ok(delta) => delta,
                            Err(_) => return Err("Failed to decord DXT5 Texture"),
                        };
                        prev_alpha_endpoint_index += delta;
                        limit(&mut prev_alpha_endpoint_index, num_alpha_endpoints);
                        alpha_endpoints[i as usize] =
                            self.alpha_endpoints[prev_alpha_endpoint_index as usize] as u32;
                    }

                    for i in 0..num_tiles {
                        let delta: u32 = match self.codec.decode(&self.endpoint_delta_dm[0]) {
                            Ok(delta) => delta,
                            Err(_) => return Err("Failed to decord DXT5 Texture"),
                        };
                        prev_color_endpoint_index += delta;
                        limit(&mut prev_color_endpoint_index, num_color_endpoints);
                        color_endpoints[i as usize] =
                            self.color_endpoints[prev_color_endpoint_index as usize];
                    }

                    let mut pd_dst = block_dst;
                    pd_dst >>= 2;
                    for by in 0..2 {
                        for bx in 0..2 {
                            let delta0: u32 = match self.codec.decode(&self.selector_delta_dm[1]) {
                                Ok(delta0) => delta0,
                                Err(_) => return Err("Failed to decord DXT5 Texture"),
                            };
                            prev_alpha_selector_index += delta0;
                            limit(&mut prev_alpha_selector_index, num_alpha_selectors);
                            let delta1: u32 = match self.codec.decode(&self.selector_delta_dm[0]) {
                                Ok(delta1) => delta1,
                                Err(_) => return Err("Failed to decord DXT5 Texture"),
                            };
                            prev_color_selector_index += delta1;
                            limit(&mut prev_color_selector_index, num_color_selectors);
                            if !(((bx != 0) && skip_right_col) || ((by != 0) && skip_bottom_row)) {
                                let tile_index: u32 = p_tile_indices[bx + by * 2] as u32;
                                let p_alpha_selectors = &self.alpha_selectors
                                    [(prev_alpha_selector_index * 3) as usize..];
                                #[cfg(target_endian = "big")]
                                {
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 0,
                                        ((alpha_endpoints[tile_index as usize] << 16)
                                            | pAlpha_selectors[0] as u32)
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 1,
                                        (((pAlpha_selectors[1] as u32) << 16)
                                            | (pAlpha_selectors[2] as u32))
                                            as u32
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 2,
                                        (color_endpoints[tile_index as usize])
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 3,
                                        (self.m_color_selectors
                                            [prev_color_selector_index as usize])
                                    );
                                }
                                #[cfg(target_endian = "little")]
                                {
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst,
                                        (alpha_endpoints[tile_index as usize]
                                            | ((p_alpha_selectors[0] as u32) << 16))
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 1,
                                        ((p_alpha_selectors[1] as u32)
                                            | ((p_alpha_selectors[2] as u32) << 16))
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 2,
                                        (color_endpoints[tile_index as usize])
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 3,
                                        (self.color_selectors[prev_color_selector_index as usize])
                                    );
                                }
                            }
                            pd_dst += 4;
                        }
                        pd_dst <<= 2;
                        pd_dst = ((pd_dst as i32)
                            + (-c_bytes_per_block * 2 + row_pitch_in_bytes as i32))
                            as usize;
                        pd_dst >>= 2;
                    }
                    block_dst = ((block_dst as i32) + block_delta) as usize;
                }
                row_dst += (row_pitch_in_bytes * 2) as usize;
            }
        }
        Ok(true)
    }
    pub fn unpack_dxt5a(
        &mut self,
        p_dst: &mut [u8],
        row_pitch_in_bytes: u32,
        blocks_x: u32,
        blocks_y: u32,
        chunks_x: u32,
        chunks_y: u32,
    ) -> Result<bool, &'static str> {
        let mut chunk_encoding_bits: u32 = 1;
        let num_alpha_endpoints: u32 = self.alpha_endpoints.len() as u32;
        let num_alpha_selectors: u32 = self.p_header.alpha_selectors.num.cast_to_uint();
        let mut prev_alpha0_endpoint_index: u32 = 0;
        let mut prev_alpha0_selector_index: u32 = 0;
        let num_faces = self.p_header.faces.cast_to_uint();
        let c_bytes_per_block = 8;
        for f in 0..num_faces as usize {
            let mut row_dst = f;
            for y in 0..chunks_y {
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item = i32>>;
                let block_delta: i32;
                if y & 1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev());
                    block_delta = -c_bytes_per_block * 2;
                    block_dst += (((chunks_x as i32) - 1) * c_bytes_per_block * 2) as usize;
                } else {
                    block_delta = c_bytes_per_block * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter {
                    let mut alpha0_endpoints = [0_u32; 4];
                    if chunk_encoding_bits == 1 {
                        chunk_encoding_bits = match self.codec.decode(&self.chunk_encoding_dm) {
                            Ok(chunk_encoding_bits) => chunk_encoding_bits,
                            Err(_) => return Err("Failed to decord DXT5A Texture"),
                        };
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index: u32 = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;
                    let num_tiles =
                        CRND_CHUNK_ENCODING_NUM_TILES[chunk_encoding_index as usize] as u32;
                    let p_tile_indices =
                        CRND_CHUNK_ENCODING_TILES[chunk_encoding_index as usize].tiles;
                    let skip_right_col = (blocks_x & 1) != 0 && (x == ((chunks_x as i32) - 1));
                    for i in 0..num_tiles {
                        let delta: u32 = match self.codec.decode(&self.endpoint_delta_dm[1]) {
                            Ok(delta) => delta,
                            Err(_) => return Err("Failed to decord DXT5A Texture"),
                        };
                        prev_alpha0_endpoint_index += delta;
                        limit(&mut prev_alpha0_endpoint_index, num_alpha_endpoints);
                        alpha0_endpoints[i as usize] =
                            self.alpha_endpoints[prev_alpha0_endpoint_index as usize] as u32;
                    }
                    let mut pd_dst = block_dst;
                    pd_dst >>= 2;
                    for by in 0..2 {
                        for bx in 0..2 {
                            let delta: u32 = match self.codec.decode(&self.selector_delta_dm[1]) {
                                Ok(delta) => delta,
                                Err(_) => return Err("Failed to decord DXT5A Texture"),
                            };
                            prev_alpha0_selector_index += delta;
                            limit(&mut prev_alpha0_selector_index, num_alpha_selectors);
                            if !(((bx != 0) && skip_right_col) || ((by != 0) && skip_bottom_row)) {
                                let tile_index: u32 = p_tile_indices[bx + by * 2] as u32;
                                let p_alpha0_selectors = &self.alpha_selectors
                                    [(prev_alpha0_selector_index * 3) as usize..];
                                #[cfg(target_endian = "big")]
                                {
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 0,
                                        ((alpha0_endpoints[tile_index as usize] << 16)
                                            | pAlpha0_selectors[0] as u32)
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 1,
                                        (((pAlpha0_selectors[1] as u32) << 16)
                                            | (pAlpha0_selectors[2] as u32))
                                            as u32
                                    );
                                }
                                #[cfg(target_endian = "little")]
                                {
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst,
                                        (alpha0_endpoints[tile_index as usize]
                                            | ((p_alpha0_selectors[0] as u32) << 16))
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 1,
                                        ((p_alpha0_selectors[1] as u32)
                                            | ((p_alpha0_selectors[2] as u32) << 16))
                                    );
                                }
                            }
                            pd_dst += 2;
                        }
                        pd_dst <<= 2;
                        pd_dst = ((pd_dst as i32)
                            + (-c_bytes_per_block * 2 + row_pitch_in_bytes as i32))
                            as usize;
                        pd_dst >>= 2;
                    }
                    block_dst = ((block_dst as i32) + block_delta) as usize;
                }
                row_dst += (row_pitch_in_bytes * 2) as usize;
            }
        }
        Ok(true)
    }
    pub fn unpack_dxn(
        &mut self,
        p_dst: &mut [u8],
        row_pitch_in_bytes: u32,
        blocks_x: u32,
        blocks_y: u32,
        chunks_x: u32,
        chunks_y: u32,
    ) -> Result<bool, &'static str> {
        let mut chunk_encoding_bits: u32 = 1;
        let num_alpha_endpoints: u32 = self.alpha_endpoints.len() as u32;
        let num_alpha_selectors: u32 = self.p_header.alpha_selectors.num.cast_to_uint();
        let mut prev_alpha0_endpoint_index: u32 = 0;
        let mut prev_alpha0_selector_index: u32 = 0;
        let mut prev_alpha1_endpoint_index: u32 = 0;
        let mut prev_alpha1_selector_index: u32 = 0;
        let num_faces: u32 = self.p_header.faces.cast_to_uint();
        let c_bytes_per_block: i32 = 16;
        for f in 0..num_faces as usize {
            let mut row_dst = f;
            for y in 0..chunks_y {
                let mut block_dst = row_dst;
                let iter: alloc::boxed::Box<dyn Iterator<Item = i32>>;
                let block_delta: i32;
                if y & 1 == 1 {
                    iter = alloc::boxed::Box::new((0..chunks_x as i32).rev());
                    block_delta = -c_bytes_per_block * 2;
                    block_dst += (((chunks_x as i32) - 1) * c_bytes_per_block * 2) as usize;
                } else {
                    block_delta = c_bytes_per_block * 2;
                    iter = alloc::boxed::Box::new(0..chunks_x as i32);
                }
                let skip_bottom_row = (y == (chunks_y - 1)) && ((blocks_y & 1) == 1);
                for x in iter {
                    let mut alpha0_endpoints = [0_u32; 4];
                    let mut alpha1_endpoints = [0_u32; 4];
                    if chunk_encoding_bits == 1 {
                        chunk_encoding_bits = match self.codec.decode(&self.chunk_encoding_dm) {
                            Ok(chunk_encoding_bits) => chunk_encoding_bits,
                            Err(_) => return Err("Failed to decord DXN Texture"),
                        };
                        chunk_encoding_bits |= 512;
                    }
                    let chunk_encoding_index: u32 = chunk_encoding_bits & 7;
                    chunk_encoding_bits >>= 3;
                    let num_tiles =
                        CRND_CHUNK_ENCODING_NUM_TILES[chunk_encoding_index as usize] as u32;
                    let p_tile_indices =
                        CRND_CHUNK_ENCODING_TILES[chunk_encoding_index as usize].tiles;
                    let skip_right_col = (blocks_x & 1) != 0 && (x == ((chunks_x as i32) - 1));
                    for i in 0..num_tiles {
                        let delta: u32 = match self.codec.decode(&self.endpoint_delta_dm[1]) {
                            Ok(delta) => delta,
                            Err(_) => return Err("Failed to decord DXN Texture"),
                        };
                        prev_alpha0_endpoint_index += delta;
                        limit(&mut prev_alpha0_endpoint_index, num_alpha_endpoints);
                        alpha0_endpoints[i as usize] =
                            self.alpha_endpoints[prev_alpha0_endpoint_index as usize] as u32;
                    }
                    for i in 0..num_tiles {
                        let delta: u32 = match self.codec.decode(&self.endpoint_delta_dm[1]) {
                            Ok(delta) => delta,
                            Err(_) => return Err("Failed to decord DXN Texture"),
                        };
                        prev_alpha1_endpoint_index += delta;
                        limit(&mut prev_alpha1_endpoint_index, num_alpha_endpoints);
                        alpha1_endpoints[i as usize] =
                            self.alpha_endpoints[prev_alpha1_endpoint_index as usize] as u32;
                    }
                    let mut pd_dst = block_dst;
                    pd_dst >>= 2;
                    for by in 0..2 {
                        for bx in 0..2 {
                            let delta0: u32 = match self.codec.decode(&self.selector_delta_dm[1]) {
                                Ok(delta0) => delta0,
                                Err(_) => return Err("Failed to decord DXN Texture"),
                            };
                            prev_alpha0_selector_index += delta0;
                            limit(&mut prev_alpha0_selector_index, num_alpha_selectors);

                            let delta1: u32 = match self.codec.decode(&self.selector_delta_dm[1]) {
                                Ok(delta1) => delta1,
                                Err(_) => return Err("Failed to decord DXN Texture"),
                            };
                            prev_alpha1_selector_index += delta1;
                            limit(&mut prev_alpha1_selector_index, num_alpha_selectors);
                            if !(((bx != 0) && skip_right_col) || ((by != 0) && skip_bottom_row)) {
                                let tile_index: u32 = p_tile_indices[bx + by * 2] as u32;
                                let p_alpha0_selectors = &self.alpha_selectors
                                    [(prev_alpha0_selector_index * 3) as usize..];
                                let p_alpha1_selectors = &self.alpha_selectors
                                    [(prev_alpha1_selector_index * 3) as usize..];
                                #[cfg(target_endian = "big")]
                                {
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 0,
                                        ((alpha0_endpoints[tile_index as usize] << 16)
                                            | pAlpha0_selectors[0] as u32)
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 1,
                                        (((pAlpha0_selectors[1] as u32) << 16)
                                            | (pAlpha0_selectors[2] as u32))
                                            as u32
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 2,
                                        ((alpha1_endpoints[tile_index as usize] << 16)
                                            | pAlpha1_selectors[0] as u32)
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        pDst,
                                        pd_dst + 3,
                                        (((pAlpha1_selectors[1] as u32) << 16)
                                            | (pAlpha1_selectors[2] as u32))
                                            as u32
                                    );
                                }
                                #[cfg(target_endian = "little")]
                                {
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst,
                                        (alpha0_endpoints[tile_index as usize]
                                            | ((p_alpha0_selectors[0] as u32) << 16))
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 1,
                                        ((p_alpha0_selectors[1] as u32)
                                            | ((p_alpha0_selectors[2] as u32) << 16))
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 2,
                                        (alpha1_endpoints[tile_index as usize]
                                            | ((p_alpha1_selectors[0] as u32) << 16))
                                    );
                                    WRITE_TO_INT_BUFFER!(
                                        p_dst,
                                        pd_dst + 3,
                                        ((p_alpha1_selectors[1] as u32)
                                            | ((p_alpha1_selectors[2] as u32) << 16))
                                    );
                                }
                            }
                            pd_dst += 4;
                        }
                        pd_dst <<= 2;
                        pd_dst = ((pd_dst as i32)
                            + (-c_bytes_per_block * 2 + row_pitch_in_bytes as i32))
                            as usize;
                        pd_dst >>= 2;
                    }
                    block_dst = ((block_dst as i32) + block_delta) as usize;
                }
                row_dst += (row_pitch_in_bytes * 2) as usize;
            }
        }
        Ok(true)
    }
}

use super::crn_utils::*;
use super::crn_consts::*;
use super::crn_static_huffman_data_model::*;

#[allow(non_camel_case_types)]
pub struct symbol_codec<'slice>{
    pub m_p_decode_buf: &'slice[u8],
    pub m_p_decode_buf_next: &'slice[u8],
    pub m_p_decode_buf_end: *const u8,
    pub m_decode_buf_size: u32,
    pub m_bit_buf: u32,
    pub m_bit_count: i32
}

impl<'slice> Default for symbol_codec<'slice>{
    fn default() -> Self {
        symbol_codec{
            m_p_decode_buf: &[0; 0],
            m_p_decode_buf_next: &[0; 0],
            m_p_decode_buf_end: core::ptr::null(),
            m_decode_buf_size: 0,
            m_bit_buf: 0,
            m_bit_count: 0
        }
    }
}

impl<'slice> symbol_codec<'slice>{
    pub fn start_decoding(&mut self, p_buf: &'slice[u8], buf_size: u32) -> bool{
        if buf_size == 0 {
            return false;
        }
        self.m_p_decode_buf = p_buf;
        self.m_p_decode_buf_next = p_buf;
        self.m_decode_buf_size = buf_size;
        self.m_p_decode_buf_end = (&p_buf[buf_size as usize]) as *const u8;
        self.get_bits_init();
        true
    }

    pub fn decode_receive_static_data_model(&mut self, model: &mut StaticHuffmanDataModel) -> bool{
        let total_used_syms = match self.decode_bits(total_bits(C_MAX_SUPPORTED_SYMS as u32)){
            Ok(total_used_syms) => total_used_syms,
            Err(_) => return false
        };
        
        if total_used_syms == 0 {
            model.clear();
        }
        
        model.m_code_sizes.resize(total_used_syms as usize, 0);
        
        let num_codelength_codes_to_send = match self.decode_bits(5){
            Ok(num_codelength_codes_to_send) => num_codelength_codes_to_send, 
            Err(_) => return false
        };
        
        if (num_codelength_codes_to_send < 1) || (num_codelength_codes_to_send > C_MAX_CODELENGTH_CODES as u32) {
            return false;
        }
        
        let mut dm = StaticHuffmanDataModel::default();
        dm.m_code_sizes.resize(C_MAX_CODELENGTH_CODES, 0);
        
        for i in 0..num_codelength_codes_to_send as usize{
            dm.m_code_sizes[G_MOST_PROBABLE_CODELENGTH_CODES[i] as usize] = match self.decode_bits(3){
                Ok(s) => s,
                Err(_) => return false
            } as u8;
        }
        
        if !dm.prepare_decoder_tables() {
            return false;
        }
        
        let mut ofs: u32 = 0;
        while ofs < total_used_syms {
            let num_remaining: u32 = total_used_syms - ofs;
            
            let code: usize = match self.decode(&dm){
                Ok(s) => s as usize,
                Err(_) => return false
            };
            
            if code <= 16 {
                model.m_code_sizes[ofs as usize] = code as u8;
                ofs += 1;
            }else if code == C_SMALL_ZERO_RUN_CODE {
                let len = match self.decode_bits(C_SMALL_ZERO_RUN_EXTRA_BITS as u32) {
                    Ok(s) => s,
                    Err(_) => return false
                } + C_MIN_SMALL_ZERO_RUN_SIZE as u32;
            
                if len > num_remaining {
                    return false;
                }
            
                ofs += len;
            }else if code == C_LARGE_ZERO_RUN_CODE {
                let len = match self.decode_bits(C_LARGE_ZERO_RUN_EXTRA_BITS as u32) {
                    Ok(s) => s,
                    Err(_) => return false
                } + C_MIN_LARGE_ZERO_RUN_SIZE as u32;
            
                if len > num_remaining {
                    return false;
                }
            
                ofs += len;
            }else if (code == C_SMALL_REPEAT_CODE) || (code == C_LARGE_REPEAT_CODE) {
                let len: u32;
            
                if code == C_SMALL_REPEAT_CODE {
                    match self.decode_bits(C_SMALL_NON_ZERO_RUN_EXTRA_BITS as u32) {
                        Ok(s) => {len = s + C_SMALL_MIN_NON_ZERO_RUN_SIZE as u32;},
                        Err(_) => return false
                    };
                }else{
                    match self.decode_bits(C_LARGE_NON_ZERO_RUN_EXTRA_BITS as u32) {
                        Ok(s) => {len = s + C_LARGE_MIN_NON_ZERO_RUN_SIZE as u32;},
                        Err(_) => return false
                    };
                }
            
                if ofs == 0 || len > num_remaining {
                    return false;
                }
            
                let prev: u32 = model.m_code_sizes[(ofs - 1) as usize] as u32;
                if prev == 0 {
                    return false;
                }
            
                let end = ofs + len;
                while ofs < end {
                    model.m_code_sizes[ofs as usize] = prev as u8;
                    ofs += 1;
                }
            }else{
                return false;
            }
        }
        model.prepare_decoder_tables()
    }

    pub fn decode_bits(&mut self, num_bits: u32) -> Result<u32, bool>{
        if num_bits == 0_u32 {
            return Ok(0_u32);
        }
        if num_bits > 16_u32 {
            let a = match self.get_bits(num_bits - 16){
                Ok(s) => s,
                Err(_) => return Err(false)
            };
            let b = match self.get_bits(16){
                Ok(s) => s,
                Err(_) => return Err(false)
            };
            Ok((a << 16) | b)
        }else{
            self.get_bits(num_bits)
        }
    }
    
    pub fn decode(&mut self, model: &StaticHuffmanDataModel) -> Result<u32, bool>{
        let p_tables = &model.m_p_decode_tables;
        if  self.m_bit_count < 24 {
            if  self.m_bit_count < 16 {
                let mut c0: u32 = 0;
                let mut c1: u32 = 0;
                let mut p = self.m_p_decode_buf_next;
                if  (&(p[0]) as *const u8) < self.m_p_decode_buf_end {c0 = p[0] as u32; p = &p[1..]};
                if  (&(p[0]) as *const u8) < self.m_p_decode_buf_end {c1 = p[0] as u32; p = &p[1..]};
                self.m_p_decode_buf_next = p;
                self.m_bit_count += 16;
                let c: u32 = (c0 << 8) | c1;
                self.m_bit_buf |= c << (32 - self.m_bit_count);
            }else{
                let c: u32;
                if (&(self.m_p_decode_buf_next[0]) as *const u8) < self.m_p_decode_buf_end {
                    c = self.m_p_decode_buf_next[0] as u32;
                    self.m_p_decode_buf_next = &self.m_p_decode_buf_next[1..];
                }else{
                    c = 0
                };
                self.m_bit_count += 8;
                self.m_bit_buf |= c << (32 - self.m_bit_count);
            }
        }
        let k: u32 = (self.m_bit_buf >> 16) + 1;
        let sym: u32;
        let mut len: u32;
        if k <= p_tables.m_table_max_code {
            let t = p_tables.m_lookup[(self.m_bit_buf >> (32 - p_tables.m_table_bits)) as usize];
            if t == u32::MAX{
                return Err(false);
            }
            sym = t & (u16::MAX as u32);
            len = t >> 16;
            if model.m_code_sizes[sym as usize] as u32 != len{
                return Err(false);
            }
        }else{
            len = p_tables.m_decode_start_code_size;
            if len == 0 {
                len = len;
            }
            loop{
                if k <= p_tables.m_max_codes[(len - 1) as usize] {
                    break;
                }
                len += 1;
            }
            let val_ptr: i32 = p_tables.m_val_ptrs[(len - 1) as usize] + (self.m_bit_buf >> (32 - len)) as i32;
            if (val_ptr as u32) >= model.m_total_syms {
                return Err(false);
            }
            sym = p_tables.m_sorted_symbol_order[val_ptr as usize] as u32;
        }
        self.m_bit_buf <<= len;
        self.m_bit_count -= len as i32;
        Ok(sym)
    }
    
    pub fn stop_decoding(&mut self){
    }
    
    fn get_bits_init(&mut self){
        self.m_bit_buf = 0;
        self.m_bit_count = 0;
    }
    
    fn get_bits(&mut self, num_bits: u32) -> Result<u32, bool>{
        if num_bits > 32{
            return Err(false);
        }
        while self.m_bit_count < num_bits as i32 {
            let mut c: u32 = 0;
            if self.m_p_decode_buf_next[0] as *const u8 != self.m_p_decode_buf_end {
                c = self.m_p_decode_buf_next[0] as u32;
                self.m_p_decode_buf_next = &self.m_p_decode_buf_next[1..];
            }
            self.m_bit_count += 8;
            if self.m_bit_count > C_BIT_BUF_SIZE as i32{
                return Err(false);
            }
            self.m_bit_buf |= c << (C_BIT_BUF_SIZE - self.m_bit_count as usize);
        }
        let result: u32 = self.m_bit_buf >> (C_BIT_BUF_SIZE - num_bits as usize);
        self.m_bit_buf <<= num_bits;
        self.m_bit_count -= num_bits as i32;
        Ok(result)
    }
}

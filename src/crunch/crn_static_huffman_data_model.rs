use super::crn_utils::*;
use super::crn_consts::*;
use core::cmp::{min, max};
extern crate alloc;

#[derive(Default)]
pub struct DecoderTables{
    pub m_num_syms: u32,
    pub m_total_used_syms: u32,
    pub m_table_bits: u32,
    pub m_table_shift: u32,
    pub m_table_max_code: u32,
    pub m_decode_start_code_size: u32,

    pub m_min_code_size: u8,
    pub m_max_code_size: u8,

    pub m_max_codes: [u32; C_MAX_EXPECTED_CODE_SIZE + 1],
    pub m_val_ptrs: [i32;  C_MAX_EXPECTED_CODE_SIZE + 1],

    pub m_cur_lookup_size: u32,
    pub m_lookup: alloc::vec::Vec<u32>,

    pub m_cur_sorted_symbol_order_size: u32,
    pub m_sorted_symbol_order: alloc::vec::Vec<u16>
}

impl DecoderTables{
    pub fn init(&mut self, num_syms: u32, p_codesizes: &[u8], mut table_bits: u32) -> bool{
        let mut min_codes = [0 as u32; C_MAX_EXPECTED_CODE_SIZE];
        
        if num_syms == (0 as u32) || table_bits > C_MAX_TABLE_BITS as u32 {
            return false;
        
        }
        self.m_num_syms = num_syms;
        let mut num_codes = [0 as u32; (C_MAX_EXPECTED_CODE_SIZE + 1)];
        
        for i in 0..num_syms as usize{
            let c = p_codesizes[i];
            if c != 0 {
                num_codes[c as usize] += 1;
            }
        
        }
        let mut sorted_positions = [0 as u32; (C_MAX_EXPECTED_CODE_SIZE + 1)];
        let mut cur_code: u32 = 0;
        let mut total_used_syms: u32 = 0;
        let mut max_code_size: u32 = 0;
        let mut min_code_size: u32 = u32::MAX;
        
        for i in 1..=C_MAX_EXPECTED_CODE_SIZE{
            let n = num_codes[i];

            if n == 0 {
                self.m_max_codes[i - 1] = 0;
            }else{
                min_code_size = min(min_code_size, i as u32);
                max_code_size = max(max_code_size, i as u32);
        
                min_codes[i - 1] = cur_code;
        
                self.m_max_codes[i - 1] = cur_code + n - 1;
                self.m_max_codes[i - 1] = 1 + ((self.m_max_codes[i - 1] << (16 - i)) | ((1 << (16 - i)) - 1));
                self.m_val_ptrs[i - 1] = total_used_syms as i32;
                sorted_positions[i] = total_used_syms;

                cur_code += n;
                total_used_syms += n;
            }
            cur_code <<= 1;
        }

        self.m_total_used_syms = total_used_syms;
        if total_used_syms > self.m_cur_sorted_symbol_order_size {
            self.m_cur_sorted_symbol_order_size = total_used_syms;

            if is_power_of_2(total_used_syms as usize) == false {
                self.m_cur_sorted_symbol_order_size = min(num_syms, next_pow2(total_used_syms as usize) as u32);
            }

            self.m_sorted_symbol_order = alloc::vec![0; self.m_cur_sorted_symbol_order_size as usize];
        }

        self.m_min_code_size = min_code_size as u8;
        self.m_max_code_size = max_code_size as u8;

        for i in 0..num_syms as usize{
            let c: u32 = p_codesizes[i] as u32;

            if c != 0 {
                if num_codes[c as usize] == 0{
                    return false;
                }

                let sorted_pos: u32 = sorted_positions[c as usize];
                sorted_positions[c as usize] += 1;

                if (sorted_pos < total_used_syms) == false{
                    return false;
                }

                self.m_sorted_symbol_order[sorted_pos as usize] = i as u16;
            }
        }

        if table_bits <= self.m_min_code_size as u32 {
            table_bits = 0;
        }

        self.m_table_bits = table_bits;
        if table_bits != 0 {
            let table_size: u32 = 1 << table_bits;

            if table_size > self.m_cur_lookup_size {
                self.m_cur_lookup_size = table_size;
                self.m_lookup = alloc::vec![0; table_size as usize];
            }

            for codesize in 1..=table_bits{

                if num_codes[codesize as usize] == 0 { continue; }
                let fillsize = table_bits - codesize;
                let fillnum: u32 = 1 << fillsize;
                let min_code: u32 = min_codes[(codesize - 1) as usize];

                let max_code: u32 = match self.get_unshifted_max_code(codesize){
                    Ok(s) => s,
                    Err(_) => return false
                };

                let val_ptr: u32 = self.m_val_ptrs[(codesize - 1) as usize] as u32;

                for code in min_code..=max_code{
                    let sym_index: u32 = self.m_sorted_symbol_order[(val_ptr + code - min_code) as usize] as u32;

                    if (p_codesizes[sym_index as usize] as u32 == codesize) == false{
                        return false;
                    }

                    for j in 0..fillnum{
                        let t: u32 = j + (code << fillsize);
                        if (t < (1 << table_bits)) == false{
                            return false;
                        }
                        self.m_lookup[t as usize] = sym_index | (codesize << 16);
                    }
                }
            }
        }

        for i in 0..C_MAX_EXPECTED_CODE_SIZE{
            self.m_val_ptrs[i] -= min_codes[i] as i32;
        }

        self.m_table_max_code = 0;
        self.m_decode_start_code_size = self.m_min_code_size as u32;

        if table_bits != 0 {
            let mut i: u32 = table_bits;

            while i >= 1 {
                if num_codes[i as usize] != 0 {
                    self.m_table_max_code = self.m_max_codes[(i - 1) as usize];
                    break;
                }
                i -= 1;
            }

            if i >= 1 {
                self.m_decode_start_code_size = table_bits + 1;
                for j in table_bits+1..=max_code_size{
                    if num_codes[j as usize] != 0 {
                        self.m_decode_start_code_size = j;
                        break;
                    }
                }
            }
        }

        if self.m_table_max_code == 0 {
            self.m_table_max_code = 0;
        }
        // sentinels
        self.m_max_codes[C_MAX_EXPECTED_CODE_SIZE] = u32::MAX;
        self.m_val_ptrs[C_MAX_EXPECTED_CODE_SIZE] = 0xFFFFF;

        self.m_table_shift = 32 - self.m_table_bits;
        true
    }

    #[inline]
    fn get_unshifted_max_code(&mut self, len: u32) -> Result<u32, bool>{
        if (len >= 1 && len <= C_MAX_EXPECTED_CODE_SIZE as u32) == false{
            return Err(false);
        }
        let k: u32 = self.m_max_codes[(len - 1) as usize];
        if k == 0 {
            return Ok(u32::MAX);
        }
        Ok(((k - 1) >> (16 - len)))
    }
}

#[derive(Default)]
pub struct StaticHuffmanDataModel{
    pub m_total_syms: u32,
    pub m_code_sizes: alloc::vec::Vec<u8>,
    pub m_p_decode_tables: DecoderTables
}

impl StaticHuffmanDataModel{
    pub fn clear(&mut self){
        *self = StaticHuffmanDataModel::default();
    }

    // pub fn init(&mut self, total_syms: u32, pCode_sizes: &[u8], mut code_size_limit: u32) -> bool{
    //     code_size_limit = min(code_size_limit, cMaxExpectedCodeSize as u32);
    //     self.m_code_sizes.resize(total_syms as usize, 0 as u8);
    //     let mut min_code_size = u32::MAX;
    //     let mut max_code_size = 0;
    //     for i in 0..total_syms as usize{
    //         let s: u32 = pCode_sizes[i] as u32;
    //         self.m_code_sizes[i] = s as u8;
    //         min_code_size = min(min_code_size, s);
    //         max_code_size = max(max_code_size, s);
    //     }
    //     if max_code_size < (1 as u32) || max_code_size > (32 as u32) || min_code_size > code_size_limit {
    //         return false;
    //     }
    //     if max_code_size > code_size_limit {
    //         return  false;
    //     }
    //     let table_bits = self.compute_decoder_table_bits();
    //     if self.m_pDecode_tables.init(self.m_total_syms, &self.m_code_sizes, table_bits) == false {a
    //         return false;
    //     }
    //     return true;
    // }

    pub fn compute_decoder_table_bits(&mut self) -> u32{
        let mut decoder_table_bits: u32 = 0;
        if self.m_total_syms > 16 {
            decoder_table_bits = min(1 + ceil_log2i(self.m_total_syms), C_MAX_TABLE_BITS as u32);
        }
        decoder_table_bits
    }

    pub fn prepare_decoder_tables(&mut self) -> bool{
        let total_syms = self.m_code_sizes.len();
        if (total_syms >= 1 && total_syms as u32 <= C_MAX_SUPPORTED_SYMS as u32) == false{
            return false;
        }
        self.m_total_syms = total_syms as u32;
        let table_bits = self.compute_decoder_table_bits();
        self.m_p_decode_tables.init(self.m_total_syms, &self.m_code_sizes, table_bits)
    }
}

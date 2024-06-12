use super::crn_consts::*;
use super::crn_utils::*;
use core::cmp::{max, min};
extern crate alloc;

#[derive(Default)]
pub struct DecoderTables {
    pub num_syms: u32,
    pub total_used_syms: u32,
    pub table_bits: u32,
    pub table_shift: u32,
    pub table_max_code: u32,
    pub decode_start_code_size: u32,

    pub min_code_size: u8,
    pub max_code_size: u8,

    pub max_codes: [u32; MAX_EXPECTED_CODE_SIZE + 1],
    pub val_ptrs: [i32; MAX_EXPECTED_CODE_SIZE + 1],

    pub cur_lookup_size: u32,
    pub lookup: alloc::vec::Vec<u32>,

    pub cur_sorted_symbol_order_size: u32,
    pub sorted_symbol_order: alloc::vec::Vec<u16>,
}

impl DecoderTables {
    pub fn init(&mut self, num_syms: u32, p_codesizes: &[u8], mut table_bits: u32) -> bool {
        let mut min_codes = [0_u32; MAX_EXPECTED_CODE_SIZE];

        if num_syms == 0_u32 || table_bits > MAX_TABLE_BITS as u32 {
            return false;
        }
        self.num_syms = num_syms;
        let mut num_codes = [0_u32; (MAX_EXPECTED_CODE_SIZE + 1)];

        for &c in p_codesizes.iter().take(num_syms as usize) {
            if c != 0 {
                num_codes[c as usize] += 1;
            }
        }

        let mut sorted_positions = [0_u32; (MAX_EXPECTED_CODE_SIZE + 1)];
        let mut cur_code: u32 = 0;
        let mut total_used_syms: u32 = 0;
        let mut max_code_size: u32 = 0;
        let mut min_code_size: u32 = u32::MAX;

        for i in 1..=MAX_EXPECTED_CODE_SIZE {
            let n = num_codes[i];

            if n == 0 {
                self.max_codes[i - 1] = 0;
            } else {
                min_code_size = min(min_code_size, i as u32);
                max_code_size = max(max_code_size, i as u32);

                min_codes[i - 1] = cur_code;

                self.max_codes[i - 1] = cur_code + n - 1;
                self.max_codes[i - 1] =
                    1 + ((self.max_codes[i - 1] << (16 - i)) | ((1 << (16 - i)) - 1));
                self.val_ptrs[i - 1] = total_used_syms as i32;
                sorted_positions[i] = total_used_syms;

                cur_code += n;
                total_used_syms += n;
            }
            cur_code <<= 1;
        }

        self.total_used_syms = total_used_syms;
        if total_used_syms > self.cur_sorted_symbol_order_size {
            self.cur_sorted_symbol_order_size = total_used_syms;

            if !total_used_syms.is_power_of_two() {
                self.cur_sorted_symbol_order_size =
                    min(num_syms, total_used_syms.next_power_of_two());
            }

            self.sorted_symbol_order = alloc::vec![0; self.cur_sorted_symbol_order_size as usize];
        }

        self.min_code_size = min_code_size as u8;
        self.max_code_size = max_code_size as u8;

        for (i, &code_size) in p_codesizes.iter().enumerate().take(num_syms as usize) {
            let c: u32 = code_size as u32;

            if c != 0 {
                let code_index = c as usize;

                if num_codes[code_index] == 0 {
                    return false;
                }

                let sorted_pos: u32 = sorted_positions[code_index];
                sorted_positions[code_index] += 1;

                if sorted_pos >= total_used_syms {
                    return false;
                }

                self.sorted_symbol_order[sorted_pos as usize] = i as u16;
            }
        }

        if table_bits <= self.min_code_size as u32 {
            table_bits = 0;
        }

        self.table_bits = table_bits;
        if table_bits != 0 {
            let table_size: u32 = 1 << table_bits;

            if table_size > self.cur_lookup_size {
                self.cur_lookup_size = table_size;
                self.lookup = alloc::vec![0; table_size as usize];
            }

            for codesize in 1..=table_bits {
                if num_codes[codesize as usize] == 0 {
                    continue;
                }
                let fillsize = table_bits - codesize;
                let fillnum: u32 = 1 << fillsize;
                let min_code: u32 = min_codes[(codesize - 1) as usize];

                let max_code: u32 = match self.get_unshifted_max_code(codesize) {
                    Ok(s) => s,
                    Err(_) => return false,
                };

                let val_ptr: u32 = self.val_ptrs[(codesize - 1) as usize] as u32;

                for code in min_code..=max_code {
                    let sym_index: u32 =
                        self.sorted_symbol_order[(val_ptr + code - min_code) as usize] as u32;

                    if p_codesizes[sym_index as usize] as u32 != codesize {
                        return false;
                    }

                    for j in 0..fillnum {
                        let t: u32 = j + (code << fillsize);
                        if t >= (1 << table_bits) {
                            return false;
                        }
                        self.lookup[t as usize] = sym_index | (codesize << 16);
                    }
                }
            }
        }

        for (val_ptr, &min_code) in self.val_ptrs.iter_mut().zip(min_codes.iter()) {
            *val_ptr -= min_code as i32;
        }

        self.table_max_code = 0;
        self.decode_start_code_size = self.min_code_size as u32;

        if table_bits != 0 {
            let mut i: u32 = table_bits;

            while i >= 1 {
                if num_codes[i as usize] != 0 {
                    self.table_max_code = self.max_codes[(i - 1) as usize];
                    break;
                }
                i -= 1;
            }

            if i >= 1 {
                self.decode_start_code_size = table_bits + 1;
                for j in table_bits + 1..=max_code_size {
                    if num_codes[j as usize] != 0 {
                        self.decode_start_code_size = j;
                        break;
                    }
                }
            }
        }

        if self.table_max_code == 0 {
            self.table_max_code = 0;
        }
        // sentinels
        self.max_codes[MAX_EXPECTED_CODE_SIZE] = u32::MAX;
        self.val_ptrs[MAX_EXPECTED_CODE_SIZE] = 0xFFFFF;

        self.table_shift = 32 - self.table_bits;
        true
    }

    #[inline]
    fn get_unshifted_max_code(&mut self, len: u32) -> Result<u32, bool> {
        if !(len >= 1 && len <= MAX_EXPECTED_CODE_SIZE as u32) {
            return Err(false);
        }
        let k: u32 = self.max_codes[(len - 1) as usize];
        if k == 0 {
            return Ok(u32::MAX);
        }
        Ok((k - 1) >> (16 - len))
    }
}

#[derive(Default)]
pub struct StaticHuffmanDataModel {
    pub total_syms: u32,
    pub code_sizes: alloc::vec::Vec<u8>,
    pub p_decode_tables: DecoderTables,
}

impl StaticHuffmanDataModel {
    pub fn clear(&mut self) {
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

    pub fn compute_decoder_table_bits(&mut self) -> u32 {
        let mut decoder_table_bits: u32 = 0;
        if self.total_syms > 16 {
            decoder_table_bits = min(1 + ceil_log2i(self.total_syms), MAX_TABLE_BITS as u32);
        }
        decoder_table_bits
    }

    pub fn prepare_decoder_tables(&mut self) -> bool {
        let total_syms = self.code_sizes.len();
        if !(total_syms >= 1 && total_syms as u32 <= MAX_SUPPORTED_SYMS as u32) {
            return false;
        }
        self.total_syms = total_syms as u32;
        let table_bits = self.compute_decoder_table_bits();
        self.p_decode_tables
            .init(self.total_syms, &self.code_sizes, table_bits)
    }
}

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;

#[pymodule]
fn texture2ddecoder_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // atc
    m.add_function(wrap_pyfunction!(decode_atc_rgb4, m)?)?;
    m.add_function(wrap_pyfunction!(decode_atc_rgba8, m)?)?;
    // astc
    m.add_function(wrap_pyfunction!(decode_astc_4_4, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_5_4, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_5_5, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_6_5, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_6_6, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_8_5, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_8_6, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_8_8, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_10_5, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_10_6, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_10_8, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_10_10, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_12_10, m)?)?;
    m.add_function(wrap_pyfunction!(decode_astc_12_12, m)?)?;
    // bcn
    m.add_function(wrap_pyfunction!(decode_bc1, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bc2, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bc3, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bc4, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bc5, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bc6_signed, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bc6_unsigned, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bc7, m)?)?;
    // etc
    m.add_function(wrap_pyfunction!(decode_etc1, m)?)?;
    m.add_function(wrap_pyfunction!(decode_etc2_rgb, m)?)?;
    m.add_function(wrap_pyfunction!(decode_etc2_rgba1, m)?)?;
    m.add_function(wrap_pyfunction!(decode_etc2_rgba8, m)?)?;
    m.add_function(wrap_pyfunction!(decode_eacr, m)?)?;
    m.add_function(wrap_pyfunction!(decode_eacr_signed, m)?)?;
    m.add_function(wrap_pyfunction!(decode_eacrg, m)?)?;
    // pvrtc
    m.add_function(wrap_pyfunction!(decode_pvrtc_2bpp, m)?)?;
    m.add_function(wrap_pyfunction!(decode_pvrtc_4bpp, m)?)?;
    // crunch
    m.add_function(wrap_pyfunction!(decode_crunch, m)?)?;
    m.add_function(wrap_pyfunction!(decode_unity_crunch, m)?)?;
    Ok(())
}

macro_rules! pybind {
    ($name:expr) => {
        paste::item! {
            #[pyfunction]
            pub fn $name<'a>(py: Python<'a>, data: &'a PyBytes, width: usize, height: usize) -> PyResult<&'a PyBytes> {
                PyBytes::new_with(py, width * height * 4, |image: & mut[u8]|{
                    let image_u32 = unsafe { std::mem::transmute(image) };
                    texture2ddecoder::$name(data.as_bytes(), width, height, image_u32).unwrap_err();
                    Ok(())
                })
            }
        }
    };
}

// atc
pybind!(decode_atc_rgb4);
pybind!(decode_atc_rgba8);
// astc
pybind!(decode_astc_4_4);
pybind!(decode_astc_5_4);
pybind!(decode_astc_5_5);
pybind!(decode_astc_6_5);
pybind!(decode_astc_6_6);
pybind!(decode_astc_8_5);
pybind!(decode_astc_8_6);
pybind!(decode_astc_8_8);
pybind!(decode_astc_10_5);
pybind!(decode_astc_10_6);
pybind!(decode_astc_10_8);
pybind!(decode_astc_10_10);
pybind!(decode_astc_12_10);
pybind!(decode_astc_12_12);
// bcn
pybind!(decode_bc1);
pybind!(decode_bc2);
pybind!(decode_bc3);
pybind!(decode_bc4);
pybind!(decode_bc5);
pybind!(decode_bc6_signed);
pybind!(decode_bc6_unsigned);
pybind!(decode_bc7);
// etc
pybind!(decode_etc1);
pybind!(decode_etc2_rgb);
pybind!(decode_etc2_rgba1);
pybind!(decode_etc2_rgba8);
pybind!(decode_eacr);
pybind!(decode_eacr_signed);
pybind!(decode_eacrg);
// pvrtc
pybind!(decode_pvrtc_2bpp);
pybind!(decode_pvrtc_4bpp);
// crunch
pybind!(decode_crunch);
pybind!(decode_unity_crunch);

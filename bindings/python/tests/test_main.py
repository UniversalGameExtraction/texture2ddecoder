from __future__ import annotations
import os
from time import time_ns

from PIL import Image

from texture2ddecoder_rs import *


def test_ATC_RGB():
    _test_format("ATC_RGB", "dds", decode_atc_rgb4)


def test_ATC_RGBA_Explicit():
    _test_format("ATC_RGBA_Explicit", "dds", decode_atc_rgba8)


def test_ATC_RGBA_Interpolated():
    _test_format("ATC_RGBA_Interpolated", "dds", decode_atc_rgba8)


def test_ASTC_4x4():
    _test_format("ASTC_4x4", "ktx2", decode_astc_4_4)


def test_ASTC_5x4():
    _test_format("ASTC_5x4", "ktx2", decode_astc_5_4)


def test_ASTC_5x5():
    _test_format("ASTC_5x5", "ktx2", decode_astc_5_5)


def test_ASTC_6x5():
    _test_format("ASTC_6x5", "ktx2", decode_astc_6_5)


def test_ASTC_6x6():
    _test_format("ASTC_6x6", "ktx2", decode_astc_6_6)


def test_ASTC_8x5():
    _test_format("ASTC_8x5", "ktx2", decode_astc_8_5)


def test_ASTC_8x6():
    _test_format("ASTC_8x6", "ktx2", decode_astc_8_6)


def test_ASTC_8x8():
    _test_format("ASTC_8x8", "ktx2", decode_astc_8_8)


def test_BC1():
    _test_format("BC1", "ktx2", decode_bc1)


def test_BC1A():
    _test_format("BC1A", "ktx2", decode_bc1a)


def test_BC2():
    _test_format("BC1", "ktx2", decode_bc2)


def test_BC3():
    _test_format("BC3", "ktx2", decode_bc3)


def test_BC4():
    _test_format("BC4", "ktx2", decode_bc4)


def test_BC5():
    _test_format("BC5", "ktx2", decode_bc5)


def test_BC6H():
    _test_format("BC6H", "ktx2", decode_bc6_unsigned)


def test_BC7():
    _test_format("BC7", "ktx2", decode_bc7)


def test_ETC1_RGB():
    _test_format("ETC1_RGB", "ktx2", decode_etc1)


def test_ETC2_RGB():
    _test_format("ETC2_RGB", "ktx2", decode_etc2_rgb)


def test_ETC2_RGBA():
    _test_format("ETC2_RGBA", "ktx2", decode_etc2_rgba8)


def test_ETC2_RGB_A1():
    _test_format("ETC2_RGB_A1", "ktx2", decode_etc2_rgba1)


def test_PVRTCI_2bpp_RGB():
    _test_format("PVRTCI_2bpp_RGB", "ktx2", decode_pvrtc_2bpp)


def test_PVRTCI_2bpp_RGBA():
    _test_format("PVRTCI_2bpp_RGBA", "ktx2", decode_pvrtc_2bpp)


def test_PVRTCI_4bpp_RGB():
    _test_format("PVRTCI_4bpp_RGB", "ktx2", decode_pvrtc_4bpp)


def test_PVRTCI_4bpp_RGBA():
    _test_format("PVRTCI_4bpp_RGBA", "ktx2", decode_pvrtc_4bpp)


def test_EAC_R11():
    _test_format("EAC_R11", "ktx2", decode_eacr)


def test_EAC_RG11():
    _test_format("EAC_RG11", "ktx2", decode_eacrg)


def test_CRUNCH_DXT1():
    _test_format("CRUNCH_DXT1", "crn", decode_crunch)


def test_CRUNCH_DXT5():
    _test_format("CRUNCH_DXT5", "crn", decode_crunch)


def test_CRUNCH_DXT5A():
    _test_format("CRUNCH_DXT5A", "crn", decode_crunch)


def test_CRUNCH_DXN():
    _test_format("CRUNCH_DXN", "crn", decode_crunch)


def test_UNITYCRUNCH_DXT1():
    _test_format("UNITYCRUNCH_DXT1", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_DXT5():
    _test_format("UNITYCRUNCH_DXT5", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_DXT5A():
    _test_format("UNITYCRUNCH_DXT5A", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_DXN():
    _test_format("UNITYCRUNCH_DXN", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_ETC1():
    _test_format("UNITYCRUNCH_ETC1", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_ETC1S():
    _test_format("UNITYCRUNCH_ETC1S", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_ETC2():
    _test_format("UNITYCRUNCH_ETC2", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_ETC2A():
    _test_format("UNITYCRUNCH_ETC2A", "crn", decode_unity_crunch)


def test_UNITYCRUNCH_ETC2AS():
    _test_format("UNITYCRUNCH_ETC2AS", "crn", decode_unity_crunch)


class Texture:
    width: int
    height: int
    data: bytes

    def __init__(self, width: int, height: int, data: bytes) -> None:
        self.width = width
        self.height = height
        self.data = data

    @classmethod
    def from_file(cls, fp: str) -> Texture:
        extension = os.path.splitext(fp)[1][1:].lower()
        data = open(fp, "rb").read()

        if extension == "ktx2":
            return cls.from_ktx2(data)
        elif extension == "dds":
            return cls.from_dds(data)
        elif extension == "crn":
            return cls.from_crn(data)
        else:
            raise ("Unsupported file format")

    @classmethod
    def from_ktx2(cls, data: bytes) -> Texture:
        assert data[:12] == b"\xabKTX 20\xbb\r\n\x1a\n"
        width = int.from_bytes(data[20:24], "little")
        height = int.from_bytes(data[24:28], "little")
        level_0_offset = int.from_bytes(data[80:88], "little")
        level_0_size = int.from_bytes(data[88:96], "little")
        print(width, height, level_0_offset, level_0_size)
        tex_data = data[level_0_offset : level_0_offset + level_0_size]
        return Texture(width, height, tex_data)

    @classmethod
    def from_dds(cls, data: bytes) -> Texture:
        assert data[:4] == b"DDS "
        height = int.from_bytes(data[12:16], "little")
        width = int.from_bytes(data[16:20], "little")
        offset = int.from_bytes(data[4:8], "little") + 4
        fourcc = int.from_bytes(data[84:88], "little", signed=False)
        block_bytes = 8 if fourcc == 0x31545844 else 16

        tex_data = data[offset : offset + (width * height * block_bytes) // 16]
        return Texture(width, height, tex_data)

    @classmethod
    def from_crn(cls, data: bytes) -> Texture:
        width = int.from_bytes(data[12:14], "big")
        height = int.from_bytes(data[14:16], "big")
        return Texture(width, height, data)

    def _decode(self, decode_func: callable) -> bytes:
        start = time_ns()
        ret = decode_func(self.data, self.width, self.height)
        duration = time_ns() - start
        print("Time elapsed in decoding is: ", duration)
        return ret

    def save_as_image(self, path: str, decode_func: callable):
        image_data = self._decode(decode_func)
        _image = Image.frombuffer(
            "RGBA", (self.width, self.height), image_data, "raw", "BGRA"
        )
        # image.save(path)


def get_texture_fp(name: str) -> str:
    return os.path.abspath(
        os.path.join(
            os.path.realpath(__file__),
            "..",
            "..",
            "..",
            "..",
            "resources",
            "tests",
            "textures",
            name,
        )
    )


def get_image_fp(name: str) -> str:
    return os.path.join(
        os.path.realpath(__file__),
        "..",
        "..",
        "..",
        "..",
        "resources",
        "tests",
        "decompressed",
        name,
    )


def _test_format(name: str, sample_extension: str, decode_func: callable):
    print("Testing ", name)
    src_fp = get_texture_fp(f"{name}.{sample_extension}")
    dst_fp = get_image_fp(f"{name}.png")

    texture = Texture.from_file(src_fp)
    texture.save_as_image(dst_fp, decode_func)


if __name__ == "__main__":
    for key, val in list(locals().items()):
        if key.startswith("test_") and callable(val):
            try:
                val()
            except Exception as e:
                print("Error in ", key)
                raise e

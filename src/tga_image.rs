use std::fs::File;
use std::io::Write;
use std::{mem, slice};
use crate::rgb_image::{RGBImage, RGBColor};

impl RGBImage {
    pub fn write_tga(&self, path: String) {
        #[repr(C, packed)]
        #[derive(Default)]
        struct TGAHeader {
            id_length: u8,
            color_map_type: u8,
            data_type_code: u8,
            color_map_origin: u16,
            color_map_length: u16,
            color_map_depth: u8,
            x_origin: u16,
            y_origin: u16,
            width: u16,
            height: u16,
            bits_per_pixel: u8,
            image_descriptor: u8,
        }
        let header = TGAHeader {
            data_type_code: 2,
            width: self.width,
            height: self.height,
            bits_per_pixel: 24,
            image_descriptor: 0x20,
            ..TGAHeader::default()
        };

        println!("Writing image to {}", path);
        let mut output_file = File::create(path).expect("Can't create file");

        unsafe {
            output_file.write_all(struct_as_bytes(&header)).expect("Can't write header");

            let p_data: *const u8 = mem::transmute(&self.pixels[0]);
            let data = slice::from_raw_parts(
                p_data, mem::size_of::<RGBColor>() * self.pixels.len(),
            );
            output_file.write_all(data).expect("Can't write pixels data");
        }

        #[repr(C, packed)]
        struct TGAFooter {
            developer_area_ref: [u8; 4],
            extension_area_ref: [u8; 4],
            footer: [u8; 18]
        }

        let footer = TGAFooter {
            developer_area_ref: [0, 0, 0, 0],
            extension_area_ref: [0, 0, 0, 0],
            footer: [
                b'T', b'R', b'U', b'E', b'V', b'I', b'S', b'I', b'O', b'N', b'-',
                b'X', b'F', b'I', b'L', b'E', b'.', 0
            ]
        };
        unsafe {
            output_file.write_all(struct_as_bytes(&footer)).expect("Can't write footer");
        }
    }
}

unsafe fn struct_as_bytes<T>(t: &T) -> &[u8] {
    let ptr: *const u8 = mem::transmute(t);
    slice::from_raw_parts(
        ptr, mem::size_of::<T>(),
    )
}
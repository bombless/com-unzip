
#![allow(non_snake_case)]

use com::sys::HRESULT;
use libc::*;
use com::interfaces::IUnknown;
use com::sys::NOERROR;
use std::ffi::CStr;
use std::fs::File;
use std::io::{Read, Write};
use com::sys::IID;

com::interfaces! {

    #[uuid("F5353C58-CFD9-4204-8D92-D274C7578B53")]
    pub unsafe interface IUnzip: IUnknown {
        fn Unzip(&self, src_path: *const c_char, dest_path: *const c_char) -> HRESULT;
    }
}


com::class! {
    pub class Unzip: IUnzip {
    }
    
    impl IUnzip for Unzip {
        fn Unzip(&self, src_path: *const c_char, dest_path: *const c_char) -> HRESULT {
            let src_file_path = unsafe { CStr::from_ptr(src_path).to_string_lossy().into_owned() };
            let dest_file_path = unsafe { CStr::from_ptr(dest_path).to_string_lossy().into_owned() };

            // 打开源文件
            let mut src_file = match File::open(&src_file_path) {
                Ok(file) => file,
                Err(_) => return -1, // 文件打开失败
            };

            // 创建目标文件
            let mut dest_file = match File::create(&dest_file_path) {
                Ok(file) => file,
                Err(_) => return -3, // 目标文件创建失败
            };

            // 将解压后的文件内容写入目标文件
            let mut in_buffer = Vec::new();
            if src_file.read_to_end(&mut in_buffer).is_err() {
                return -4; // 文件读取失败
            }

            let mut out_buffer = Vec::new();

            decode(&in_buffer, &mut out_buffer);

            if dest_file.write_all(&out_buffer).is_err() {
                return -5; // 文件写入失败
            }
            NOERROR
        }
    }

}

fn decode(data: &[u8], buf: &mut Vec<u8>) -> usize {
    use flate2::read::GzDecoder;

    let mut decoder = GzDecoder::new(data);
    
    decoder.read_to_end(buf).unwrap()
}

pub const CLSID_CLASS: IID = IID {
    data1: 0xC5F45CBC,
    data2: 0x4439,
    data3: 0x418C,
    data4: [0xA9, 0xF9, 0x05, 0xAC, 0x67, 0x52, 0x5E, 0x43],
};

com::inproc_dll_module![(CLSID_CLASS, Unzip),];

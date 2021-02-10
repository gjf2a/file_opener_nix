use nix::fcntl::{open,OFlag};
use nix::unistd::read;
use nix::sys::stat::Mode;
use std::ffi::CString;

fn main() {
    for arg in std::env::args().skip(1) {
        println!("Contents of {}", arg);
        let fs_str = CString::new(arg.as_str()).unwrap();
        let file_descriptor = open(fs_str.as_c_str(), OFlag::O_RDONLY,
                                   Mode::empty()).unwrap();
        let mut bytes = [0; 256];
        loop {
            let bytes_read = read(file_descriptor, &mut bytes).unwrap();
            if bytes_read == 0 {
                break;
            } else {
                let string_bytes = std::str::from_utf8(&bytes).unwrap();
                print!("{}", string_bytes);
            }
        }
        println!();
    }
}

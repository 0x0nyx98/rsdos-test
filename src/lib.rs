#![no_std]
use core::arch::asm;

#[no_mangle]
fn main() {
    exit(run());
}

fn run() -> u8 {
    let a = 3;
    let b = 4;

    output_string("rust on dos$");

    a * b
}

fn output_string(s: &str) {
    unsafe {
        asm!(
            "mov ah, 0x09",
            "int 0x21",
            in("edx") s.as_ptr()
        );
    }
}


fn exit(code: u8) {
    unsafe {
        asm!(
            "mov ah, 0x4C",
            "int 0x21",
            in("al") code
        );
    }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        asm!(
            "mov ah, 0x4C",
            "int 0x21"
        );
    }
    loop {}
}
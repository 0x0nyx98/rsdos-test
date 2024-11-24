#![no_std]
use core::arch::asm;

#[no_mangle]
fn main() {
    //output_character(':');
    //output_character('3');
    exit();
}

fn output_character(c: char) {
    unsafe {
        asm!(
            "mov ah, 6",
            "int 0x21",
            in("dl") c as u8
        );
    }
}


fn exit() {
    unsafe {
        asm!(
            "mov ah, 0",
            "int 0x21"
        );
    }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        asm!(
            "int 0x20"
        );
    }
    loop {}
}
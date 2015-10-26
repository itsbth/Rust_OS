#![feature(no_std, lang_items)]
#![feature(const_fn, unique, core_str_ext, asm)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

unsafe fn inb(port: u16) -> u8 {
    let mut ret: u8;
    asm!("inb $1,$0" : "={ax}" (ret) : "{dx}" (port) :: "volatile");
    ret
}

#[no_mangle]
pub extern fn rust_main() {
    use core::fmt::Write;
    vga_buffer::clear_screen();
    vga_buffer::WRITER.lock().write_str("Hello, World!\n");
    println!("Some numbers: {} {}", 42, 13.37);
    println!("KBSTATP: {}", unsafe {inb(0x64)});
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_ftm() -> ! {loop{}}


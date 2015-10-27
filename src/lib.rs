#![feature(no_std, lang_items)]
#![feature(const_fn, unique, core_str_ext, asm)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("inb $1,$0" : "={ax}" (ret) : "{dx}" (port) :: "volatile");
    ret
}

unsafe fn outb(port: u16, val: u8) {
    asm!("outb $1,$0" :: "{dx}"(port), "r"(val) :: "volatile");
}

fn get_cmos(register: u8) -> u8 {
    let mut ret: u8 = 0;
    unsafe {
        outb(0x70, register);
        loop {
            let nv = inb(0x71);
            if nv == ret {
                break
            }
            ret = nv
        }
    }
    ret
}

unsafe fn read_tsc() -> u64 {
    let mut low: u32;
    let mut high: u32;
    asm!("rdtsc" : "={eax}"(low), "={edx}"(high));
    (high as u64) << 32 | (low as u64)
}

fn decode_bcd(bcd: u8) -> u8 {
    ((bcd & 0xF0) >> 1) + ((bcd & 0xF0) >> 3) + (bcd & 0xF)
}

unsafe fn rdrand() -> u64 {
    let mut ret: u64;
    asm!("rdrand $0" : "=r"(ret));
    ret
}

#[no_mangle]
pub extern fn rust_main() {
    use core::fmt::Write;
    vga_buffer::clear_screen();
    vga_buffer::WRITER.lock().write_str("Hello, World!\n");
    println!("Some numbers: {} {}", 42, 13.37);
    println!("KBSTATP: {}", unsafe {inb(0x64)});
    // println!("Rand: {}", unsafe{rdrand()});
    unsafe { outb(0x0B, 0b11); }
    let mut old_sec = 0;
    let mut last_tsc = 0;
    loop {
        let sec = get_cmos(0x00);
        if sec != old_sec {
            let tsc = unsafe { read_tsc() };
            println!("{:02}:{:02}:{:02} {} c/s", decode_bcd(get_cmos(0x04)), decode_bcd(get_cmos(0x02)), decode_bcd(sec), tsc - last_tsc);
            old_sec = sec;
            last_tsc = tsc;
        }
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt(fmt: core::fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::White, vga_buffer::Color::Red);
    print!("PANIC: ");
    vga_buffer::WRITER.lock().write_fmt(fmt);
    loop{}
}


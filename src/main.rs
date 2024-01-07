#![no_std]
#![no_main]

//3F20 0008 fsel2 1 << 3 turn pin21 into an output
//3f20 001c gpiol_set 1 << 21 turns pin 21 on
//3f20_0028 gpiol clear 1 << 21 turns pin 21 off

const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;

const GPIO_SET0: u32 = 0x3F20_001C;
const GPIO_CLR0: u32 = 0x3F20_0028;

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

struct GPIO;

impl GPIO {
    pub fn set_output(pin: u32) {
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Something has gone terribly wrong"),
        };

        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }

        // create mask
        let mut mask: u32 = 0b111;

        // shift the mask to the right location
        let pinnum = pin % 10;
        
        mask = mask << pinnum * 3;

        // and in the NOT of the mask
        val = val & !(mask);

        // set OUR value
        val |= 1 << pinnum * 3;

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val)
        }
    }

    pub fn set(pin: u32) {
        let bitpos = pin;

        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(GPIO_SET0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_SET0 as *mut u32, val)
        }
    }

    pub fn clear(pin: u32) {
        let bitpos = pin;

        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(GPIO_CLR0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val)
        }
    }
}

#[link_section=".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    // turn PIN21 into an output
    GPIO::set_output(21);

    loop {
        // Turn Pin On
        GPIO::set(21);

        for _ in 1..50000 {
            unsafe { asm!("nop"); }
        }

        // Turn Pin Off
        GPIO::clear(21);

        for _ in 1..50000 {
            unsafe { asm!("nop"); }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
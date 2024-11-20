#![no_std]
#![no_main]

use d1_pac::Peripherals;
use panic_halt as _;
use riscv_rt::entry;

pub struct Uart {
    regs: d1_pac::UART1,
}

impl Uart {
    pub fn new(regs: d1_pac::UART1) -> Self {
        regs.lcr().write(|w| {
            w.dls().eight();
            w.pen().disabled();
            w.stop().one();
            w.dlab().set_bit()
        });
        regs.dll().write(|w| unsafe { w.dll().bits(13) });
        regs.dlh().write(|w| unsafe { w.dlh().bits(0) });

        regs.lcr().modify(|_, w| w.dlab().rx_buffer());
        Self { regs }
    }

    pub fn write_char(&mut self, c: char) {
        self.regs.thr().write(|w| unsafe { w.thr().bits(c as u8) });
        while !self.regs.lsr().read().thre().bit_is_set() {}
    }
}

#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };

    p.CCU.uart_bgr().write(|w| {
        w.uart1_rst().deassert();
        w.uart1_gating().pass()
    });

    p.GPIO.pg_cfg0().write(|w| w.pg6_select().uart1_tx());
    p.GPIO.pg_drv0().write(|w| w.pg6_drv().l3());
    p.GPIO.pg_pull0().write(|w| w.pg6_pull().pull_disable());

    let mut uart = Uart::new(p.UART1);

    loop {
        for c in "Hello world!\n".chars() {
            uart.write_char(c);
        }
        riscv::asm::delay(24_000_000);
    }
}

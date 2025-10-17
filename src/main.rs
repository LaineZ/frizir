#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let mut x_step = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
    let mut x_dir = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);

    x_dir.set_high(); 
    
    loop {
        x_step.set_high();
        delay.delay_us(1000_u16);
        x_step.set_low();
        delay.delay_us(800_u16);
    }
}

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;
use defmt_rtt as _;
use defmt::println;
use nb::block;
use stm32f3xx_hal::flash::FlashExt;
use stm32f3xx_hal::gpio::{GpioExt, Gpiod, Gpioe, Input, Output, Pin, PushPull, U};
use stm32f3xx_hal::gpio::marker::Gpio;
use stm32f3xx_hal::{pac, timer};
use stm32f3xx_hal::prelude::{_embedded_hal_digital_InputPin, _embedded_hal_digital_OutputPin, _embedded_hal_timer_CountDown, _stm32f3xx_hal_time_rate_Extensions, _stm32f3xx_hal_time_time_Extensions};
use stm32f3xx_hal::rcc::RccExt;

fn set_bit<P: Gpio,const N: u8>(value:u8, bit_pos:u8, pin: &mut Pin<P,U<N>,Output<PushPull>>) {
    let is_set:bool = ((value >> bit_pos) & 0b0000_0001) == 1;
    if is_set {
        pin.set_high().unwrap();
    } else {
        pin.set_low().unwrap();
    }
}

fn alu_enable(ena: &mut Pin<Gpioe,U<8>,Output<PushPull>>) {
    ena.set_high().unwrap();
    defmt::trace!("ALU enabled");
}

fn alu_disable(ena: &mut Pin<Gpioe,U<8>,Output<PushPull>>) {
    ena.set_low().unwrap();
    defmt::trace!("ALU disabled");
}

fn alu_add(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_low().unwrap();
    op1.set_low().unwrap();
    op2.set_low().unwrap();
    defmt::trace!("ALU OP ADD");
}

fn alu_sub(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_low().unwrap();
    op1.set_low().unwrap();
    op2.set_high().unwrap();
    defmt::trace!("ALU OP SUB");
}

fn alu_rsh(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_low().unwrap();
    op1.set_high().unwrap();
    op2.set_high().unwrap();
    defmt::trace!("ALU OP LSH");
}

fn alu_lsh(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_low().unwrap();
    op1.set_high().unwrap();
    op2.set_low().unwrap();
    defmt::trace!("ALU OP RSH");
}

fn alu_xor(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_high().unwrap();
    op1.set_low().unwrap();
    op2.set_low().unwrap();
    defmt::trace!("ALU OP XOR");
}

fn alu_not(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_high().unwrap();
    op1.set_low().unwrap();
    op2.set_high().unwrap();
    defmt::trace!("ALU OP NOT");
}

fn alu_and(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_high().unwrap();
    op1.set_high().unwrap();
    op2.set_low().unwrap();
    defmt::trace!("ALU OP AND");
}

fn alu_or(op0: &mut Pin<Gpioe,U<9>,Output<PushPull>>, op1: &mut Pin<Gpioe,U<10>,Output<PushPull>>, op2: &mut Pin<Gpioe,U<11>,Output<PushPull>>) {
    op0.set_high().unwrap();
    op1.set_high().unwrap();
    op2.set_high().unwrap();
    defmt::trace!("ALU OP OR");
}

fn read_x_in(x0: &Pin<Gpiod,U<0>,Input>,
             x1: &Pin<Gpiod,U<1>,Input>,
             x2: &Pin<Gpiod,U<2>,Input>,
             x3: &Pin<Gpiod,U<3>,Input>,
             x4: &Pin<Gpiod,U<4>,Input>,
             x5: &Pin<Gpiod,U<5>,Input>,
             x6: &Pin<Gpiod,U<6>,Input>,
             x7: &Pin<Gpiod,U<7>,Input>) -> u8 {
    let mut x_in = 0u8;
    x_in |= (x0.is_high().unwrap() as u8)<<0;
    x_in |= (x1.is_high().unwrap() as u8)<<1;
    x_in |= (x2.is_high().unwrap() as u8)<<2;
    x_in |= (x3.is_high().unwrap() as u8)<<3;
    x_in |= (x4.is_high().unwrap() as u8)<<4;
    x_in |= (x5.is_high().unwrap() as u8)<<5;
    x_in |= (x6.is_high().unwrap() as u8)<<6;
    x_in |= (x7.is_high().unwrap() as u8)<<7;

    x_in
}

fn check_x_in(x_in: u8, expected: u8) -> bool {
    if x_in != expected {
        defmt::info!("ERROR: X_IN: {=u8:08b}", x_in);
    }
    x_in == expected
}

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze(&mut flash.acr);

    // Acquire the GPIO peripheral
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb);

    // X_OUT
    let mut x_out_0 = gpiod.pd8.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut x_out_1 = gpiod.pd9.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut x_out_2 = gpiod.pd10.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut x_out_3 = gpiod.pd11.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut x_out_4 = gpiod.pd12.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut x_out_5 = gpiod.pd13.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut x_out_6 = gpiod.pd14.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut x_out_7 = gpiod.pd15.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    // ALU_OP
    let mut alu_ena = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut alu_op0 = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut alu_op1 = gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut alu_op2 = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // X_IN
    let x_in_0 = gpiod.pd0.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let x_in_1 = gpiod.pd1.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let x_in_2 = gpiod.pd2.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let x_in_3 = gpiod.pd3.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let x_in_4 = gpiod.pd4.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let x_in_5 = gpiod.pd5.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let x_in_6 = gpiod.pd6.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);
    let x_in_7 = gpiod.pd7.into_floating_input(&mut gpiod.moder, &mut gpiod.pupdr);

    // Configure the syst timer to trigger an update every second
    let mut timer = timer::Timer::new(dp.TIM2, clocks, &mut rcc.apb1);
    timer.start(5.milliseconds());

    println!("Hello, world!");

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        for out_value in 0u8..=254 {
            set_bit(out_value,7, &mut x_out_7);
            set_bit(out_value,6, &mut x_out_6);
            set_bit(out_value,5, &mut x_out_5);
            set_bit(out_value,4, &mut x_out_4);
            set_bit(out_value,3, &mut x_out_3);
            set_bit(out_value,2, &mut x_out_2);
            set_bit(out_value,1, &mut x_out_1);
            set_bit(out_value,0, &mut x_out_0);

            defmt::info!("X_OUT: {=u8:08b}", out_value);

            let mut alu_pass = true;
            alu_disable(&mut alu_ena);
            alu_add(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_sub(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_rsh(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_lsh(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_xor(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_not(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_and(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_or(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            if alu_pass {
                defmt::info!("ALU disabled check: pass");
            } else {
                panic!("ALU disabled check: fail");
            }


            alu_enable(&mut alu_ena);
            alu_pass = true;
            alu_add(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_sub(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_rsh(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, out_value>>1);
            defmt::trace!("RSH X_IN: {=u8:08b}", x_in);
            alu_lsh(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, out_value<<1);
            defmt::trace!("LSH X_IN: {=u8:08b}", x_in);
            alu_xor(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_not(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, !out_value);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_and(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            alu_or(&mut alu_op0, &mut alu_op1, &mut alu_op2);
            block!(timer.wait()).unwrap();
            let x_in = read_x_in(&x_in_0,&x_in_1,&x_in_2,&x_in_3,&x_in_4,&x_in_5,&x_in_6,&x_in_7);
            alu_pass &= check_x_in(x_in, 0b1111_1111);
            defmt::trace!("X_IN: {=u8:08b}", x_in);
            if alu_pass {
                defmt::info!("ALU enabled check: pass");
            } else {
                panic!("ALU enabled check: fail");
            }
        }
    }
}

#![allow(unused)]
#![allow(dead_code)]

use std::{io::stdin, thread::sleep, time::Duration};

use rust_gpiozero::*;

// function to convert letters and numbers to binary
fn num_from_dec_to_bin(mut num: u8) -> Vec<bool> {
    let mut binary: Vec<bool> = Vec::new();

    loop {
        if num % 2 == 1 && num != 0 {
            binary.push(true);
            num = (num - 1) / 2;
        } else if num % 2 == 0 && num != 0 {
            binary.push(false);
            num = num / 2;
        } else {
            break;
        }
    }

    match binary.len() {
        // This match is used to overwrite the previous numbers written on the register with 0
        8 => binary,

        1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            let x = 8 - binary.len();

            for _i in 1..=x {
                binary.push(false);
            }

            return binary;
        }

        _ => panic!("the number is greater than 255"),
    }
}

// function for updating the register output
fn update_register(
    message: Vec<bool>,
    mut clock_pin: OutputDevice,
    mut data_pin: OutputDevice,
    mut latch_pin: OutputDevice,
) {
    for i in message {
        if i == true {
            data_pin.on();
            clock_pin.on();
            sleep(Duration::from_nanos(20));
            data_pin.off();
            clock_pin.off();
        } else {
            clock_pin.on();
            sleep(Duration::from_nanos(20));
            clock_pin.off();
        }
    }

    latch_pin.on();
    latch_pin.off()
}

fn main() {
    let mut clock = OutputDevice::new(0); // this pin will be used to clock the circuit
    let mut data = OutputDevice::new(5); // this pin will be used to write the bits to the register
    let mut latch = OutputDevice::new(6); // this pin will be used to confirm the written data in the register and put them in output
    let mut reset = OutputDevice::new(21); // this pin will be used to reset the register

    reset.off();
    latch.on();

    sleep(Duration::from_nanos(500));

    latch.off();
    reset.on();

    let mut number = String::new();

    println!("Enter the number to be represented in binary (MAX RANGE 256): ");

    stdin()
        .read_line(&mut number)
        .expect("Error in reading the number, possible range overflow or negative number");

    let mut number: u8 = number.trim().parse().expect("Invalid number");

    let bin = num_from_dec_to_bin(number);

    update_register(bin, clock, data, latch);
}

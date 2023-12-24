# 8-Bit Register Control with Raspberry Pi

## Project Overview
This Rust project has been developed to control an 8-bit register ([SN74HC595N](https://www.mouser.it/ProductDetail/Texas-Instruments/SN74HC595N?qs=IEl3ej0IqwBTHkYa8XPoMQ%3D%3D)) with a [Raspberry Pi](https://www.raspberrypi.com). It allows interfacing with the register to control external devices.

## Dependencies
For the project to work it requires the following items:
  -  [Rust](https://www.rust-lang.org/it "The best programing language ever") (latest version)
  -  [Cargo](https://github.com/rust-lang/cargo) (latest version) 
  -  [rust_gpiozero](https://docs.rs/rust_gpiozero/latest/rust_gpiozero/): used to control the GPIO pins

## How It Works
The program uses the *rust_gpiozero* library to communicate with the 8-bit register (SN74HC595N) and manage pin outputs. The code leverages specific Raspberry Pi features for register control.

## How to Make It Work
To make the project work, follow these steps:
1. Clone the repository to your Raspberry Pi.
2. Install the dependencies by running the command: `git clone https://github.com/Matthiea/8-bit-register.git`.
3. Run the program with the required privileges: `cargo run`.



## ⚠️⚠️ Warning ⚠️⚠️

The program has been calibrated for the voltages of the Raspberry Pi and the SN74HC595N. It is therefore necessary to check the output voltage of your development board. Additionally, if using a different register, it is advisable to refer to the datasheet

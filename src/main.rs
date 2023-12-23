#![allow(unused)]
#![allow(dead_code)]

use std::{io::stdin, thread::sleep, time::Duration};

use rust_gpiozero::*;

// funzione per convertire lettere e numeri in binario
fn num_from_dec_to_bin(mut num: u8) -> Vec<bool> {
    let mut binario: Vec<bool> = Vec::new();

    loop {
        if num % 2 == 1 && num != 0 {
            binario.push(true);
            num = (num - 1) / 2;
        } else if num % 2 == 0 && num != 0 {
            binario.push(false);
            num = num / 2;
        } else {
            break;
        }
    }

    match binario.len() {
        8 => binario,

        1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            let x = 8 - binario.len();

            for _i in 1..=x {
                binario.push(false);
            }

            return binario;
        }

        _ => panic!("il numero è maggiore di 255"),
    }
}

// funzione per l'update dell'output del reggistro
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
    let mut clock = OutputDevice::new(0); // questo pin servirà per dare il clock al circuito
    let mut data = OutputDevice::new(5); // questo pin serivrà per scrivere i bist nel reggistro
    let mut latch = OutputDevice::new(6); // questo pin servirà per confermare i dati scritti nel reggistro e metterli in output
    let mut reset = OutputDevice::new(21); // questo pin servira per resettare il reggistro

    reset.off();
    latch.on();

    sleep(Duration::from_nanos(500));

    latch.off();
    reset.on();

    let mut numero = String::new();

    println!("Inserisci il numero che dovra essere rappresentato in binario (MAX RANGE 256): ");

    stdin().read_line(&mut numero).expect(
        "Errore nella lettura del numero possibile sumeramento del range o numero negativo",
    );

    let mut numero: u8 = numero.trim().parse().expect("Numero non valido");

    let bin = num_from_dec_to_bin(numero);

    update_register(bin, clock, data, latch);
}

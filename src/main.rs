#![deny(warnings)]
extern crate libbeaglebone;
use gpio_cdev::{Chip, LineRequestFlags};
use libbeaglebone::adc::ADC;

fn main() {

    let analog = ADC::new(0, 0.0);

    match analog.read() {
        Ok(value) => println!("Analog: {}", value),
        Err(error) => println!("Failed reading analog value, with: {}", error)
    }

    match Chip::new("/dev/gpiochip0") {
        Ok(mut chip_value) => {
            match chip_value.get_line(4) {
                Ok(line) => {
                    match line.request(LineRequestFlags::INPUT, 0, "read-input") {
                        Ok(line_handle) => {
                            // for _ in 1..4 {
                            println!("Digital: {:?}", line_handle.get_value());
                            // }
                        }
                        Err(err) => println!("Failed to request line with error {}", err)
                    }
                },
                Err(error) => println!("Failed to get line with error {}", error)
            }
        }
        Err(error) => println!("Failed get chip with error {}", error)
    }
}

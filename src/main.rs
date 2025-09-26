#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]  
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut buffer: CircularBuffer<u8, 128> = CircularBuffer::new();
    
    loop {
        // Read all available bytes (non-blocking)
        while let Ok(byte) = serial.read() {
            if let Err(_) = buffer.write(byte) {
                ufmt::uwriteln!(&mut serial, "Buffer full!").unwrap();
            }
        }
        
        // Process all buffered data
        while let Some(received_byte) = buffer.read() {
            ufmt::uwriteln!(&mut serial, "Received: {}", received_byte).unwrap();
            process_uart_byte(received_byte);
        }
    }
}

fn process_uart_byte(byte: u8) {
    match byte {
        0x01 => {
            // Start of message
        },
        0x02 => {
            // End of message  
        },
        _ => {
            // Data byte
        }
    }
}

struct CircularBuffer<T, const N: usize> {
    buffer: [Option<T>; N],
    read_index: usize,
    write_index: usize,
    count: usize,
}

impl<T: Copy + Default, const N: usize> CircularBuffer<T, N> {
    fn new() -> Self {
        CircularBuffer {
            buffer: [None; N],
            read_index: 0,
            write_index: 0,
            count: 0,
        }
    }
    
    fn write(&mut self, value: T) -> Result<(), &'static str> {
        if self.count >= N {
            return Err("Buffer full");
        }
        
        self.buffer[self.write_index] = Some(value);
        self.write_index = (self.write_index + 1) % N;
        self.count += 1;
        Ok(())
    }
    
    fn read(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        
        let value = self.buffer[self.read_index].take();
        self.read_index = (self.read_index + 1) % N;
        self.count -= 1;
        value
    }
    
    fn is_empty(&self) -> bool {
        self.count == 0
    }
    
    fn is_full(&self) -> bool {
        self.count >= N
    }
}

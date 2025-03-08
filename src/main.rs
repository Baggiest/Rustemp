// use std::io::Read;
use std::fs::File;
use std::io::{self, Read, Write};
use std::{thread, time};

fn main() -> std::io::Result<()> {
    let interval = time::Duration::from_millis(500);

    // clear and print a message before starting the loop
    print!("\x1B[2J\x1B[1;1H");
    // println!("Monitoring CPU temperatures (Press Ctrl+C to exit)...");
    
    loop {
        let mut core0 = File::open("/sys/class/thermal/thermal_zone0/temp")?;
        let mut core1 = File::open("/sys/class/thermal/thermal_zone1/temp")?;

        let mut contents_core0 = Vec::new();
        let mut contents_core1 = Vec::new();

        core0.read_to_end(&mut contents_core0)?;
        core1.read_to_end(&mut contents_core1)?;

        let mut temp0: f32 = 0.0;
        let mut temp1: f32 = 0.0;

        match String::from_utf8(contents_core0) {
            Ok(text) => {
                temp0 = text.trim().parse::<f32>().unwrap() / 1000.0;
            }
            Err(e) => eprintln!("Failed to parse contents as UTF-8 string: {:?}", e),
        }

        match String::from_utf8(contents_core1) {
            Ok(text) => {
                temp1 = text.trim().parse::<f32>().unwrap() / 1000.0;
            }
            Err(e) => eprintln!("Failed to parse contents as UTF-8 string: {:?}", e),
        }

        // Use \r to return to the beginning of the line and overwrite previous output
        print!("\rCPU 0: {:.1}° C | CPU 1: {:.1}° C", temp0, temp1);
        io::stdout().flush().unwrap();
        
        thread::sleep(interval);
    }
}

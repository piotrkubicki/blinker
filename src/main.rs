use gpio_cdev::{Chip, LineRequestFlags, LineHandle};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> Result<(), gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let mut blue = Led::new(&mut chip, 17)?;
    let mut green = Led::new(&mut chip, 27)?;
    let mut red = Led::new(&mut chip, 22)?;

    let duration = Duration::from_millis(20000);
    let start_time = Instant::now();

    while start_time.elapsed() < duration {
        sleep(Duration::from_millis(1000));
        blue.on();
        red.off();
        sleep(Duration::from_millis(1000));
        blue.off();
        green.on();
        sleep(Duration::from_millis(1000));
        green.off();
        red.on();
    }

    Ok(())
}

struct Led {
    line: LineHandle,
    state: u8,
}

impl Led {
    fn new(chip: &mut Chip, line: u32) -> Result<Self, gpio_cdev::Error> {
        match chip.get_line(line)?.request(LineRequestFlags::OUTPUT, 0, "blinky") {
            Ok(line) => return Ok(
                Led {
                    line,
                    state: 0,
                }
            ),
            Err(e) => Err(e),
        }
    }

    fn turn(&self) -> Result<(), gpio_cdev::Error>{
        match self.line.set_value(self.state) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn on(&mut self) {
        self.state = 1;
        let _ = self.turn();
    }

    fn off(&mut self) {
        self.state = 0;
        let _ = self.turn();
    }
}

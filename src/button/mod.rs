use rppal::gpio::{Gpio, InputPin, Level};
use std::{error::Error, thread, time::Duration};

pub struct Button {
    pin: InputPin,
    count: u8,
    status: Level,
}

impl Button {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pin: Gpio::new()?.get(pin)?.into_input(),
            count: 0,
            status: Level::High,
        })
    }

    pub fn run(mut self) {
        thread::spawn(move || loop {
            self.observe();
            thread::sleep(Duration::from_millis(10));
        });
    }

    pub fn observe(&mut self) {
        match self.pin.read() {
            Level::Low => {
                if self.status == Level::High {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = Level::Low;
                        self.count = 0;
                        let url = "https://notify-api.line.me/api/notify";
                        ureq::post(url)
                            .set(
                                "Authorization",
                                "Bearer {発行したトークンを記入}",
                            )
                            .send_form(&[
                                ("message", "test notify"),
                                ("stickerPackageId", "1"),
                                ("stickerId", "113"),
                            ])
                            .unwrap();
                    }
                } else {
                    self.count = 0;
                }
            }
            Level::High => {
                if self.status == Level::Low {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = Level::High;
                        self.count = 0;
                    }
                } else {
                    self.count = 0;
                }
            }
        }
    }
}

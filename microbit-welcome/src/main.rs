#![no_std]
#![no_main]

use embassy_executor::{Spawner, main};
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive},
    init,
    pwm::SimplePwm,
};
use embassy_time::Timer;
use panic_halt as _;

const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 5;

struct Display {
    row: [Output<'static>; NUM_ROWS],
    col: [Output<'static>; NUM_COLS],
}

impl Display {
    async fn show(&mut self, display_matrix: [[u8; NUM_COLS]; NUM_ROWS], duration: u8) {
        for _ in 0..duration {
            for (r, d_row) in display_matrix.iter().enumerate() {
                for i in 0..NUM_ROWS {
                    self.row[i].set_low();
                }
                for i in 0..NUM_COLS {
                    self.col[i].set_high();
                }

                self.row[r].set_high();

                for (c, d_col) in d_row.iter().enumerate() {
                    if *d_col == 1 {
                        self.col[c].set_low();
                    }
                }
                Timer::after_millis(1).await;
            }
        }
    }

    fn clear(&mut self) {
        for r in 0..NUM_ROWS {
            self.row[r].set_low();
        }
        for c in 0..NUM_COLS {
            self.col[c].set_high();
        }
    }
}

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Default::default());

    let mut display: Display = Display {
        row: [
            Output::new(peripherals.P0_21, Level::Low, OutputDrive::Standard),
            Output::new(peripherals.P0_22, Level::Low, OutputDrive::Standard),
            Output::new(peripherals.P0_15, Level::Low, OutputDrive::Standard),
            Output::new(peripherals.P0_24, Level::Low, OutputDrive::Standard),
            Output::new(peripherals.P0_19, Level::Low, OutputDrive::Standard),
        ],
        col: [
            Output::new(peripherals.P0_28, Level::High, OutputDrive::Standard),
            Output::new(peripherals.P0_11, Level::High, OutputDrive::Standard),
            Output::new(peripherals.P0_31, Level::High, OutputDrive::Standard),
            Output::new(peripherals.P1_05, Level::High, OutputDrive::Standard),
            Output::new(peripherals.P0_30, Level::High, OutputDrive::Standard),
        ],
    };

    let display_matrix: [[u8; NUM_COLS]; NUM_ROWS] = [
        [0, 1, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    let mut speaker = SimplePwm::new_1ch(peripherals.PWM0, peripherals.P0_00);
    let hello = [262, 330, 294, 349, 392];

    loop {
        display.show(display_matrix, 200).await;
        display.clear();

        for frequency in hello {
            speaker.set_period(frequency * 10);
            speaker.set_duty(0, speaker.max_duty() / 2);
            Timer::after_millis(200).await;
            speaker.set_duty(0, 0);
            Timer::after_millis(50).await;
        }
    }
}

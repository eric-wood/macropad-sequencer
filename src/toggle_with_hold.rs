use core::future::IntoFuture;

use embassy_futures::select::{Either, select};
use embassy_time::{Duration, Timer};

use crate::debounced_button::DebouncedButton;

pub struct ToggleWithHold<'a> {
    button: DebouncedButton<'a>,
    pub is_held: bool,
    pub is_pressed: bool,
    hold_threshold: Duration,
    timer: Option<Timer>,
}

impl<'a> ToggleWithHold<'a> {
    pub fn new(button: DebouncedButton<'a>, hold_threshold: Duration) -> Self {
        ToggleWithHold {
            button,
            is_pressed: false,
            is_held: false,
            hold_threshold,
            timer: None,
        }
    }

    pub async fn on_change(&mut self) {
        let button_future = self.button.on_change();
        let timer_future = if let Some(timer) = self.timer.as_mut() {
            &mut timer.into_future()
        } else {
            &mut Timer::after_secs(31536000) // one year, arbitrarily large
        };

        self.is_pressed = match select(button_future, timer_future).await {
            Either::First(pressed) => {
                self.timer = if pressed {
                    Some(Timer::after(self.hold_threshold))
                } else {
                    None
                };

                if !pressed {
                    self.is_held = false;
                }

                pressed
            }
            Either::Second(_) => {
                self.is_held = true;
                self.timer = None;
                true
            }
        };
    }
}

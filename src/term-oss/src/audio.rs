//! Audio buzzer using OSS.
//!
//! By default, OSS provides `8kHz u8 mono` sound, which is enough for a
//! buzzer. Thus no parameter tuning will be done here.
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;    // Multiple Producer Single Consumer
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn buzz() {
        let b: Buzzer = Default::default();
        b.buzz(true);
        thread::sleep(Duration::from_millis(500));
        b.buzz(false);
        thread::sleep(Duration::from_millis(500));
        for _ in 0..10 {
            b.buzz(true);
            thread::sleep(Duration::from_millis(100));
            b.buzz(false);
            thread::sleep(Duration::from_millis(100));
        }
    }
}

type Sample = u8;
const SAMPLE_RATE: usize = 8000;
const BUFFER_SIZE: usize = 32;
const AMPLITUDE: Sample = 0x20; // amplitude +/- 0x20, don't be too loud
const ZERO_DC: Sample = 0x80;
const ZEROS: [Sample; BUFFER_SIZE] = [ZERO_DC; BUFFER_SIZE];
const HIGHS: [Sample; BUFFER_SIZE] = [ZERO_DC + AMPLITUDE; BUFFER_SIZE];
const LOWS:  [Sample; BUFFER_SIZE] = [ZERO_DC - AMPLITUDE; BUFFER_SIZE];

#[derive(Debug)]
enum Message {
    On,
    Off,
    Quit,
}

pub struct Buzzer {
    thread: Option<thread::JoinHandle<()>>,
    tx: mpsc::Sender<Message>,
    on: bool,
}

impl Buzzer {
    /// Only allow construction from Default trait.
    fn new() -> Buzzer {
        let mut dsp = File::create("/dev/dsp").unwrap();
        let (tx, rx) = mpsc::channel();
        let thread = move || {
            let mut on = false;
            let mut is_high = false;
            'main: loop {
                'recv: loop {
                    match rx.try_recv() {
                        Err(mpsc::TryRecvError::Empty) => break 'recv,
                        Err(mpsc::TryRecvError::Disconnected) => panic!("WTF"),
                        Ok(Message::Quit) => break 'main,
                        Ok(Message::On) => on = true,
                        Ok(Message::Off) => on = false,
                    }
                }

                //println!("{}", on);
                if on {
                    if is_high { dsp.write_all(&HIGHS).unwrap() }
                    else { dsp.write_all(&LOWS).unwrap() }
                    is_high = !is_high;
                }
                else { dsp.write_all(&ZEROS).unwrap() }
                dsp.flush().unwrap();
                thread::sleep(Duration::from_millis((BUFFER_SIZE * 1000 / SAMPLE_RATE) as u64));
            }
        };
        let child = thread::Builder::new()
            .name("Buzzer".to_string())
            .spawn(thread)
            .unwrap();
        Buzzer {
            thread: Some(child),
            tx: tx,
            on: false,
        }
    }

    pub fn buzz(&mut self, on: bool) {
        if self.on == on { return }
        self.on = on;
        let msg = if on { Message::On } else { Message::Off };
        self.tx.send(msg).unwrap();
    }
}

impl Default for Buzzer {
    fn default() -> Buzzer {
        Buzzer::new()
    }
}

impl Drop for Buzzer {
    fn drop(&mut self) {
        self.tx.send(Message::Quit).unwrap();
        self.thread.take().unwrap().join().unwrap();
    }
}


#![windows_subsystem = "windows"]

extern crate clipboard;

use std::f64::INFINITY;

use iced::{
    widget::{button, column, row, text, text_input},
    Element, Padding, Sandbox, Settings,
};

use clipboard::{ClipboardContext, ClipboardProvider};

const TPS: f64 = 192.0;
const ZERO_POINT: i32 = -92544;

fn main() -> iced::Result {
    App::run(Settings::default())
}

pub struct App {
    clipboard: Option<ClipboardContext>,
    attack_table: Vec<i32>,
    decay_table: Vec<i32>,
    sustain_table: Vec<i32>,
    attack: u8,
    attack_f: f32,
    attack_input: String,
    attack_result: f64,
    decay: u8,
    decay_f: f32,
    decay_input: String,
    decay_result: f64,
    sustain: u8,
    sustain_f: f32,
    sustain_input: String,
    sustain_result: f64,
    release: u8,
    release_f: f32,
    release_input: String,
    release_result: f64,
    result: String,
    to_nds: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    CalculatePressed,
    AttackChanged(String),
    DecayChanged(String),
    SustainChanged(String),
    ReleaseChanged(String),
    CopyToClipboard,
    PasteFromClipboard(i32),
}

impl App {
    fn calculate_attack(&self, to_nds: bool) -> f64 {
        let mut steps = 0;
        let mut vel = ZERO_POINT;
        if !to_nds {
            if self.attack != 0 {
                while vel < 0 {
                    steps += 1;
                    vel = self.attack_table[self.attack as usize] * vel / 0xff;
                }
                steps as f64 / TPS
            } else {
                INFINITY
            }
        } else {
            for i in 0..127_u8 {
                if i != 0 {
                    while vel < 0 {
                        steps += 1;
                        vel = self.attack_table[i as usize] * vel / 0xff;
                    }
                    if (steps as f64 / TPS) < self.attack_f as f64 {
                        return i as f64;
                    }
                }
                steps = 0;
                vel = ZERO_POINT;
            }
            127.0
        }
    }

    fn calculate_decay(&self, to_nds: bool) -> f64 {
        let mut steps = 0;
        let mut vel = 0;
        if !to_nds {
            while vel > ZERO_POINT {
                steps += 1;
                vel -= self.decay_table[self.decay as usize];
            }
            steps as f64 / TPS
        } else {
            for i in 0..127_u8 {
                if i != 0 {
                    while vel > ZERO_POINT {
                        steps += 1;
                        vel -= self.decay_table[i as usize];
                    }
                    if (steps as f64 / TPS) < self.decay_f as f64 {
                        return i as f64;
                    }
                }
                steps = 0;
                vel = 0;
            }
            127.0
        }
    }

    fn calculate_sustain(&self, to_nds: bool) -> f64 {
        let zero_point = ZERO_POINT as f64;
        if !to_nds {
            if self.sustain == 0 {
                0.0
            } else {
                let sus = self.sustain_table[(127 - self.sustain) as usize] as f64;
                let amplitude = sus / zero_point; // 0 is 1.0, 127 is 0.0
                let decibels = 20.0 * f64::log10(amplitude.abs());
                decibels.abs() // Written as "decibels to diminish by" in Polyphone
            }
        } else {
            for i in 0..127_u8 {
                if i != 127 {
                    let sus = self.sustain_table[(127 - i) as usize] as f64;
                    let amplitude = sus / zero_point;
                    let decibels = 20.0 * f64::log10(amplitude.abs());
                    if f64::from(-self.sustain_f) < decibels {
                        return i as f64;
                    }
                }
            }
            127.0
        }
    }

    fn calculate_release(&self, to_nds: bool) -> f64 {
        let mut steps = 0;
        let zero = -92544;
        let mut vel = 0;
        if !to_nds {
            while vel > zero {
                steps += 1;
                vel -= self.decay_table[self.release as usize]
            }
            steps as f64 / TPS
        } else {
            for i in 0..127_u8 {
                if i != 0 {
                    while vel > zero {
                        steps += 1;
                        vel -= self.decay_table[i as usize];
                    }
                    if (steps as f64 / TPS) < self.release_f as f64 {
                        return i as f64;
                    }
                }
                steps = 0;
                vel = 0;
            }
            127.0
        }
    }

    fn calculate(&mut self, to_nds: bool) {
        self.attack_result = self.calculate_attack(to_nds);
        self.decay_result = self.calculate_decay(to_nds);
        self.sustain_result = self.calculate_sustain(to_nds);
        self.release_result = self.calculate_release(to_nds);
        let result_string = match self.to_nds {
            true => format!(
                "Attack: {:.0} \nDecay: {:.0} \nSustain: {:.0} \nRelease: {:.0}",
                self.attack_result, self.decay_result, self.sustain_result, self.release_result
            ),
            false => format!(
                "Attack: {:.3} \nDecay: {:.3} \nSustain: {:.3} \nRelease: {:.3}",
                self.attack_result, self.decay_result, self.sustain_result, self.release_result
            ),
        };
        self.result = result_string
    }

    fn button_text(&self) -> String {
        if self.to_nds {
            "To SDAT".to_string()
        } else {
            "To SF2".to_string()
        }
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            clipboard: None,
            attack_table: [
                255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240,
                239, 238, 237, 236, 235, 234, 233, 232, 231, 230, 229, 228, 227, 226, 225, 224,
                223, 222, 221, 220, 219, 218, 217, 216, 215, 214, 213, 212, 211, 210, 209, 208,
                207, 206, 205, 204, 203, 202, 201, 200, 199, 198, 197, 196, 195, 194, 193, 192,
                191, 190, 189, 188, 187, 186, 185, 184, 183, 182, 181, 180, 179, 178, 177, 176,
                175, 174, 173, 172, 171, 170, 169, 168, 167, 166, 165, 164, 163, 162, 161, 160,
                159, 158, 157, 156, 155, 154, 153, 152, 151, 150, 149, 148, 147, 143, 137, 132,
                127, 123, 116, 109, 100, 92, 84, 73, 63, 51, 38, 26, 14, 5, 1, 0,
            ]
            .to_vec(),
            decay_table: [
                1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43,
                45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85,
                87, 89, 91, 93, 95, 97, 99, 101, 102, 104, 105, 107, 108, 110, 111, 113, 115, 116,
                118, 120, 122, 124, 126, 128, 130, 132, 135, 137, 140, 142, 145, 148, 151, 154,
                157, 160, 163, 167, 171, 175, 179, 183, 187, 192, 197, 202, 208, 213, 219, 226,
                233, 240, 248, 256, 265, 274, 284, 295, 307, 320, 334, 349, 366, 384, 404, 427,
                452, 480, 512, 549, 591, 640, 698, 768, 853, 960, 1097, 1280, 1536, 1920, 2560,
                3840, 7680, 15360, 65535,
            ]
            .to_vec(),
            sustain_table: [
                -92544, -92416, -92288, -83328, -76928, -71936, -67840, -64384, -61440, -58880,
                -56576, -54400, -52480, -50688, -49024, -47488, -46080, -44672, -43392, -42240,
                -41088, -40064, -39040, -38016, -36992, -36096, -35328, -34432, -33664, -32896,
                -32128, -31360, -30592, -29952, -29312, -28672, -28032, -27392, -26880, -26240,
                -25728, -25088, -24576, -24064, -23552, -23040, -22528, -22144, -21632, -21120,
                -20736, -20224, -19840, -19456, -19072, -18560, -18176, -17792, -17408, -17024,
                -16640, -16256, -16000, -15616, -15232, -14848, -14592, -14208, -13952, -13568,
                -13184, -12928, -12672, -12288, -12032, -11648, -11392, -11136, -10880, -10496,
                -10240, -9984, -9728, -9472, -9216, -8960, -8704, -8448, -8192, -7936, -7680,
                -7424, -7168, -6912, -6656, -6400, -6272, -6016, -5760, -5504, -5376, -5120, -4864,
                -4608, -4480, -4224, -3968, -3840, -3584, -3456, -3200, -2944, -2816, -2560, -2432,
                -2176, -2048, -1792, -1664, -1408, -1280, -1024, -896, -768, -512, -384, -128, 0,
            ]
            .to_vec(),
            attack: 127,
            attack_f: 0.0,
            attack_input: "".to_string(),
            attack_result: 0.0,
            decay: 127,
            decay_f: 0.0,
            decay_input: "".to_string(),
            decay_result: 0.0,
            sustain: 127,
            sustain_f: 0.0,
            sustain_input: "".to_string(),
            sustain_result: 0.0,
            release: 127,
            release_f: 0.0,
            release_input: "".to_string(),
            release_result: 0.0,
            result: "".to_string(),
            to_nds: false,
        }
    }

    fn scale_factor(&self) -> f64 {
        1.2
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn title(&self) -> String {
        "NDS ADSR Calculator".to_string()
    }

    fn view(&self) -> Element<Message> {
        column!(
            row!(
                column!(
                    text_input("127", &self.attack_input.to_string())
                        .on_input(Message::AttackChanged)
                        .on_submit(Message::CalculatePressed)
                        .on_paste(|_| Message::PasteFromClipboard(0)),
                    text("Attack"),
                ),
                column!(
                    text_input("127", &self.decay_input.to_string())
                        .on_input(Message::DecayChanged)
                        .on_submit(Message::CalculatePressed)
                        .on_paste(|_| Message::PasteFromClipboard(1)),
                    text("Decay"),
                ),
                column!(
                    text_input("127", &self.sustain_input.to_string())
                        .on_input(Message::SustainChanged)
                        .on_submit(Message::CalculatePressed)
                        .on_paste(|_| Message::PasteFromClipboard(2)),
                    text("Sustain"),
                ),
                column!(
                    text_input("127", &self.release_input.to_string())
                        .on_input(Message::ReleaseChanged)
                        .on_submit(Message::CalculatePressed)
                        .on_paste(|_| Message::PasteFromClipboard(3)),
                    text("Release"),
                ),
            ),
            button(text(self.button_text()))
                .on_press(Message::CalculatePressed)
                .padding(Padding::from([10, 20])),
            text(self.result.to_string()),
            button(text("Copy to clipboard".to_string()))
                .on_press(Message::CopyToClipboard)
                .padding(Padding::from([10, 20])),
            button(text("Paste from clipboard".to_string()))
                .on_press(Message::PasteFromClipboard(0))
                .padding(Padding::from([10, 20])),
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::CalculatePressed => {
                if !self.attack == 0 || !self.decay == 0 || !self.sustain == 0 || !self.release == 0
                {
                    self.to_nds = false
                }
                self.calculate(self.to_nds)
            }
            Message::AttackChanged(s) => {
                if string_is_int(s.clone()) {
                    self.attack_input = s.clone();
                    self.attack_f = s.clone().parse().unwrap();
                    self.attack = s.parse().unwrap()
                } else {
                    self.attack_input = s.clone();
                    self.to_nds = true;
                    self.attack = 0;
                    self.attack_f = s.parse().unwrap_or(0.0)
                }
            }
            Message::DecayChanged(s) => {
                if string_is_int(s.clone()) {
                    self.decay_input = s.clone();
                    self.decay_f = s.clone().parse().unwrap();
                    self.decay = s.parse().unwrap()
                } else {
                    self.decay_input = s.clone();
                    self.to_nds = true;
                    self.decay = 0;
                    self.decay_f = s.parse().unwrap_or(0.0)
                }
            }
            Message::SustainChanged(s) => {
                if string_is_int(s.clone()) {
                    self.sustain_input = s.clone();
                    self.sustain_f = s.clone().parse().unwrap();
                    self.sustain = s.parse().unwrap()
                } else {
                    self.sustain_input = s.clone();
                    self.to_nds = true;
                    self.sustain = 0;
                    self.sustain_f = s.parse().unwrap_or(0.0)
                }
            }
            Message::ReleaseChanged(s) => {
                if string_is_int(s.clone()) {
                    self.release_input = s.clone();
                    self.release_f = s.clone().parse().unwrap();
                    self.release = s.parse().unwrap()
                } else {
                    self.release_input = s.clone();
                    self.to_nds = true;
                    self.release = 0;
                    self.release_f = s.parse().unwrap_or(0.0)
                }
            }
            Message::CopyToClipboard => {
                let content = if !self.to_nds {
                   format!("{:.3}\n\n{:.3}\n{:.3}\n{:.3}", self.attack_result, self.decay_result, self.sustain_result, self.release_result)
                } else {
                    format!("{:.0}\t{:.0}\t{:.0}\t{:.0}", self.attack_result, self.decay_result, self.sustain_result, self.release_result)
                };
                self.clipboard = Some(ClipboardProvider::new().unwrap());
                if let Some(ref mut cb) = self.clipboard {
                    if cb.set_contents(content.to_string()).is_ok() {
                        let _ = cb.set_contents(content.to_string());
                    }
                }
            }
            Message::PasteFromClipboard(place) => {
                self.clipboard = Some(ClipboardProvider::new().unwrap());
                if let Some(ref mut cb) = self.clipboard {
                    let content = cb.get_contents().unwrap_or("".to_string());
                    if !content.is_empty() {
                        let content_iter = content.split_ascii_whitespace();
                        let mut count = place;
                        let mut num = "".to_string();
                        for s in content_iter {
                            if s.parse::<u8>().is_ok() {
                                num = s.parse::<u8>().unwrap().to_string();
                            } else if s.parse::<f32>().is_ok() {
                                num = s.parse::<f32>().unwrap().to_string();
                            } 
                            match count {
                                0 => {
                                    self.attack_input = num.clone();
                                    self.update(Message::AttackChanged(num.clone()));
                                }
                                1 => {
                                    self.decay_input = num.clone();
                                    self.update(Message::DecayChanged(num.clone()));
                                }
                                2 => {
                                    self.sustain_input = num.clone();
                                    self.update(Message::SustainChanged(num.clone()));
                                }
                                3 => {
                                    self.release_input = num.clone();
                                    self.update(Message::ReleaseChanged(num.clone()));
                                }
                                _ => break
                            }
                            count += 1;
                        }
                    }
                }
            }
        }
        if string_is_int(self.attack_input.clone())
            && string_is_int(self.decay_input.clone())
            && string_is_int(self.sustain_input.clone())
            && string_is_int(self.release_input.clone())
        {
            self.to_nds = false
        }
    }
}

fn string_is_int(s: String) -> bool {
    let x_int_result = s.parse::<u8>();
    let x_float_result = s.parse::<f32>();
    if let Ok(_x) = x_int_result {
        true
    } else if let Ok(_x) = x_float_result {
        false
    } else {
        false
    }
}

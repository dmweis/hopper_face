use crate::driver::{
    ColorPacket, ALL_COLORS, BIGGER_RING_PIXEL_COUNT, BLUE, BRIGHT_COLORS, GREEN, NORMAL_COLORS,
    PIXEL_COUNT, RED, RGB, SMALLER_RING_PIXEL_COUNT,
};

fn map(value: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    let value = value as f32;
    let in_min = in_min as f32;
    let in_max = in_max as f32;
    let out_min = out_min as f32;
    let out_max = out_max as f32;
    let result = (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    result.round() as i32
}

#[derive(Debug)]
pub enum Animation {
    LarsonScanner(RGB),
    RunAnimation(RGB),
    CycleAllColors,
    CycleBrightColors,
    CycleNormalColors,
    CountDownBasic,
    CountDown(Vec<RGB>),
    Off,
}

impl Animation {
    pub fn to_iterator(&self) -> Box<dyn Iterator<Item = ColorPacket>> {
        match self {
            Animation::LarsonScanner(color) => Box::new(LarsonScanner::new(*color)),
            Animation::RunAnimation(color) => Box::new(RunAnimation::new(*color)),
            Animation::CycleAllColors => Box::new(CycleColors::all_colors()),
            Animation::CycleBrightColors => Box::new(CycleColors::bright_colors()),
            Animation::CycleNormalColors => Box::new(CycleColors::normal_colors()),
            Animation::CountDownBasic => Box::new(CountDownAnimation::default()),
            Animation::CountDown(colors) => Box::new(CountDownAnimation::new(colors.clone())),
            Animation::Off => Box::new(std::iter::repeat(ColorPacket::off())),
        }
    }
}

pub struct LarsonScanner {
    index: i32,
    color: RGB,
}

impl LarsonScanner {
    pub fn new(color: RGB) -> Self {
        LarsonScanner { index: 0, color }
    }
}

impl Iterator for LarsonScanner {
    type Item = ColorPacket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 48 {
            return None;
        }
        let index_smaller = map(self.index, 0, 48, 0, BIGGER_RING_PIXEL_COUNT as i32);
        let index_bigger = map(
            self.index,
            0,
            48,
            BIGGER_RING_PIXEL_COUNT as i32,
            (BIGGER_RING_PIXEL_COUNT + SMALLER_RING_PIXEL_COUNT) as i32,
        );
        let mut frame = ColorPacket::default();
        frame.set_pixel(index_smaller, self.color);
        frame.set_pixel(index_smaller - 1, self.color.fade_out(0.4));
        frame.set_pixel(index_smaller - 2, self.color.fade_out(0.2));
        frame.set_pixel(index_smaller - 3, self.color.fade_out(0.1));
        frame.set_pixel(index_bigger, self.color);
        frame.set_pixel(index_bigger - 1, self.color.fade_out(0.4));
        frame.set_pixel(index_bigger - 2, self.color.fade_out(0.2));
        frame.set_pixel(index_bigger - 3, self.color.fade_out(0.1));
        self.index += 1;
        Some(frame)
    }
}

pub struct RunAnimation {
    index: i32,
    color: RGB,
}

impl RunAnimation {
    pub fn new(color: RGB) -> Self {
        RunAnimation { index: 0, color }
    }
}

impl Iterator for RunAnimation {
    type Item = ColorPacket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= PIXEL_COUNT as i32 {
            return None;
        }

        let mut frame = ColorPacket::default();
        frame.set_pixel(self.index, self.color);
        self.index += 1;
        Some(frame)
    }
}

pub struct CycleColors {
    index: usize,
    colors: &'static [RGB],
}

impl CycleColors {
    pub fn all_colors() -> Self {
        CycleColors {
            index: 0,
            colors: &ALL_COLORS,
        }
    }

    pub fn bright_colors() -> Self {
        CycleColors {
            index: 0,
            colors: &BRIGHT_COLORS,
        }
    }

    pub fn normal_colors() -> Self {
        CycleColors {
            index: 0,
            colors: &NORMAL_COLORS,
        }
    }
}

impl Iterator for CycleColors {
    type Item = ColorPacket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.colors.len() {
            return None;
        }
        let result = Some(ColorPacket::with_color(self.colors[self.index]));
        self.index += 1;
        result
    }
}

#[derive(Debug)]
pub struct CountDownAnimation {
    index: usize,
    frame: ColorPacket,
    colors: Vec<RGB>,
}

impl Default for CountDownAnimation {
    fn default() -> Self {
        Self::new(vec![BLUE, RED, GREEN])
    }
}

impl CountDownAnimation {
    pub fn new(colors: Vec<RGB>) -> Self {
        Self {
            index: 0,
            frame: ColorPacket::off(),
            colors,
        }
    }
}

impl Iterator for CountDownAnimation {
    type Item = ColorPacket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= PIXEL_COUNT {
            self.colors.pop();
            self.index = 0;
        }
        if self.colors.is_empty() {
            return None;
        }
        self.frame
            .set_pixel(self.index as i32, *self.colors.first().unwrap());
        self.index += 1;
        Some(self.frame.clone())
    }
}

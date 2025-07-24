use crate::util::{UP, DOWN, LEFT, RIGHT, TOP, BOTTOM, CLEAR};

const TOTAL: f32 = 100.0;

pub struct LoadingBar {
    /// Current value for the loading bar
    _current: f32,
    /// Width of the loading bar
    pub width: usize,
    /// Height of the loading bar
    pub height: usize,
    /// Character for the filled part of the bar
    pub bar_char: char,
    /// Character for the empty part of the bar
    pub empty_char: char,
    /// Character for both sides of the bar
    pub container_char: char,
    /// Character for the left side of the container
    pub container_left: char,
    /// Character for the right side of the container
    pub container_right: char,
    /// ansi code for the color of the bar
    pub fill_color: &'static str,
    /// ansi code for the color of the container
    pub container_color: &'static str,
    /// ansi code for the color of the empty part of the bar
    pub empty_color: &'static str,
}

impl LoadingBar {
    /// default settings for the loading bar
    pub fn new() -> Self {
        let l = LoadingBar {
            _current: 0.0,
            width: 50,
            height: 1,
            bar_char: '█',
            empty_char: '▒',
            container_char: '|',
            container_left: '[',
            container_right: ']',
            fill_color: "\u{1b}[32m",
            container_color: "\u{1b}[33m",
            empty_color: "\u{1b}[37m",
        };
        // Assuming `top` and `bottom` are format strings like "{}"
        // l.show(); // Assuming you have a `show` method for LoadingBar
        print!("{}", TOP);
        l
    }

    pub fn show(&self) {
        print!("\r{color_side}{left}{reset}{color_empty}{empty}{reset}{color_side}{right}{reset}",
            color_side = self.container_color,
            left = self.container_char,
            color_empty = self.empty_color,
            empty = self.empty_char.to_string().repeat(self.width),
            right = self.container_char,
            reset = "\u{1b}[0m"
        );
    }

    /// Render the loading bar to the console
    pub fn lerp(&mut self, percent: i32, unique_sides: bool) {
        let loading_length = (((percent as f32 / TOTAL) * self.width as f32).round() as usize).clamp(0, self.width);
        let loading = self.bar_char.to_string().repeat(loading_length);
        let padding = self.empty_char.to_string().repeat(self.width - loading_length);

        print!("{TOP}");
        // Example loading graphic: [█████     ]
        if unique_sides {
            print!(
                "\r{color_side}{left}{reset}{color_bar}{bar}{reset}{color_empty}{empty}{reset}{color_side}{right}{reset}",
                color_side = self.container_color,
                left = self.container_left,
                color_bar = self.fill_color,
                bar = loading,
                color_empty = self.empty_color,
                empty = padding,
                right = self.container_right,
                reset = "\u{1b}[0m"
            );
        } else {
            print!(
                "\r{color_side}{side}{reset}{color_bar}{bar}{reset}{color_empty}{empty}{reset}{color_side}{side}{reset}",
                color_side = self.container_color,
                side = self.container_char,
                color_bar = self.fill_color,
                bar = loading,
                color_empty = self.empty_color,
                empty = padding,
                reset = "\u{1b}[0m"
            );
        }
        print!("{DOWN}");
        //std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
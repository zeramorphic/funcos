use core::fmt::Write;

use spin::Mutex;

use crate::{colour::Colour, linalg::vec::Vec2, screen_font::ScreenFont, video::VideoBuffer};

/// This structure owns a video buffer and some fonts,
/// and treats the entire video buffer as a terminal.
///
/// This will not allocate memory (we haven't written that yet!),
/// so it can't perform any of the usual terminal interactions.
///
/// # Invariants
///
/// `cursor.x < width()` and `cursor.y < height()`.
/// The `regular_font` and the `bold_font` must have the same `character_size`.
pub struct TerminalVideoBuffer {
    video_buffer: VideoBuffer,
    regular_font: ScreenFont,
    bold_font: ScreenFont,
    cursor: Vec2<usize>,
    foreground: Colour,
    background: Colour,
}

/// For now, we'll have a single static terminal object.
/// Calls to `println!` will output to this terminal.
///
/// # Invariants
///
/// It must only be used inside `with_terminal` blocks.
static TERMINAL: Mutex<Option<TerminalVideoBuffer>> = Mutex::new(None);

impl TerminalVideoBuffer {
    pub fn new(video_buffer: VideoBuffer) -> Self {
        Self {
            video_buffer,
            regular_font: ScreenFont::regular_font(),
            bold_font: ScreenFont::bold_font(),
            cursor: Vec2::zero(),
            foreground: Colour::WHITE,
            background: Colour::BLACK,
        }
    }

    /// Registers this terminal as the default terminal for things like `println!`.
    ///
    /// # Panics
    ///
    /// If there is already a default terminal, this will panic.
    pub fn make_default(self) {
        let mut default = TERMINAL.lock();
        match *default {
            Some(_) => panic!("default terminal already assigned"),
            None => {
                *default = Some(self);
            }
        }
    }

    /// Runs a given closure with the default terminal.
    ///
    /// # Panics
    ///
    /// Panics if a default terminal has not been assigned.
    pub fn with_default<T>(f: impl FnOnce(&mut TerminalVideoBuffer) -> T) -> T {
        f(TERMINAL.lock().as_mut().unwrap())
    }

    /// Returns the width of this terminal in characters.
    pub fn width(&self) -> usize {
        self.video_buffer.width() / 8
    }

    /// Returns the height of this terminal in characters.
    pub fn height(&self) -> usize {
        self.video_buffer.height() / self.regular_font.header().character_size as usize
    }

    pub fn clear_screen(&mut self) {
        self.video_buffer.fill_buffer(self.background);
    }

    /// Set the cursor to the given position, clamping if bounds are exceeded.
    pub fn set_cursor(&mut self, pos: Vec2<usize>) {
        self.cursor = Vec2::new(
            pos.x.min(self.width().saturating_sub(1)),
            pos.y.min(self.height().saturating_sub(1)),
        );
    }

    /// Advance the cursor to the start of the next line.
    ///
    /// If the cursor would advance past the end of the screen,
    /// the screen is first moved upwards so that there is room for the new cursor position.
    pub fn put_newline(&mut self) {
        self.cursor = Vec2::new(0, self.cursor.y + 1);
        if self.cursor.y == self.width() {
            // TODO: Push the screen up
            unimplemented!()
        }
    }

    /// Advances the cursor to the next position.
    ///
    /// If this would advance the cursor past the end of a line,
    /// the cursor will be placed at the start of the next line.
    ///
    /// If the cursor would advance past the end of the screen,
    /// the screen is first moved upwards so that there is room for the new cursor position.
    pub fn advance_cursor(&mut self) {
        if self.cursor.x + 1 == self.width() {
            self.put_newline();
        } else {
            self.cursor.x += 1;
        }
    }

    /// Regardless of the current cursor position,
    /// draw the given character at the given position (in characters) on the terminal screen.
    /// This does not check whether `c` is `\n`, for example.
    ///
    /// # Panics
    ///
    /// If the position is out of bounds (we need `pos.x < width` and `pos.y < height`),
    /// this will panic.
    pub fn put_char_at(&mut self, c: u8, pos: Vec2<usize>) {
        assert!(pos.x < self.width());
        assert!(pos.y < self.height());
        unsafe {
            self.video_buffer.draw_glyph_unchecked(
                pos * Vec2::new(8, self.regular_font.header().character_size as usize),
                &self.regular_font,
                c,
                self.foreground,
                self.background,
            );
        }
    }

    /// Draw the given character on the terminal screen, advancing the internal cursor.
    /// This does not check whether `c` is `\n`, for example; see also [Self::put_char].
    pub fn put_char_raw(&mut self, c: u8) {
        self.put_char_at(c, self.cursor);
        self.advance_cursor();
    }

    /// Draw the given character on the terminal screen, advancing the internal cursor.
    ///
    /// # Special characters
    ///
    /// * `\n`: instead, moves the cursor to the start of a new line
    /// * `\t`: instead, moves the cursor until the `x` position is a multiple of 4
    pub fn put_char(&mut self, c: u8) {
        match c {
            b'\n' => self.put_newline(),
            b'\t' => {
                for _ in 0..4 - (self.cursor.x % 4) {
                    self.advance_cursor();
                }
            }
            _ => self.put_char_raw(c),
        }
    }

    /// Draw the given string on the terminal screen, advancing the internal cursor.
    /// For special characters, see [Self::put_char].
    pub fn put_string(&mut self, string: &[u8]) {
        for c in string {
            self.put_char(*c);
        }
    }
}

impl Write for TerminalVideoBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.put_string(s.as_bytes());
        Ok(())
    }
}

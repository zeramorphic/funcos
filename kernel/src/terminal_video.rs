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

    /// Advances the cursor to the next position.
    ///
    /// If this would advance the cursor past the end of a line,
    /// the cursor will be placed at the start of the next line.
    ///
    /// If the cursor would advance past the end of the screen,
    /// the screen is first moved upwards so that there is room for the new cursor position.
    pub fn advance_cursor(&mut self) {
        if self.cursor.x + 1 == self.width() {
            self.cursor = Vec2::new(0, self.cursor.y + 1);
        } else if self.cursor.y + 1 == self.height() {
            // TODO: Move screen upwards.
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
    /// This does not check whether `c` is `\n`, for example; see also `put_char`.
    pub fn put_char_raw(&mut self, c: u8) {
        self.put_char_at(c, self.cursor);
        self.advance_cursor();
    }
}

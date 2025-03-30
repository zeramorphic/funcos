use crate::{
    colour::Colour,
    linalg::{rect::Rect, vec::Vec2},
    screen_font::ScreenFont,
};

/// A data structure that semantically owns a framebuffer.
/// Each pixel is a [Colour].
/// The pixel at `(x, y)` is at byte offset `x * 4 + pitch * y` from `addr`.
pub struct VideoBuffer {
    /// This struct owns the space from `addr` up to `addr + height * pitch`.
    addr: *mut Colour,
    width: usize,
    height: usize,
    pitch: usize,
}

impl VideoBuffer {
    pub fn from_limine(framebuffer: limine::framebuffer::Framebuffer) -> Self {
        Self {
            addr: framebuffer.addr().cast(),
            width: framebuffer.width() as usize,
            height: framebuffer.height() as usize,
            pitch: framebuffer.pitch() as usize,
        }
    }

    /// Returns the width of the video buffer in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the video buffer in pixels.
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn screen_rect(&self) -> Rect<usize> {
        Rect::new_zero_to_max(Vec2::new(self.width, self.height))
    }

    /// Without bounds checking, draw the given pixel.
    ///
    /// # Safety
    ///
    /// We must have `x < width` and `y < height`.
    pub unsafe fn draw_pixel_unchecked(&mut self, pos: Vec2<usize>, colour: Colour) {
        self.addr
            .byte_add(self.pitch * pos.y)
            .add(pos.x)
            .write(colour);
    }

    /// Without bounds checking, draw a solid rectangle of the given colour.
    /// The maximum on the `rect` is treated as exclusive bounds.
    ///
    /// # Safety
    ///
    /// `min.x <= max.x <= width` and `min.y <= max.x <= height`.
    pub unsafe fn draw_rect_unchecked(&mut self, rect: Rect<usize>, colour: Colour) {
        let mut addr = self.addr;
        let width = rect.width();
        let height = rect.height();
        for _ in 0..height {
            for x in 0..width {
                addr.add(x).write(colour);
            }
            addr = addr.byte_add(self.pitch);
        }
    }

    /// Draw a solid rectangle of the given colour.
    /// The maximum on the `rect` is treated as exclusive bounds.
    ///
    /// # Panics
    ///
    /// Panics unless `min.x <= max.x <= width` and `min.y <= max.x <= height`.
    pub fn draw_rect(&mut self, rect: Rect<usize>, colour: Colour) {
        assert!(rect.min().x <= rect.max().x);
        assert!(rect.max().x <= self.width);
        assert!(rect.min().y <= rect.max().y);
        assert!(rect.max().y <= self.height);
        unsafe {
            self.draw_rect_unchecked(rect, colour);
        }
    }

    /// Fills the entire buffer with the given colour.
    pub fn fill_buffer(&mut self, colour: Colour) {
        unsafe {
            self.draw_rect_unchecked(self.screen_rect(), colour);
        }
    }

    /// # Safety
    ///
    /// `pos.x + 8 < width`, `0 <= pos.y < height`.
    pub unsafe fn draw_glyph_unchecked(
        &mut self,
        pos: Vec2<usize>,
        font: &ScreenFont,
        index: u8,
        foreground: Colour,
        background: Colour,
    ) {
        let mut addr = self.addr.byte_add(self.pitch * pos.y).add(pos.x);
        let height = font.header().character_size;
        let mut glyph_data = font.font_data().add(height as usize * index as usize);
        for _ in 0..height {
            for x in 0..8 {
                addr.add(x).write(if (*glyph_data) & (1 << (7 - x)) > 0 {
                    foreground
                } else {
                    background
                });
            }
            addr = addr.byte_add(self.pitch);
            glyph_data = glyph_data.add(1);
        }
    }
}

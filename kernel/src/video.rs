use crate::{
    colour::Colour,
    linalg::{rect::Rect, vec::Vec2},
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

    /// Without bounds checking, draw the given pixel.
    ///
    /// # Safety
    ///
    /// We must have `0 <= x < width` and `0 <= y < height`.
    pub unsafe fn draw_pixel(&mut self, pos: Vec2<usize>, colour: Colour) {
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
    /// `0 <= min.x <= max.x < width` and `0 <= min.y <= max.x < height`.
    pub unsafe fn draw_rect(&mut self, rect: Rect<usize>, colour: Colour) {
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

    /// Returns the width of the video buffer in pixels.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the video buffer in pixels.
    pub fn height(&self) -> usize {
        self.height
    }
}

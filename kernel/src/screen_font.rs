use bytemuck::{checked::from_bytes, Pod, Zeroable};

/// This struct references a PC screen font (version 1) stored statically in the kernel binary.
/// The glyphs are 8 pixels wide and `header.character_size` pixels tall.
///
/// # Invariants
///
/// `font_data` must be a pointer to the start of `256 * header.character_size` bytes of font data.
pub struct ScreenFont {
    header: ScreenFontHeader,
    font_data: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ScreenFontHeader {
    magic: [u8; 2],
    pub font_mode: FontMode,
    pub character_size: u8,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Pod, Zeroable)]
    pub struct FontMode: u8 {
        /// If this bit is set, the font face will have 512 glyphs.
        /// If it is unset, then the font face will have just 256 glyphs.
        const Mode512 = 0x01;
        /// If this bit is set, the font face will have a unicode table.
        const HasTab = 0x02;
        /// Equivalent to `HasTab`
        const Seq = 0x04;
        const _ = !0;
    }
}

impl ScreenFont {
    /// Parse a PC screen font from data stored in the binary.
    pub fn from_data(data: &'static [u8]) -> Self {
        let header: ScreenFontHeader = *from_bytes(&data[0..4]);
        if header.magic != [0x36, 0x04] {
            panic!(
                "PC screen font magic number incorrect, got {:?}",
                header.magic
            )
        }

        Self {
            header,
            font_data: unsafe { data.as_ptr().add(4) },
        }
    }

    pub fn regular_font() -> Self {
        Self::from_data(include_bytes!("../data/fonts/Lat2-Terminus14.psf"))
    }

    pub fn bold_font() -> Self {
        Self::from_data(include_bytes!("../data/fonts/Lat2-TerminusBold14.psf"))
    }

    pub fn header(&self) -> ScreenFontHeader {
        self.header
    }

    pub fn font_data(&self) -> *const u8 {
        self.font_data
    }
}

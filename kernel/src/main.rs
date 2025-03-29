#![no_std]
#![no_main]

pub mod colour;
pub mod linalg;
pub mod video;

use core::arch::asm;

use colour::Colour;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;
use linalg::rect::Rect;
use linalg::vec::Vec2;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
/// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the start and end markers for Limine requests.
#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be removed by the linker.
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let mut buffer = video::VideoBuffer::from_limine(framebuffer);
            loop {
                for colour in [Colour::RED, Colour::GREEN, Colour::BLUE] {
                    unsafe {
                        buffer.draw_rect(
                            Rect::new_unchecked(
                                Vec2::zero(),
                                Vec2::new(buffer.width(), buffer.height()),
                            ),
                            colour,
                        );
                    }
                }
            }
        }
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use std::os::raw::{c_char, c_int};

pub mod window;
pub mod draw;
pub mod event;
pub mod color;

//////////
// Type //
//////////

/// xcb_connection_t
pub enum Connection {}

/// xcb_keycode_t
pub type Keycode = u8;

/// xcb_setup_t
#[repr(C)]
pub struct Setup {
  pub status:                      u8,
  pub pad0:                        u8,
  pub protocol_major_version:      u16,
  pub protocol_minor_version:      u16,
  pub length:                      u16,
  pub release_number:              u32,
  pub resource_id_base:            u32,
  pub resource_id_mask:            u32,
  pub motion_buffer_size:          u32,
  pub vendor_len:                  u16,
  pub maximum_request_length:      u16,
  pub roots_len:                   u8,
  pub pixmap_formats_len:          u8,
  pub image_byte_order:            u8,
  pub bitmap_format_bit_order:     u8,
  pub bitmap_format_scanline_unit: u8,
  pub bitmap_format_scanline_pad:  u8,
  pub min_keycode:                 Keycode,
  pub max_keycode:                 Keycode,
  pub pad1:                        [u8; 4],
}

/// xcb_screen_t
#[repr(C)]
pub struct Screen {
  pub root:                  window::Window,
  pub default_colormap:      color::Colormap,
  pub white_pixel:           u32,
  pub black_pixel:           u32,
  pub current_input_masks:   u32,
  pub width_in_pixels:       u16,
  pub height_in_pixels:      u16,
  pub width_in_millimeters:  u16,
  pub height_in_millimeters: u16,
  pub min_installed_maps:    u16,
  pub max_installed_maps:    u16,
  pub root_visual:           window::Visualid,
  pub backing_stores:        u8,
  pub save_unders:           u8,
  pub root_depth:            u8,
  pub allowed_depths_len:    u8,
}

/// xcb_screen_iterator_t
#[repr(C)]
pub struct ScreenIterator<'a> {
  pub data:  *mut Screen,
  pub rem:   c_int,
  pub index: c_int,
  _phantom:  std::marker::PhantomData<&'a Screen>,
}

/// xcb_void_cookie_t
#[repr(C)]
pub struct VoidCookie {
  pub sequence: c_int
}

///////////////
// Functions //
///////////////

#[link(name = "xcb")]
extern "C" {

  /// xcb_connect
  #[link_name = "xcb_connect"]
  pub fn connect(displayname: *const c_char, screenp: *mut c_int) -> *mut Connection;
  
  /// xcb_setu@_roots_iterator
  #[link_name = "xcb_setup_roots_iterator"]
  pub fn setup_roots_iterator<'a>(R: *const Setup) -> ScreenIterator<'a>;
  
  /// xcb_get_setup
  #[link_name = "xcb_get_setup"]
  pub fn get_setup(c: *mut Connection) -> *const Setup;
  
  /// xcb_generate_id
  #[link_name = "xcb_generate_id"]
  pub fn generate_id(c: *mut Connection) -> u32;
  
  /// xcb_flush
  #[link_name = "xcb_flush"]
  pub fn flush(c: *mut Connection) -> c_int;
  
}


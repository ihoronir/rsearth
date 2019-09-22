use crate::xcb;

///////////
// Types //
///////////

/// xcb_window_t
pub type Window = u32;

/// xcb_visualid_t
pub type Visualid = u32;

/// xcb_drawable_t
pub type Drawable = u32;

///////////////
// Functions //
///////////////

#[link(name = "xcb")]
extern "C" {

  /// xcb_create_window
  #[link_name = "xcb_create_window"]
  pub fn create_window(
    c:            *mut xcb::Connection,
    depth:        u8, 
    wid:          Window, 
    parent:       Window, 
    x:            i16, 
    y:            i16, 
    width:        u16, 
    height:       u16, 
    border_width: u16, 
    class:        u16, 
    visual:       Visualid, 
    value_mask:   u32, 
    value_list:   *const u32
  ) -> xcb::VoidCookie;

  /// xcb_map_window
  #[link_name = "xcb_map_window"]
  pub fn map_window(c: *mut xcb::Connection, window: Window) -> xcb::VoidCookie;

}

///////////////
// Constants //
///////////////

pub const COPY_FROM_PARENT: u8 = 0;

/// xcb_window_class_t
pub type WindowClass = u16;

pub const WINDOW_CLASS_INPUT_OUTPUT: WindowClass = 0x01;

/// xcb_cw_t
pub type CW = u32;

pub const CW_BACK_PIXMAP      : CW =   0x01;
pub const CW_BACK_PIXEL       : CW =   0x02;
pub const CW_BORDER_PIXMAP    : CW =   0x04;
pub const CW_BORDER_PIXEL     : CW =   0x08;
pub const CW_BIT_GRAVITY      : CW =   0x10;
pub const CW_WIN_GRAVITY      : CW =   0x20;
pub const CW_BACKING_PLANES   : CW =   0x80;
pub const CW_BACKING_PIXEL    : CW =  0x100;
pub const CW_OVERRIDE_REDIRECT: CW =  0x200;
pub const CW_SAVE_UNDER       : CW =  0x400;
pub const CW_EVENT_MASK       : CW =  0x800;
pub const CW_DONT_PROPAGATE   : CW = 0x1000;
pub const CW_COLORMAP         : CW = 0x2000;
pub const CW_CURSOR           : CW = 0x4000;

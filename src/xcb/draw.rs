use crate::xcb;

///////////
// Types //
///////////

/// xcb_gcontext_t
pub type GContext = u32;

/// xcb_point_t
#[repr(C)]
pub struct Point {
  pub x: i16,
  pub y: i16
}

/// xcb_segment_t
#[repr(C)]
pub struct Segment {
  pub x1: i16,
  pub y1: i16,
  pub x2: i16,
  pub y2: i16,
}

/// xcb_rectangle_t
#[repr(C)]
pub struct Rectangle {
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
}

/// xcb_arc_t
#[repr(C)]
pub struct Arc {
  pub x: i16,
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub angle1: i16,
  pub angle2: i16,
}

///////////////
// Functions //
///////////////

#[link(name = "xcb")]
extern "C" {

  /// xcb_create_gc
  #[link_name = "xcb_create_gc"]
  pub fn create_gc(
    c:          *mut xcb::Connection,
    cid:        GContext,
    drawable:   xcb::window::Drawable,
    value_mask: u32,
    value_list: *const u32
  ) -> xcb::VoidCookie;
  
  /// xcb_change_gc
  #[link_name = "xcb_change_gc"]
  pub fn change_gc(
    c:          *mut xcb::Connection,
    gc:         GContext,
    value_mask: u32,
    value_list: *const u32
  ) -> xcb::VoidCookie;
  
  /// xcb_poly_point
  #[link_name = "xcb_poly_point"]
  pub fn poly_point(
    c:               *mut xcb::Connection,
    coordinate_mode: u8,
    drawable:        xcb::window::Drawable,
    gc:              GContext,
    points_len:      u32,
    points:          *const Point
  ) -> xcb::VoidCookie;
  
  /// xcb_poly_line
  #[link_name = "xcb_poly_line"]
  pub fn poly_line(
    c:               *mut xcb::Connection,
    coordinate_mode: u8,
    drawable:        xcb::window::Drawable,
    gc:              GContext,
    points_len:      u32,
    points:          *const Point
  ) -> xcb::VoidCookie;
  
  /// xcb_poly_segment
  #[link_name = "xcb_poly_segment"]
  pub fn poly_segment (
    c:            *mut xcb::Connection,
    drawable:     xcb::window::Drawable,
    gc:           GContext,
    segments_len: u32,
    segments:     *const Segment
  ) -> xcb::VoidCookie;
  
  /// xcb_poly_segment
  #[link_name = "xcb_poly_rectangle"]
  pub fn poly_rectangle (
    c:              *mut xcb::Connection,
    drawable:       xcb::window::Drawable,
    gc:             GContext,
    rectangles_len: u32,
    rectangles:     *const Rectangle
  ) -> xcb::VoidCookie;

  /// xcb_poly_arc
  #[link_name = "xcb_poly_arc"]
  pub fn poly_arc (
    c:        *mut xcb::Connection,
    drawable: xcb::window::Drawable,
    gc:       GContext,
    arcs_len: u32,
    arcs:     *const Arc
  ) -> xcb::VoidCookie;
}

///////////////
// Constants //
///////////////

/// xcb_coord_mode_t
pub type CoordMode = u8;

pub const COORD_MODE_ORIGIN  : CoordMode = 0x00;
pub const COORD_MODE_PREVIOUS: CoordMode = 0x01;

/// xcb_gc_t
pub type GC = u32;

pub const GC_FUNCTION             : GC =     0x01;
pub const GC_PLANE_MASK           : GC =     0x02;
pub const GC_FOREGROUND           : GC =     0x04;
pub const GC_BACKGROUND           : GC =     0x08;
pub const GC_LINE_WIDTH           : GC =     0x10;
pub const GC_LINE_STYLE           : GC =     0x20;
pub const GC_CAP_STYLE            : GC =     0x40;
pub const GC_JOIN_STYLE           : GC =     0x80;
pub const GC_FILL_STYLE           : GC =    0x100;
pub const GC_FILL_RULE            : GC =    0x200;
pub const GC_TILE                 : GC =    0x400;
pub const GC_STIPPLE              : GC =    0x800;
pub const GC_TILE_STIPPLE_ORIGIN_X: GC =   0x1000;
pub const GC_TILE_STIPPLE_ORIGIN_Y: GC =   0x2000;
pub const GC_FONT                 : GC =   0x4000;
pub const GC_SUBWINDOW_MODE       : GC =   0x8000;
pub const GC_GRAPHICS_EXPOSURES   : GC =  0x10000;
pub const GC_CLIP_ORIGIN_X        : GC =  0x20000;
pub const GC_CLIP_ORIGIN_Y        : GC =  0x40000;
pub const GC_CLIP_MASK            : GC =  0x80000;
pub const GC_DASH_OFFSET          : GC = 0x100000;
pub const GC_DASH_LIST            : GC = 0x200000;
pub const GC_ARC_MODE             : GC = 0x400000;


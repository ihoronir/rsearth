use crate::xcb;

///////////
// Types //
///////////

/// xcb_generic_event_t
#[repr(C)]
pub struct GenericEvent {
  pub response_type: u8,
  pub pad0:          u8,
  pub sequence:      u16,
  pub pad:           [u32; 7],
  pub full_sequence: u32
}

///////////////
// Functions //
///////////////

#[link(name = "xcb")]
extern "C" {

  /// xcb_wait_for_event
  #[link_name = "xcb_wait_for_event"]
  pub fn wait_for_event(c: *mut xcb::Connection) -> *mut GenericEvent;

}

///////////////
// Constants //
///////////////

/// XCB_EXPOSE
pub const EXPOSE: u8 = 12;

/// xcb_event_mask_t
pub type EventMask = u32;

pub const EVENT_MASK_NO_EVENT             : EventMask =      0x00;
pub const EVENT_MASK_KEY_PRESS            : EventMask =      0x01;
pub const EVENT_MASK_KEY_RELEASE          : EventMask =      0x02;
pub const EVENT_MASK_BUTTON_PRESS         : EventMask =      0x04;
pub const EVENT_MASK_BUTTON_RELEASE       : EventMask =      0x08;
pub const EVENT_MASK_ENTER_WINDOW         : EventMask =      0x10;
pub const EVENT_MASK_LEAVE_WINDOW         : EventMask =      0x20;
pub const EVENT_MASK_POINTER_MOTION       : EventMask =      0x40;
pub const EVENT_MASK_POINTER_MOTION_HINT  : EventMask =      0x80;
pub const EVENT_MASK_BUTTON_1_MOTION      : EventMask =     0x100;
pub const EVENT_MASK_BUTTON_2_MOTION      : EventMask =     0x200;
pub const EVENT_MASK_BUTTON_3_MOTION      : EventMask =     0x400;
pub const EVENT_MASK_BUTTON_4_MOTION      : EventMask =     0x800;
pub const EVENT_MASK_BUTTON_5_MOTION      : EventMask =    0x1000;
pub const EVENT_MASK_BUTTON_MOTION        : EventMask =    0x2000;
pub const EVENT_MASK_KEYMAP_STATE         : EventMask =    0x4000;
pub const EVENT_MASK_EXPOSURE             : EventMask =    0x8000;
pub const EVENT_MASK_VISIBILITY_CHANGE    : EventMask =   0x10000;
pub const EVENT_MASK_STRUCTURE_NOTIFY     : EventMask =   0x20000;
pub const EVENT_MASK_RESIZE_REDIRECT      : EventMask =   0x40000;
pub const EVENT_MASK_SUBSTRUCTURE_NOTIFY  : EventMask =   0x80000;
pub const EVENT_MASK_SUBSTRUCTURE_REDIRECT: EventMask =  0x100000;
pub const EVENT_MASK_FOCUS_CHANGE         : EventMask =  0x200000;
pub const EVENT_MASK_PROPERTY_CHANGE      : EventMask =  0x400000;
pub const EVENT_MASK_COLOR_MAP_CHANGE     : EventMask =  0x800000;
pub const EVENT_MASK_OWNER_GRAB_BUTTON    : EventMask = 0x1000000;


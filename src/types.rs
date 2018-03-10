#![no_std]

pub type int = i32;

pub type uint = u32;
pub type ushort = u16;
pub type uchar = u8;

pub type c_char = i8;

pub type pde_t = uint;

#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

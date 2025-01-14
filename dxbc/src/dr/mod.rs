use int_enum::IntEnum;

#[repr(u32)]
#[derive(Clone, Copy, Debug, IntEnum)]
pub enum ResourceReturnType {
    NotApplicable = 0,
    UNorm = 1,
    SNorm = 2,
    SInt = 3,
    UInt = 4,
    Float = 5,
    Mixed = 6,
    Double = 7,
    Continued = 8,
}

pub mod builder;
pub mod isgn;
pub mod rdef;
pub mod shex;
pub mod stat;

pub use self::builder::*;
pub use self::isgn::*;
pub use self::rdef::*;
pub use self::shex::*;
pub use self::stat::*;

#[repr(C)]
#[derive(Debug)]
pub struct DxbcHeader {
    pub magic: [u8; 4],
    pub checksum: [u32; 4],
    _unknown: u32,
    pub size: u32,
    pub chunk_count: u32,
}

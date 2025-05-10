use plain::Plain;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RiteBinaryHeader {
    pub ident: [u8; 4],
    pub major_version: [u8; 2],
    pub minor_version: [u8; 2],
    pub size: [u8; 4],
    pub compiler_name: [u8; 4],
    pub compiler_version: [u8; 4],
}
unsafe impl Plain for RiteBinaryHeader {}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SectionMiscHeader {
    pub ident: [u8; 4],
    pub size: [u8; 4],
}
unsafe impl Plain for SectionMiscHeader {}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SectionIrepHeader {
    pub ident: [u8; 4],
    pub size: [u8; 4],

    pub rite_version: [u8; 4],
}
unsafe impl Plain for SectionIrepHeader {}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IrepRecord {
    pub size: [u8; 4],
    pub nlocals: [u8; 2],
    pub nregs: [u8; 2],
    pub rlen: [u8; 2],
    pub clen: [u8; 2],
    pub ilen: [u8; 4],
}

unsafe impl Plain for IrepRecord {}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IrepCatchHandler {
    pub type_: u8,
    pub begin: [u8; 4],
    pub end: [u8; 4],
    pub target: [u8; 4],
}

unsafe impl Plain for IrepCatchHandler {}
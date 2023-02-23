use pad_struct::pad_struct;
use memoffset::offset_of;

#[repr(C)]
#[derive(Debug)]
struct SizedStruct {
    f1: u64,
    f2: u64,
}

pad_struct!(
    #[repr(C)]
    #[derive(Debug)]
    struct Padded {
        0x100 => field: u32,
        /* 0x104 */ middle: u32,
        0xc00 => another: SizedStruct,
        /* 0xc10 */ something: u8,
        0xfff => canteven: u8,
    }
);

fn main() {
    println!("Padded size: 0x{:x}", core::mem::size_of::<Padded>());
    println!("SizedStruct size: 0x{:x}", core::mem::size_of::<SizedStruct>());
    println!("field offset: 0x{:x}", offset_of!(Padded, field));
    println!("middle offset: 0x{:x}", offset_of!(Padded, middle));
    println!("another offset: 0x{:x}", offset_of!(Padded, another));
    println!("something offset: 0x{:x}", offset_of!(Padded, something));
    println!("canteven offset: 0x{:x}", offset_of!(Padded, canteven));
}

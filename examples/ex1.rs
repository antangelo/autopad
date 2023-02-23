use memoffset::offset_of;
use pad_struct::pad_struct;

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
        /* 0x104 */ between: u32,
        0xc00 => another: SizedStruct,
        /* 0xc10 */ after_struct: u8,
        0xfff => end: u8,
    }
);

fn main() {
    println!("Padded size: \t\t0x{:x}", core::mem::size_of::<Padded>());
    println!(
        "SizedStruct size: \t0x{:x}",
        core::mem::size_of::<SizedStruct>()
    );
    println!("field offset: \t\t0x{:x}", offset_of!(Padded, field));
    println!("middle offset: \t\t0x{:x}", offset_of!(Padded, between));
    println!("another offset: \t0x{:x}", offset_of!(Padded, another));
    println!("something offset: \t0x{:x}", offset_of!(Padded, after_struct));
    println!("canteven offset: \t0x{:x}", offset_of!(Padded, end));

    assert_eq!(offset_of!(Padded, field), 0x100);
    assert_eq!(offset_of!(Padded, between), 0x104);
    assert_eq!(offset_of!(Padded, another), 0xc00);
    assert_eq!(offset_of!(Padded, end), 0xfff);
    assert_eq!(core::mem::size_of::<Padded>(), 0xfff + 1);
}

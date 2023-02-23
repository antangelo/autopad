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
        /* 0x104 */ middle: u32,
        0xc00 => another: SizedStruct,
        /* 0xc10 */ something: u8,
        0xfff => canteven: u8,
    }
);

fn main() {
    println!("Padded size: \t\t0x{:x}", core::mem::size_of::<Padded>());
    println!(
        "SizedStruct size: \t0x{:x}",
        core::mem::size_of::<SizedStruct>()
    );
    println!("field offset: \t\t0x{:x}", offset_of!(Padded, field));
    println!("middle offset: \t\t0x{:x}", offset_of!(Padded, middle));
    println!("another offset: \t0x{:x}", offset_of!(Padded, another));
    println!("something offset: \t0x{:x}", offset_of!(Padded, something));
    println!("canteven offset: \t0x{:x}", offset_of!(Padded, canteven));
}

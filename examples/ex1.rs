use pad_struct::pad_struct;

pad_struct!(
    #[derive(Debug)]
    struct Padded {
        0x100 => field: u32,
    }
);

fn main() {
    let ps = Padded { field: 10 };
    println!("ps is: {:?}", ps);
}

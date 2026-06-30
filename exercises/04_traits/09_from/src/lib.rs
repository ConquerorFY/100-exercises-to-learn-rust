// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<i32> for WrappingU32 {
    fn from(value: i32) -> Self {
        WrappingU32 {
            // value: value.into() // alternative
            value: value as u32, // using as might have issues if over the limit causing truncate (typically from large to small (eg. i64 --> i32))
        }
        // value.into()         // alternative #2
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

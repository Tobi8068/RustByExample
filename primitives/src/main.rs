fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("{}, {}", 1e4, -2.5e-3);
    println!("{}", true & false);
    println!("{}", true | false);

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);
    println!("{:?}", long_tuple);
    let pair = (1, true);
    println!("{:?}", pair);
    println!("{:?}", reverse(pair));
}

// run-rustfix

fn foo<N>(_x: N) {}

fn main() {
    foo::<usize>(42_usize);
    foo::<usize>(42_u64);
    //~^ ERROR mismatched types
    foo::<usize>(42_u32);
    //~^ ERROR mismatched types
    foo::<usize>(42_u16);
    //~^ ERROR mismatched types
    foo::<usize>(42_u8);
    //~^ ERROR mismatched types
    foo::<usize>(42_isize);
    //~^ ERROR mismatched types
    foo::<usize>(42_i64);
    //~^ ERROR mismatched types
    foo::<usize>(42_i32);
    //~^ ERROR mismatched types
    foo::<usize>(42_i16);
    //~^ ERROR mismatched types
    foo::<usize>(42_i8);
    //~^ ERROR mismatched types
    foo::<usize>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<usize>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<isize>(42_usize);
    //~^ ERROR mismatched types
    foo::<isize>(42_u64);
    //~^ ERROR mismatched types
    foo::<isize>(42_u32);
    //~^ ERROR mismatched types
    foo::<isize>(42_u16);
    //~^ ERROR mismatched types
    foo::<isize>(42_u8);
    //~^ ERROR mismatched types
    foo::<isize>(42_isize);
    foo::<isize>(42_i64);
    //~^ ERROR mismatched types
    foo::<isize>(42_i32);
    //~^ ERROR mismatched types
    foo::<isize>(42_i16);
    //~^ ERROR mismatched types
    foo::<isize>(42_i8);
    //~^ ERROR mismatched types
    foo::<isize>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<isize>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<u64>(42_usize);
    //~^ ERROR mismatched types
    foo::<u64>(42_u64);
    foo::<u64>(42_u32);
    //~^ ERROR mismatched types
    foo::<u64>(42_u16);
    //~^ ERROR mismatched types
    foo::<u64>(42_u8);
    //~^ ERROR mismatched types
    foo::<u64>(42_isize);
    //~^ ERROR mismatched types
    foo::<u64>(42_i64);
    //~^ ERROR mismatched types
    foo::<u64>(42_i32);
    //~^ ERROR mismatched types
    foo::<u64>(42_i16);
    //~^ ERROR mismatched types
    foo::<u64>(42_i8);
    //~^ ERROR mismatched types
    foo::<u64>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<u64>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<i64>(42_usize);
    //~^ ERROR mismatched types
    foo::<i64>(42_u64);
    //~^ ERROR mismatched types
    foo::<i64>(42_u32);
    //~^ ERROR mismatched types
    foo::<i64>(42_u16);
    //~^ ERROR mismatched types
    foo::<i64>(42_u8);
    //~^ ERROR mismatched types
    foo::<i64>(42_isize);
    //~^ ERROR mismatched types
    foo::<i64>(42_i64);
    foo::<i64>(42_i32);
    //~^ ERROR mismatched types
    foo::<i64>(42_i16);
    //~^ ERROR mismatched types
    foo::<i64>(42_i8);
    //~^ ERROR mismatched types
    foo::<i64>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<i64>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<u32>(42_usize);
    //~^ ERROR mismatched types
    foo::<u32>(42_u64);
    //~^ ERROR mismatched types
    foo::<u32>(42_u32);
    foo::<u32>(42_u16);
    //~^ ERROR mismatched types
    foo::<u32>(42_u8);
    //~^ ERROR mismatched types
    foo::<u32>(42_isize);
    //~^ ERROR mismatched types
    foo::<u32>(42_i64);
    //~^ ERROR mismatched types
    foo::<u32>(42_i32);
    //~^ ERROR mismatched types
    foo::<u32>(42_i16);
    //~^ ERROR mismatched types
    foo::<u32>(42_i8);
    //~^ ERROR mismatched types
    foo::<u32>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<u32>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<i32>(42_usize);
    //~^ ERROR mismatched types
    foo::<i32>(42_u64);
    //~^ ERROR mismatched types
    foo::<i32>(42_u32);
    //~^ ERROR mismatched types
    foo::<i32>(42_u16);
    //~^ ERROR mismatched types
    foo::<i32>(42_u8);
    //~^ ERROR mismatched types
    foo::<i32>(42_isize);
    //~^ ERROR mismatched types
    foo::<i32>(42_i64);
    //~^ ERROR mismatched types
    foo::<i32>(42_i32);
    foo::<i32>(42_i16);
    //~^ ERROR mismatched types
    foo::<i32>(42_i8);
    //~^ ERROR mismatched types
    foo::<i32>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<i32>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<u16>(42_usize);
    //~^ ERROR mismatched types
    foo::<u16>(42_u64);
    //~^ ERROR mismatched types
    foo::<u16>(42_u32);
    //~^ ERROR mismatched types
    foo::<u16>(42_u16);
    foo::<u16>(42_u8);
    //~^ ERROR mismatched types
    foo::<u16>(42_isize);
    //~^ ERROR mismatched types
    foo::<u16>(42_i64);
    //~^ ERROR mismatched types
    foo::<u16>(42_i32);
    //~^ ERROR mismatched types
    foo::<u16>(42_i16);
    //~^ ERROR mismatched types
    foo::<u16>(42_i8);
    //~^ ERROR mismatched types
    foo::<u16>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<u16>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<i16>(42_usize);
    //~^ ERROR mismatched types
    foo::<i16>(42_u64);
    //~^ ERROR mismatched types
    foo::<i16>(42_u32);
    //~^ ERROR mismatched types
    foo::<i16>(42_u16);
    //~^ ERROR mismatched types
    foo::<i16>(42_u8);
    //~^ ERROR mismatched types
    foo::<i16>(42_isize);
    //~^ ERROR mismatched types
    foo::<i16>(42_i64);
    //~^ ERROR mismatched types
    foo::<i16>(42_i32);
    //~^ ERROR mismatched types
    foo::<i16>(42_i16);
    foo::<i16>(42_i8);
    //~^ ERROR mismatched types
    foo::<i16>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<i16>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<u8>(42_usize);
    //~^ ERROR mismatched types
    foo::<u8>(42_u64);
    //~^ ERROR mismatched types
    foo::<u8>(42_u32);
    //~^ ERROR mismatched types
    foo::<u8>(42_u16);
    //~^ ERROR mismatched types
    foo::<u8>(42_u8);
    foo::<u8>(42_isize);
    //~^ ERROR mismatched types
    foo::<u8>(42_i64);
    //~^ ERROR mismatched types
    foo::<u8>(42_i32);
    //~^ ERROR mismatched types
    foo::<u8>(42_i16);
    //~^ ERROR mismatched types
    foo::<u8>(42_i8);
    //~^ ERROR mismatched types
    foo::<u8>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<u8>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<i8>(42_usize);
    //~^ ERROR mismatched types
    foo::<i8>(42_u64);
    //~^ ERROR mismatched types
    foo::<i8>(42_u32);
    //~^ ERROR mismatched types
    foo::<i8>(42_u16);
    //~^ ERROR mismatched types
    foo::<i8>(42_u8);
    //~^ ERROR mismatched types
    foo::<i8>(42_isize);
    //~^ ERROR mismatched types
    foo::<i8>(42_i64);
    //~^ ERROR mismatched types
    foo::<i8>(42_i32);
    //~^ ERROR mismatched types
    foo::<i8>(42_i16);
    //~^ ERROR mismatched types
    foo::<i8>(42_i8);
    foo::<i8>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<i8>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<f64>(42_usize);
    //~^ ERROR mismatched types
    foo::<f64>(42_u64);
    //~^ ERROR mismatched types
    foo::<f64>(42_u32);
    //~^ ERROR mismatched types
    foo::<f64>(42_u16);
    //~^ ERROR mismatched types
    foo::<f64>(42_u8);
    //~^ ERROR mismatched types
    foo::<f64>(42_isize);
    //~^ ERROR mismatched types
    foo::<f64>(42_i64);
    //~^ ERROR mismatched types
    foo::<f64>(42_i32);
    //~^ ERROR mismatched types
    foo::<f64>(42_i16);
    //~^ ERROR mismatched types
    foo::<f64>(42_i8);
    //~^ ERROR mismatched types
    foo::<f64>(42.0_f64);
    foo::<f64>(42.0_f32);
    //~^ ERROR mismatched types

    foo::<f32>(42_usize);
    //~^ ERROR mismatched types
    foo::<f32>(42_u64);
    //~^ ERROR mismatched types
    foo::<f32>(42_u32);
    //~^ ERROR mismatched types
    foo::<f32>(42_u16);
    //~^ ERROR mismatched types
    foo::<f32>(42_u8);
    //~^ ERROR mismatched types
    foo::<f32>(42_isize);
    //~^ ERROR mismatched types
    foo::<f32>(42_i64);
    //~^ ERROR mismatched types
    foo::<f32>(42_i32);
    //~^ ERROR mismatched types
    foo::<f32>(42_i16);
    //~^ ERROR mismatched types
    foo::<f32>(42_i8);
    //~^ ERROR mismatched types
    foo::<f32>(42.0_f64);
    //~^ ERROR mismatched types
    foo::<f32>(42.0_f32);

    foo::<u32>(42_u8 as u16);
    //~^ ERROR mismatched types
    foo::<i32>(-42_i8);
    //~^ ERROR mismatched types
}

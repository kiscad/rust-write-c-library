use std::slice;

///# Safety
#[no_mangle]
pub unsafe extern "C" fn reverse_sequence(seq_lengths: *const i64, input_dims: u32) -> u32 {
    let seq_lengths_ = slice::from_raw_parts(seq_lengths, input_dims as usize);
    println!("{:?}", seq_lengths_);
    1
}

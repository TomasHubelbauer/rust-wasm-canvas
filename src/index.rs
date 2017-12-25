use std::mem;
use std::slice;
use std::os::raw::c_void;

// We need to provide an (empty) main function, as the target currently is compiled as a binary.
#[allow(dead_code)]
fn main() {}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
  let mut buf = Vec::with_capacity(size);
  let ptr = buf.as_mut_ptr();
  mem::forget(buf);
  return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
  unsafe {
    let _buf = Vec::from_raw_parts(ptr, 0, cap);
  }
}

#[no_mangle]
pub fn fill(pointer: *mut u8, width: usize, height: usize, time: usize /* f64 */) {
  let length = width * height * 4; // RGBA
  let data = unsafe { slice::from_raw_parts_mut(pointer, length) };

  for channel_index in 0..length {
    let pixel_index = channel_index / 4;
    let y: usize = pixel_index / width;
    let x: usize = pixel_index % width;

    match channel_index % 4 {
      0 => data[channel_index] = ((time + x) % 255) as u8, // R
      1 => data[channel_index] = ((time + y) % 255) as u8, // G
      2 => data[channel_index] = 128, // B
      3 => data[channel_index] = 255, // A
      _ => panic!("i % 4 not within 1 and 3"),
    }
  }
}

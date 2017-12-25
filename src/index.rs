use std::mem;
use std::slice;
use std::os::raw::c_void;

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
pub fn fill(pointer: *mut u8, width: usize, height: usize, time: f64) {
  let data = unsafe {
    slice::from_raw_parts_mut(pointer, width * height * 4) // RGBA
  };

  for pixel in 0..(width * height) {
    let y: usize = pixel / width;
    let x: usize = pixel % width;
    let index = pixel * 4;

    data[index + 0] = data[index + 0] + (time % 30.) as u8; // R
    data[index + 1] = data[index + 1] + (time % 20.) as u8; // G
    data[index + 2] = data[index + 2] + (time % 10.) as u8; // B
    data[index + 3] = 255; // A
  }
}

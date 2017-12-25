#![feature(iterator_step_by)]

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
#[allow(unused_variables)]
pub fn fill(
  pointer: *mut u8, width: usize, height: usize, time: f64,
  center_x: f64, center_y: f64, center_z: f64,
  camera_x: f64, camera_y: f64, camera_z: f64,
  viewer_x: f64, viewer_y: f64, viewer_z: f64,
  ) {
  let length = width * height * 4;
  let data = unsafe { slice::from_raw_parts_mut(pointer, length) }; // RGBA

  // Clear the canvas.
  for index in 0..length { data[index] = if index % 4 == 3 { 255 } else { 0 }; }

  let camera = Point3D { x: camera_x, y: camera_y, z: camera_z };
  let camera_rotation = Point3D { x: 0., y: 0., z: 0. };
  let viewer = Point3D { x: viewer_x, y: viewer_y, z: viewer_z };

  // TODO: Introduce z-buffer.

  for x in (-10..10).step_by(10) {
    for y in -10..10 {
      for z in -10..10 {
        // WTF!
        let point3d = Point3D { x: (x as f64), y: (y as f64), z: (z as f64) };
        let point2d = project_3d_to_2d(point3d, &camera, &camera_rotation, &viewer);
        let a = point2d.x as usize;
        let b = point2d.y as usize;
        if a <= width && b <= height {
          let index = b * width + a;
          data[index + 0] = 255; // R
          data[index + 1] = 255; // G
          data[index + 2] = 255; // B
          data[index + 3] = 255; // A
        }
      }
    }
  }
}

struct Point2D {
  x: f64,
  y: f64,
}

struct Point3D {
  x: f64,
  y: f64,
  z: f64
}

fn project_3d_to_2d(a: Point3D, c: &Point3D, o: &Point3D, e: &Point3D) -> Point2D {
  // https://en.wikipedia.org/wiki/3D_projection#Perspective_projection
  let x = a.x - c.x;
  let y = a.y - c.y;
  let z = a.z - c.z;

  let szy = o.z.sin() * y;
  let czx = o.z.cos() * x;
  let syz = o.y.sin() * z;
  let h1 = szy + czx;

  let d_x = o.y.cos() * h1 - syz;

  let cyz = o.y.cos() * z;
  let h2 = cyz + (o.y.sin() * h1);

  let czy = o.z.cos() * y;
  let szx = o.z.sin() * x;
  let h3 = czy - szx;

  let d_y = (o.x.sin() * h2) + (o.x.cos() * h3);
  let d_z = (o.x.cos() * h2) - (o.x.sin() * h3);

  let h4 = e.z / d_z;

  let b_x = (h4 * d_x) - e.x;
  let b_y = (h4 * d_y) - e.y;

  return Point2D { x: b_x, y: b_y };
}

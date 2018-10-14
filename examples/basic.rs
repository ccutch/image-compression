extern crate cc_compression;


fn main() {
  let mut result = cc_compression::compress_image("examples/test_images/high-contrast.png", 10).expect("Failed to do stuff");
  result.reduce_colors(2);
  println!("{:?}", result);
  // for color in result.colors {
  //   println!("{}", color);
  // }
}

// fn main() -> Result<(), Box<std::error::Error>> {
//   let img = image::open("examples/test_images/keyboard.jpg")?;
//   println!("dimensions {:?}", img.dimensions());
//   let mut max_r = 0;
//   let mut max_b = 0;
//   let mut max_g = 0;

//   for i in 0..4000 {
//     let pixel = img.get_pixel(i, i);
//     let [r, g, b, _] = pixel.data;

//     if r > max_r {
//       max_r = r;
//     }
//     if r > max_g {
//       max_g = g;
//     }
//     if r > max_b {
//       max_b = b;
//     }
//   }
//   println!("{} {} {}", max_r, max_g, max_b);
//   Ok(())
// }
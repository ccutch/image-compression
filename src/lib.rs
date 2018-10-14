extern crate image;

use image::GenericImageView;
use std::collections::HashSet;
use std::fmt;


// trait Loadable {}
// trait Saveable {}
// enum Formats { // : loadable + Saveable
//     PNG(),
//     JPG(),
//     CCCC(),
// }



#[derive(Debug)]
pub struct CompressionError;

impl From<image::ImageError> for CompressionError {
    fn from(_other: image::ImageError) -> CompressionError {
        CompressionError
    }
}


#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Color(i32, i32, i32);

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        Color(r, g, b)
    }

    fn from_average(lhs: &Color, rhs: &Color) -> Color {
        let rdiff = (lhs.0 + rhs.0) / 2;
        let gdiff = (lhs.1 + rhs.1) / 2;
        let bdiff = (lhs.2 + rhs.2) / 2;

        Color::new(rdiff, gdiff, bdiff) 
    }

    fn difference(&self, other: &Color) -> f32 {
        let rdiff = self.0 - other.0;
        let gdiff = self.1 - other.1;
        let bdiff = self.2 - other.2;

        ((rdiff.pow(2) + gdiff.pow(2) + bdiff.pow(2)) as f32).sqrt()
    }

}



impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:X}{:X}{:X}", self.0, self.1, self.2)
    }
}

// impl PartialEq for Color {
//     fn eq(&self, other: &Color) -> bool {
//         self.0 == other.0 && self.1 == other.1 && self.2 == other.2
//     }
// }

// impl Eq for Color {}



#[derive(Default, Debug)]
pub struct ColorCell {
    size: (u32, u32),
    pub colors: HashSet<Color>,
}

impl ColorCell {
    fn get_most_similar_colors(&self) -> (Color, Color) {
        let mut lowest_pair: (Color, Color) = (Color::default(), Color::default());
        let mut lowest_difference = std::f32::MAX;
        for color in &self.colors {
            for comp_color in &self.colors { // Fix later
                if color != comp_color {
                    let diff = color.difference(&comp_color);
                    if diff < lowest_difference {
                        lowest_difference = diff;
                        lowest_pair = (Color::new(color.0, color.1, color.2), Color::new(comp_color.0, comp_color.1, comp_color.2));
                    }
                }
            }
        }

        lowest_pair
    }

    fn merge_closest_colors(&mut self) {
        let (color1, color2)= self.get_most_similar_colors();
        let new_color = Color::from_average(&color1, &color2);

        self.colors.remove(&color1);
        self.colors.remove(&color2);
        self.colors.insert(new_color);
    }

    pub fn reduce_colors(&mut self, desired: usize) {
        // let mut new_colors = HashSet::<Color>::new();
        while self.colors.len() > desired {
            println!("{}", self.colors.len());
            self.merge_closest_colors();
        }
    }
}

/// Compress image for path. Basic function that should be removed in final
/// version
pub fn compress_image(path: &str, res: u32) -> Result<ColorCell, CompressionError> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    println!("{} {}", width, height);
    // if width % res != 0 {
    //     eprintln!("Width compression error");
    //     return Err(CompressionError);
    // }
    // if height % res != 0 {
    //     eprintln!("Height compression error");
    //     return Err(CompressionError);
    // }



    let offset = 0;
    let mut cell = ColorCell::default();
    cell.size = (width / res, height / res);
    for x in offset..(offset + res) {
        for y in offset..(offset + res) {
            let pixel = img.get_pixel(x, y);
            let [red, green, blue, _alpha] = pixel.data;
            cell.colors.insert(Color(red as i32, green as i32, blue as i32));
        }
    }

    // // println!("{:#?}", cell);
    // for color1 in &cell.colors {
    //     for color2 in &cell.colors {
    //         if color1 != color2 {
    //             println!("{}", color1.difference(&color2));
    //         }
    //     }
    // }
    Ok(cell)
}





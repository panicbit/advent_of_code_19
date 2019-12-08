#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2019, 08, 1)]
fn main(input: &str) -> usize {
    let width = 25;
    let height = 6;
    let stride = width * height;
    let layers: Vec<Layer> =
        input
        .trim()
        .split("")
        .filter(|&n| n != "")
        .map(|n| n.parse::<i32>().unwrap())
        .chunks(stride)
        .into_iter()
        .map(|layer| Layer::parse(layer))
        .collect();

    let layer_with_fewest_zeros =
        layers
        .iter()
        .min_by_key(|layer|
            layer.count_pixels(|pixel| pixel == 0)
        )
        .unwrap();

    let num_1_digits =
        layer_with_fewest_zeros
        .count_pixels(|pixel| pixel == 1);

    let num_2_digits =
        layer_with_fewest_zeros
        .count_pixels(|pixel| pixel == 2);

    num_1_digits * num_2_digits
}

struct Layer {
    pixels: Vec<i32>,
}

impl Layer {
    fn parse(pixels: impl Iterator<Item = i32>) -> Self {
        Layer {
            pixels: pixels.collect()
        }
    }

    fn pixels(&self) -> impl Iterator<Item = i32> + '_ {
        self
        .pixels
        .iter()
        .copied()
    }

    fn count_pixels(&self, mut f: impl FnMut(i32) -> bool) -> usize {
        self
        .pixels()
        .filter(|&pixel| f(pixel))
        .count()
    }
}

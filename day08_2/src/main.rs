#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2019, 08, 2)]
fn main(input: &str) -> usize {
    let width = 25;
    let height = 6;
    let stride = width * height;
    let layers: Vec<Layer> =
        input
        .chars()
        .map(|c| c.to_digit(10))
        .flatten()
        .chunks(stride)
        .into_iter()
        .map(|layer| Layer::parse(layer))
        .collect();

    let merged_layers =
        layers
        .into_iter()
        .rev()
        .fold1(|a, b| a.apply(&b))
        .unwrap();

    let output =
        merged_layers
        .pixels()
        .map(|pixel| match pixel {
            1 => '█',
            _ => ' ',
        })
        .chunks(width)
        .into_iter()
        .map(|mut row| row.join(""))
        .join("\n");

    println!("{}", output);

    panic!("HUMAN REQUIRED")
}

struct Layer {
    pixels: Vec<u32>,
}

impl Layer {
    fn parse(pixels: impl Iterator<Item = u32>) -> Self {
        Layer {
            pixels: pixels.collect()
        }
    }

    fn pixels(&self) -> impl Iterator<Item = u32> + '_ {
        self
        .pixels
        .iter()
        .copied()
    }

    fn apply(&self, layer: &Layer) -> Self {
        let pixels =
            self
            .pixels()
            .zip(layer.pixels())
            .map(|pixels| match pixels {
                (a, 2) => a,
                (_, b) => b,
            });

        Self {
            pixels: pixels.collect(),
        }
    }
}

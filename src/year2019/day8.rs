use crate::utils::*;
use itertools::Itertools;
use rayon::iter::*;
use std::str;

pub fn decode_layers(data: &str, width: usize, height: usize) -> Vec<Vec<i32>> {
    let pixels_per_layer = width * height;
    data.as_bytes()
        .iter()
        .chunks(pixels_per_layer)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|byte| str::from_utf8(&[*byte]).unwrap().parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2019/data/day8input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                const width: usize = 25;
                const height: usize = 6;
                let layers = decode_layers(&contents, width, height);
                let digit_counts: Vec<_> = layers
                    .iter()
                    .map(|layer| {
                        layer.iter().fold([0; 10], |mut acc, x| {
                            acc[*x as usize] += 1;
                            acc
                        })
                    })
                    .collect();
                let min = digit_counts.iter().min_by(|x, y| x[0].cmp(&y[0])).unwrap();
                dbg!(min[1] * min[2]);

                let mut acc = [[2; width]; height];
                dbg!(layers.len());

                acc = layers.iter().fold(acc, |mut pic, layer| {
                    for (y, row) in layer.iter().chunks(width).into_iter().enumerate() {
                        for (x, px) in row.enumerate() {
                            if pic[y][x] == 2 {
                                dbg!(y, x);
                                pic[y][x] = *px;
                            }
                        }
                    }

                    pic
                });
                //let image =
            }
        }
    }
}

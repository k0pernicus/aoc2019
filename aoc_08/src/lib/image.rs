type Layer = Vec<u8>;

#[derive(Debug)]
struct Image {
    pub layers: Vec<Layer>,
    height: usize,
    width: usize,
}

impl Image {
    pub fn new(data: &str, height: usize, width: usize) -> Image {
        let mut data_values = data.trim().chars().into_iter().peekable();
        let mut layers: Vec<Layer> = Vec::new();
        while data_values.peek().is_some() {
            layers.push(
                data_values
                    .by_ref()
                    .take(height * width)
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>(),
            );
        }
        Image {
            layers: layers,
            height: height,
            width: width,
        }
    }

    pub fn count_with_digit(&self, digit: u8) -> Vec<usize> {
        self.layers
            .iter()
            .map(|l| l.iter().filter(|&n| *n == digit).count())
            .collect::<Vec<usize>>()
    }

    pub fn decode(&self) -> Vec<u8> {
        let mut decoded_image: Vec<u8> = vec![2u8; self.height * self.width];
        for layer in self.layers.iter().rev() {
            for (index, pixel) in layer.into_iter().enumerate() {
                if pixel != &2u8 {
                    decoded_image[index] = *pixel;
                }
            }
        }
        decoded_image
    }
}

pub fn solve_part_1(data: &str, height: usize, width: usize) -> usize {
    let image = Image::new(data, height, width);
    let zero_vectors = image.count_with_digit(0);
    let vector_with_min_0: usize = *zero_vectors.iter().min_by(|x, y| x.cmp(y)).unwrap();
    let layer: Vec<u8> = image.layers[zero_vectors
        .iter()
        .position(|&r| r == vector_with_min_0)
        .unwrap()]
    .clone();
    let number_of_one = layer.iter().filter(|&n| *n == 1).count();
    let number_of_two = layer.iter().filter(|&n| *n == 2).count();
    number_of_one * number_of_two
}

pub fn solve_part_2(data: &str, height: usize, width: usize) {
    let image = Image::new(data, height, width);
    let decoded_image = image.decode();
    for i in 0..height {
        let v = &decoded_image[i * width..(i + 1) * width];
        println!(
            "{:?}",
            v.iter()
                .map(|c| {
                    if *c == 1u8 {
                        return '-';
                    } else {
                        return ' ';
                    }
                })
                .collect::<String>()
        );
    }
}

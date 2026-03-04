use std::collections::HashMap;

use burn::{
    backend::Wgpu,
    tensor::{Tensor, TensorData},
};

type Backend = Wgpu;

pub struct Bigram {
    pub data: Vec<char>,
}

impl Bigram {
    pub fn new(list: Vec<String>) -> Self {
        let sep = '.';
        let total_chars: usize = list.iter().map(|s| s.len() + 2).sum();
        let mut newvec = Vec::with_capacity(total_chars);
        for word in list {
            newvec.push(sep);
            newvec.extend(word.chars());
            newvec.push(sep);
        }
        Bigram { data: newvec }
    }

    pub fn to_tensor(&self) -> Tensor<Backend, 2> {
        let map: HashMap<char, usize> = std::iter::once('.')
            .chain('a'..='z')
            .enumerate()
            .map(|(i, c)| (c, i))
            .collect();

        let mut counts = vec![0i32; 27 * 27];

        for (&ch1, &ch2) in self.pairs() {
            if let (Some(&i), Some(&j)) = (map.get(&ch1), map.get(&ch2)) {
                counts[i * 27 + j] += 1;
            }
        }
        let tensor_data = TensorData::new(counts, [27, 27]);
        Tensor::<Backend, 2>::from_data(tensor_data, &Default::default())
    }

    fn pairs(&self) -> impl Iterator<Item = (&char, &char)> {
        self.data.iter().zip(self.data.iter().skip(1))
    }
}

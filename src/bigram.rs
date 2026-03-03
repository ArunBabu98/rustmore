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

    pub fn pairs(&self) -> impl Iterator<Item = (&char, &char)> {
        self.data.iter().zip(self.data.iter().skip(1))
    }
}

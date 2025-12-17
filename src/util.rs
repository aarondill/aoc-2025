pub trait IterCombinationsPairs {
    type Item;
    fn iter_combinations(self) -> impl Iterator<Item = Self::Item>;
    fn iter_permutations(self) -> impl Iterator<Item = Self::Item>;
}

impl<'a, T> IterCombinationsPairs for &'a [T] {
    type Item = [&'a T; 2];

    fn iter_combinations(self) -> impl Iterator<Item = Self::Item> {
        self.iter().enumerate().flat_map(|(i, p)| self.iter().skip(i + 1).map(move |q| [p, q]))
    }

    fn iter_permutations(self) -> impl Iterator<Item = Self::Item> {
        self.iter().flat_map(|p| self.iter().map(move |q| [p, q]))
    }
}

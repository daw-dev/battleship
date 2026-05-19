use std::ops::{Index, IndexMut};

pub struct Grid<const W: usize, const H: usize, T> {
    matrix: [[T; W]; H],
}

impl<const W: usize, const H: usize, T> Grid<W, H, T> {
    pub fn new() -> Self
    where
        T: Default,
    {
        Self {
            matrix: std::array::from_fn(|_| std::array::from_fn(|_| T::default())),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.matrix.get(H - y - 1)?.get(x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.matrix.get_mut(H - y - 1)?.get_mut(x)
    }
}

impl<const W: usize, const H: usize, T> IndexMut<(usize, usize)> for Grid<W, H, T>
{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut(x, y).unwrap()
    }
}

impl<const W: usize, const H: usize, T> Index<(usize, usize)> for Grid<W, H, T>
{
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(x, y).unwrap()
    }
}

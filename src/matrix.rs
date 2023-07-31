use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Clone)]
pub struct Matrix<T> {
    pub vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn to_index(&self, (x, y): (usize, usize)) -> usize {
        if !self.test_coords(x, y) {
            panic!("Bad coords: {x},{y}");
        }

        y * self.width + x
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn to_coords(&self, idx: usize) -> (usize, usize) {
        if !idx < self.vec.len() {
            panic!("Out of bounds");
        }
        let x = idx % self.width;
        let y = idx / self.width;
        (x, y)
    }

    pub fn test_coords(&self, x: usize, y: usize) -> bool {
        if x >= self.width {
            return false;
        }
        if y >= self.height {
            return false;
        }
        true
    }
}

impl<T: Copy> Matrix<T> {
    pub fn from_vecs(vecs: Vec<Vec<T>>) -> Self {
        let width = vecs[0].len();
        let height = vecs.len();
        let mut ret = Self::new_default(width, height, vecs[0][0]);
        for y in 0..height {
            for x in 0..width {
                ret[(x, y)] = vecs[y][x]
            }
        }
        ret
    }

    pub fn new_default(width: usize, height: usize, value: T) -> Matrix<T> {
        let mut vec = Vec::with_capacity(height * width);

        for _ in 0..width * height {
            vec.push(value);
        }

        Matrix { vec, width, height }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if self.test_coords(x, y) {
            Some(self[(x, y)])
        } else {
            None
        }
    }

    pub fn get_or(&self, x: usize, y: usize, default: T) -> T {
        if let Some(ret) = self.get(x, y) {
            ret
        } else {
            default
        }
    }

    pub fn draw_with(&self, func: &dyn Fn(&T) -> String) {
        let mut repr = String::new();
        for i in 0..self.vec.len() {
            if i % self.width == 0 {
                repr += "\n";
            }
            let val = self.vec[i];
            repr += &func(&val);
        }
        println!("{repr}");
    }
}

impl<T: Copy + Default> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_default(width, height, T::default())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        &self.vec[self.to_index(coords)]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        let i = self.to_index(coords);
        &mut self.vec[i]
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in 0..self.vec.capacity() {
            if i % self.width == 0 {
                writeln!(f)?
            }
            self.vec[i].fmt(f)?;
        }
        Ok(())
    }
}

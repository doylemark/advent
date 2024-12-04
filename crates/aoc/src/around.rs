pub const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), // northwest
    (0, -1),  // north
    (1, -1),  // northeast
    (-1, 0),  // west
    (1, 0),   // east
    (1, 1),   // southeast
    (0, 1),   // south
    (-1, 1),  // southwest
];

pub struct IterAround<'v, T> {
    i: usize,
    j: usize,
    idx: usize,
    offsets: Vec<(i32, i32)>,
    vec: &'v Vec<Vec<T>>,
}

pub trait Around<T> {
    fn around(&self, i: usize, j: usize) -> IterAround<'_, T>;

    fn around_offsets(&self, i: usize, j: usize, offsets: Vec<(i32, i32)>) -> IterAround<'_, T>;
}

impl<T> Around<T> for Vec<Vec<T>> {
    fn around(&self, i: usize, j: usize) -> IterAround<'_, T> {
        IterAround {
            i,
            j,
            idx: 0,
            offsets: OFFSETS.to_vec(),
            vec: self,
        }
    }

    fn around_offsets(&self, i: usize, j: usize, offsets: Vec<(i32, i32)>) -> IterAround<'_, T> {
        IterAround {
            i,
            j,
            idx: 0,
            offsets,
            vec: self,
        }
    }
}

impl<'v, T> Iterator for IterAround<'v, T> {
    type Item = &'v T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.offsets.len() {
            return None;
        }

        let (offset_x, offset_y) = self.offsets.get(self.idx)?;

        let x = offset_x + self.j as i32;
        let y = offset_y + self.i as i32;

        if x < 0 || y < 0 {
            self.idx += 1;
            return self.next();
        }

        self.idx += 1;
        self.vec
            .get(y as usize)
            .map(|row| row.get(x as usize))
            .flatten()
            .or_else(|| self.next())
    }
}

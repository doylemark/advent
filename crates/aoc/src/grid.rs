use core::fmt::Display;

pub const OFFSETS: [(i32, i32); 8] = [
    // north
    (0i32, -1i32),
    // south
    (0, 1),
    // east
    (1, 0),
    // west
    (-1, 0),
    // northeast
    (1, -1),
    // northwest
    (-1, -1),
    // southeast
    (1, 1),
    // southwest
    (-1, 1),
];

#[derive(Debug)]
pub struct Item<T, U>
where
    T: ToString,
{
    pub label: T,
    pub data: U,
}

#[derive(Debug)]
pub struct Grid<T, U>
where
    T: ToString,
{
    pub inner: Vec<Vec<Item<T, U>>>,
}

pub struct IterGrid<'a, T, U>
where
    T: ToString,
{
    pub inner: &'a Grid<T, U>,
    pub offsets: Vec<(i32, i32)>,
    pub offset_idx: usize,
    pub i: usize,
    pub j: usize,
}

impl<T, U> Display for Grid<T, U>
where
    T: ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        let mut max_width = 0;
        let mut max_row_width = 0;

        for row in &self.inner {
            if row.len() > max_row_width {
                max_row_width = row.len();
            }

            for item in row.iter() {
                if item.label.to_string().len() > max_width {
                    max_width = item.label.to_string().len();
                }
            }
        }

        for row in &self.inner {
            for i in 0..max_row_width {
                let item = row
                    .get(i)
                    .map(|t| t.label.to_string())
                    .unwrap_or("".to_string());
                let spacing = (0..max_width.saturating_sub(item.to_string().len()))
                    .map(|_| ' ')
                    .collect::<String>();

                out.push_str(&format!("{item}{spacing}|"));
            }

            out.push_str("\n");
        }
        write!(f, "{out}")
    }
}

impl<T, U> Default for Grid<T, U>
where
    T: Display,
{
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T, U> Grid<T, U>
where
    T: Display,
{
    pub fn add(&mut self, item: Item<T, U>) {
        let len = self.inner.len();
        self.inner[len - 1].push(item)
    }

    pub fn add_row(&mut self) {
        self.inner.push(vec![]);
    }

    pub fn iter_around<O>(&self, i: usize, j: usize, offsets: O) -> IterGrid<T, U>
    where
        O: IntoIterator<Item = (i32, i32)>,
    {
        IterGrid {
            inner: self,
            offset_idx: 0,
            offsets: offsets.into_iter().collect::<Vec<_>>(),
            i,
            j,
        }
    }

    pub fn visit_around<F, I>(&self, y: usize, x: usize, offsets: I, mut visit: F)
    where
        F: FnMut(&Item<T, U>),
        I: IntoIterator<Item = (i32, i32)>,
    {
        for (offset_x, offset_y) in offsets.into_iter() {
            let x = match offset_x.checked_add(x as i32) {
                Some(v) => v,
                None => continue,
            };
            let y = match offset_y.checked_add(y as i32) {
                Some(v) => v,
                None => continue,
            };

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if let Some(item) = self.inner.get(y).map(|row| row.get(x).to_owned()).flatten() {
                visit(item);
            }
        }
    }
}

impl<'a, T, U> Iterator for IterGrid<'a, T, U>
where
    T: Display,
{
    type Item = &'a Item<T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        let (offset_x, offset_y) = self.offsets.get(self.offset_idx)?;

        let x = offset_x.checked_add(self.j as i32)?;
        let y = offset_y.checked_add(self.j as i32)?;

        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        self.offset_idx += 1;
        self.inner.inner.get(y).map(|row| row.get(x)).flatten()
    }
}

use core::fmt::Display;

#[derive(Debug)]
pub struct Item<T, U>
where
    T: ToString,
{
    pub label: T,
    pub data: U,
}

pub struct Grid<T, U>
where
    T: ToString,
{
    pub inner: Vec<Vec<Item<T, U>>>,
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

    pub fn visit_around<F>(&self, y: usize, x: usize, mut visit: F)
    where
        F: FnMut(&Item<T, U>),
    {
        for (offset_x, offset_y) in [
            (-1i32, -1i32),
            (-0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
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

            if let Some(item) = self.inner.get(x).map(|row| row.get(y).to_owned()).flatten() {
                visit(item);
            }
        }
    }
}

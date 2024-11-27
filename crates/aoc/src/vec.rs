pub fn visit_offsets<T, F>(vec: &Vec<Vec<T>>, i: usize, j: usize, mut visit: F)
where
    F: FnMut(&T),
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
        let x = match offset_x.checked_add(j as i32) {
            Some(v) => v,
            None => continue,
        };
        let y = match offset_y.checked_add(i as i32) {
            Some(v) => v,
            None => continue,
        };

        if x < 0 || y < 0 {
            continue;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(item) = vec.get(x).map(|row| row.get(y).to_owned()).flatten() {
            visit(item);
        }
    }
}

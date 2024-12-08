use super::point::Point;

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<char> {
    pub fn parse(input: &str) -> Self {
        let mut data = vec![];
        let mut width = 0;
        let mut height = 0;
        for line in input.lines() {
            let line = line.trim();
            width = line.len();
            height += 1;
            data.extend(line.chars());
        }
        Self {
            data,
            width,
            height,
        }
    }
}

impl Grid<u8> {
    pub fn parse_bytes(input: &str) -> Self {
        let mut data = Vec::with_capacity(input.as_bytes().len());

        let mut width = 0;
        let mut height = 0;

        for &b in input.as_bytes() {
            if b == b'\n' {
                height += 1;
            } else {
                if height == 0 {
                    width += 1;
                }

                data.push(b);
            }
        }

        Self {
            data,
            width,
            height,
        }
    }

    pub fn display(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self[Point::new(x, y)];
                s.push(if c == 0 {
                    '.'
                } else if c < 10 {
                    (b'0' + c) as char
                } else {
                    c as char
                });
            }
            s.push('\n');
        }
        s
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn insert_row(&mut self, y: usize, element: T) {
        self.data
            .splice(y * self.width..y * self.width, vec![element; self.width]);
        self.height += 1;
    }

    pub fn insert_col(&mut self, x: usize, element: T) {
        self.width += 1;
        for y in 0..self.height {
            self.data.insert(y * self.width + x, element.clone());
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn find(&self, predicate: impl Fn(&T) -> bool) -> Option<Point<usize>> {
        self.data.iter().position(predicate).map(|i| {
            let x = i % self.width;
            let y = i / self.width;
            Point::new(x, y)
        })
    }

    #[inline]
    pub fn get(&self, point: impl Into<Point<usize>>) -> Option<&T> {
        let point = point.into();
        if point.x >= self.width || point.y >= self.height {
            None
        } else {
            Some(&self.data[point.y * self.width + point.x])
        }
    }

    #[inline]
    pub fn get_mut(&mut self, point: impl Into<Point<usize>>) -> Option<&mut T> {
        let point = point.into();
        if point.x >= self.width || point.y >= self.height {
            None
        } else {
            Some(&mut self.data[point.y * self.width + point.x])
        }
    }

    #[inline]
    pub fn try_get(&self, point: impl TryInto<Point<usize>>) -> Option<&T> {
        let point = point.try_into().ok()?;
        if point.x >= self.width || point.y >= self.height {
            None
        } else {
            Some(&self.data[point.y * self.width + point.x])
        }
    }

    #[inline]
    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

impl<T, P> std::ops::Index<P> for Grid<T>
where
    P: Into<Point<usize>>,
{
    type Output = T;

    fn index(&self, point: P) -> &Self::Output {
        self.get(point.into()).unwrap()
    }
}

impl<T, P> std::ops::IndexMut<P> for Grid<T>
where
    P: Into<Point<usize>>,
{
    fn index_mut(&mut self, point: P) -> &mut Self::Output {
        self.get_mut(point.into()).unwrap()
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, y: usize) -> &Self::Output {
        let start = y * self.width;
        &self.data[start..start + self.width]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        let start = y * self.width;
        &mut self.data[start..start + self.width]
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid([")?;
        for y in 0..self.height {
            write!(f, "  ")?;
            for x in 0..self.width {
                write!(f, "{:?}, ", self[Point::new(x, y)])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "])")?;
        Ok(())
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (&'a T, Point<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            None
        } else {
            let point = Point::new(self.x, self.y);
            let item = &self.grid[point];
            self.x += 1;
            if self.x >= self.grid.width {
                self.x = 0;
                self.y += 1;
            }
            Some((item, point))
        }
    }
}

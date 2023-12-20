use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Debug)]
// Line, Col
pub struct Pos(pub usize, pub usize);

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos(value.0, value.1)
    }
}

impl TryFrom<(isize, isize)> for Pos {
    type Error = &'static str;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        if value.0.is_negative() || value.1.is_negative() {
            Err("Position coordinates must be > 0")
        } else {
            Ok(Pos(value.0 as usize, value.1 as usize))
        }
    }
}

impl Pos {
    #[inline(always)]
    pub fn shift_unchecked(&mut self, dir: &Dir) {
        match dir {
            Dir::U => self.0 -= 1,
            Dir::D => self.0 += 1,
            Dir::L => self.1 -= 1,
            Dir::R => self.1 += 1,
        };
    }

    #[inline(always)]
    pub fn shift(&mut self, dir: &Dir) -> bool {
        match dir {
            Dir::U if self.0 > 0 => self.0 - 1,
            Dir::D if self.0 < usize::MAX => self.0 + 1,
            Dir::L if self.1 > 0 => self.1 - 1,
            Dir::R if self.1 < usize::MAX => self.1 + 1,
            _ => return false,
        };
        true
    }

    #[inline(always)]
    pub fn shift_diag(&mut self, dir: &DiagDir) -> bool {
        use std::usize::MAX;
        match dir {
            DiagDir::UL if self.0 > 0 && self.1 > 0 => {
                self.0 -= 1;
                self.1 -= 1
            }
            DiagDir::UC if self.0 > 0 => self.0 -= 1,
            DiagDir::UR if self.0 > 0 && self.1 < MAX - 1 => {
                self.0 -= 1;
                self.1 += 1
            }
            DiagDir::ML if self.1 > 0 => self.1 -= 1,
            DiagDir::MR if self.1 < MAX - 1 => self.1 += 1,
            DiagDir::BL if self.0 < MAX - 1 && self.1 > 0 => {
                self.0 += 1;
                self.1 -= 1
            }
            DiagDir::BC if self.0 < MAX - 1 => self.0 += 1,
            DiagDir::BR if self.0 < MAX - 1 && self.1 < MAX - 1 => {
                self.0 += 1;
                self.1 += 1
            }
            _ => return false,
        };
        true
    }

    #[inline(always)]
    pub fn shift_diag_unchecked(&mut self, dir: &DiagDir) {
        use DiagDir::*;
        match dir {
            UL => {
                self.0 -= 1;
                self.1 -= 1
            }
            UC => self.0 -= 1,
            UR => {
                self.0 -= 1;
                self.1 += 1
            }
            ML => self.1 -= 1,
            MR => self.1 += 1,
            BL => {
                self.0 += 1;
                self.1 -= 1
            }
            BC => self.0 += 1,
            BR => {
                self.0 += 1;
                self.1 += 1
            }
        }
    }

    #[inline(always)]
    pub fn add_pos(self, rhs: Dir) -> Option<Self> {
        match rhs {
            Dir::U if self.0 > 0 => Some(Pos(self.0 - 1, self.1)),
            Dir::D if self.0 < usize::MAX => Some(Pos(self.0 + 1, self.1)),
            Dir::L if self.1 > 0 => Some(Pos(self.0, self.1 - 1)),
            Dir::R if self.1 < usize::MAX => Some(Pos(self.0, self.1 + 1)),
            _ => None,
        }
    }
}

// Up, Down, Left, Right
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

const DIRS: [Dir; 4] = [Dir::U, Dir::D, Dir::L, Dir::R];
const DIAG_DIRS: [DiagDir; 8] = [
    DiagDir::UL,
    DiagDir::UC,
    DiagDir::UR,
    DiagDir::ML,
    DiagDir::MR,
    DiagDir::BL,
    DiagDir::BC,
    DiagDir::BR,
];

// Upper Left, Upper Center, Upper Right,
// Middle Left, Middle Right,
// Bottom Left, Bottom Center, Bottom Right
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DiagDir {
    UL,
    UC,
    UR,
    ML,
    MR,
    BL,
    BC,
    BR,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Grid<T> {
    grid: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.grid.iter()
    }

    #[inline(always)]
    pub fn shift(&self, pos: &mut Pos, dir: &Dir) -> bool {
        use Dir::*;
        match dir {
            U if pos.0 > 0 => pos.shift_unchecked(dir),
            D if pos.0 < self.height - 1 => pos.shift_unchecked(dir),
            L if pos.1 > 0 => pos.shift_unchecked(dir),
            R if pos.1 < self.width - 1 => pos.shift_unchecked(dir),
            _ => return false,
        };
        true
    }

    #[inline(always)]
    pub fn shift_diag(&self, pos: &mut Pos, dir: &DiagDir) -> bool {
        use DiagDir::*;
        match dir {
            UL if pos.0 > 0 && pos.1 > 0 => pos.shift_diag_unchecked(dir),
            UC if pos.0 > 0 => pos.shift_diag_unchecked(dir),
            UR if pos.0 > 0 && pos.1 < self.width - 1 => pos.shift_diag_unchecked(dir),
            ML if pos.1 > 0 => pos.shift_diag_unchecked(dir),
            MR if pos.1 < self.width - 1 => pos.shift_diag_unchecked(dir),
            BL if pos.0 < self.height - 1 && pos.1 > 0 => pos.shift_diag_unchecked(dir),
            BC if pos.0 < self.height - 1 => pos.shift_diag_unchecked(dir),
            BR if pos.0 < self.height - 1 && pos.1 < self.width - 1 => {
                pos.shift_diag_unchecked(dir)
            }
            _ => return false,
        };
        true
    }

    #[inline(always)]
    pub fn neighbor(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        use Dir::*;
        match dir {
            U if pos.0 > 0 => Some(Pos(pos.0 - 1, pos.1)),
            D if pos.0 < self.height - 1 => Some(Pos(pos.0 + 1, pos.1)),
            L if pos.1 > 0 => Some(Pos(pos.0, pos.1 - 1)),
            R if pos.1 < self.width - 1 => Some(Pos(pos.0, pos.1 + 1)),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn neighbor_diag(&self, pos: &Pos, dir: &DiagDir) -> Option<Pos> {
        match dir {
            DiagDir::UL if pos.0 > 0 && pos.1 > 0 => Some(Pos(pos.0 - 1, pos.1 - 1)),
            DiagDir::UC if pos.0 > 0 => Some(Pos(pos.0 - 1, pos.1)),
            DiagDir::UR if pos.0 > 0 && pos.1 < self.width - 1 => Some(Pos(pos.0 - 1, pos.1 + 1)),
            DiagDir::ML if pos.1 > 0 => Some(Pos(pos.0, pos.1 - 1)),
            DiagDir::MR if pos.1 < self.width - 1 => Some(Pos(pos.0, pos.1 + 1)),
            DiagDir::BL if pos.0 < self.height - 1 && pos.1 > 0 => Some(Pos(pos.0 + 1, pos.1 - 1)),
            DiagDir::BC if pos.0 < self.height - 1 => Some(Pos(pos.0 + 1, pos.1)),
            DiagDir::BR if pos.0 < self.height - 1 && pos.1 < self.width - 1 => {
                Some(Pos(pos.0 + 1, pos.1 + 1))
            }
            _ => None,
        }
    }

    #[inline(always)]
    pub fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        DIRS.iter()
            .filter_map(|dir| self.neighbor(&pos, dir))
            .collect()
    }

    #[inline(always)]
    pub fn diag_neighbors(&self, pos: Pos) -> Vec<Pos> {
        DIAG_DIRS
            .iter()
            .filter_map(|dir| self.neighbor_diag(&pos, dir))
            .collect()
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    #[inline(always)]
    pub fn new(value: T, height: usize, width: usize) -> Self {
        Self {
            grid: vec![value; height * width],
            height,
            width,
        }
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new_default(height: usize, width: usize) -> Self {
        Self {
            grid: vec![T::default(); height * width],
            height,
            width,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, line: usize, col: usize) -> &T {
        &self.grid[line * self.width + col]
    }
    pub fn get_mut(&mut self, line: usize, col: usize) -> &mut T {
        &mut self.grid[line * self.width + col]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.grid.index(index.0 * self.width + index.1)
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.grid.index_mut(index.0 * self.width + index.1)
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Pos) -> &Self::Output {
        self.grid.index(index.0 * self.width + index.1)
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        self.grid.index_mut(index.0 * self.width + index.1)
    }
}

impl<T> From<&str> for Grid<T>
where
    T: From<char>,
{
    // Assumes every newline is evenly spaced, so the rows are the same size.
    fn from(str_input: &str) -> Self {
        let map: Vec<T> = str_input.lines().fold(vec![], |mut acc, s| {
            acc.extend(s.chars().map(|s| s.into()));
            acc
        });
        let w = str_input.find('\n').unwrap();
        let h = map.len() / w;
        Grid {
            grid: map,
            height: h,
            width: w,
        }
    }
}

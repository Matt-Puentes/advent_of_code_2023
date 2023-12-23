use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
// Line, Col
pub struct Pos(pub usize, pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Hash)]
// Line, Col
pub struct IPos(pub i32, pub i32);

impl From<Pos> for IPos {
    fn from(value: Pos) -> Self {
        IPos(value.0 as i32, value.1 as i32)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 + self.1).cmp(&(other.0 + other.1))
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

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

    pub fn distance(&self, other: &Pos) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    pub fn range(&self, other: &Pos) -> impl Iterator<Item = Pos> {
        if self.0 <= other.0 {
            Box::new(self.0..other.0) as Box<dyn Iterator<Item = _>>
        } else {
            Box::new((other.0..self.0).rev()) as Box<dyn Iterator<Item = _>>
        }
        .zip(if self.1 <= other.1 {
            Box::new(self.1..other.1) as Box<dyn Iterator<Item = _>>
        } else {
            Box::new((other.1..self.1).rev()) as Box<dyn Iterator<Item = _>>
        })
        .map(|(p1, p2)| Pos(p1, p2))
    }
}

// Up, Down, Left, Right
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn rotr(self) -> Self {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }

    pub fn rotl(self) -> Self {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Dir::U => Dir::D,
            Dir::D => Dir::U,
            Dir::L => Dir::R,
            Dir::R => Dir::L,
        }
    }

    pub fn vertical(&self) -> bool {
        match self {
            Dir::U | Dir::D => true,
            Dir::R | Dir::L => false,
        }
    }

    pub fn val(self) -> (i8, i8) {
        match self {
            Dir::U => (-1, 0),
            Dir::D => (1, 0),
            Dir::L => (0, -1),
            Dir::R => (0, 1),
        }
    }
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

impl<T> Grid<T>
where
    T: Display,
{
    pub fn print_grid(&self, marked_spots: &[Pos], marker_value: char) {
        for l in 0..self.height {
            for c in 0..self.width {
                if marked_spots.contains(&Pos(l, c)) {
                    print!("{}", marker_value)
                } else {
                    print!("{}", self[(l, c)])
                }
            }
            println!()
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}

impl<T> Grid<T> {
    pub fn lines(&self) -> std::slice::ChunksExact<T> {
        self.grid.chunks_exact(self.width)
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.grid.iter()
    }

    #[inline]
    pub fn iter_pos(&self) -> impl Iterator<Item = (Pos, &T)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, t)| (self.idx_to_pos(i), t))
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::Iter<'_, T> {
        self.grid.iter()
    }

    #[inline]
    pub fn iter_pos_mut(&mut self) -> impl Iterator<Item = (Pos, &mut T)> + '_ {
        self.grid
            .iter_mut()
            .enumerate()
            .map(|(i, t)| (Pos(i / self.width, i % self.width), t))
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
    pub fn go(&self, pos: &Pos, magnitude: usize, dir: &Dir) -> Option<Pos> {
        use Dir::*;
        match dir {
            U if pos.0 > magnitude - 1 => Some(Pos(pos.0 - magnitude, pos.1)),
            D if pos.0 < self.height - magnitude => Some(Pos(pos.0 + magnitude, pos.1)),
            L if pos.1 > magnitude - 1 => Some(Pos(pos.0, pos.1 - magnitude)),
            R if pos.1 < self.width - magnitude => Some(Pos(pos.0, pos.1 + magnitude)),
            _ => None,
        }
    }

    pub fn range(&self, dir: &Dir, pos: &Pos, len: usize) -> Option<impl Iterator<Item = &T>> {
        self.go(pos, len, dir)
            .map(|other_pos| pos.range(&other_pos).map(|p| &self[p]))
    }

    #[inline(always)]
    pub fn ipos_neighbor(&self, pos: &IPos, dir: &Dir) -> Option<Pos> {
        use Dir::*;
        match dir {
            U if pos.0 > 0 => Some(Pos((pos.0 - 1) as usize, pos.1 as usize)),
            D if pos.0 < (self.height as i32) - 1 => {
                Some(Pos((pos.0 + 1) as usize, pos.1 as usize))
            }
            L if pos.1 > 0 => Some(Pos(pos.0 as usize, (pos.1 - 1) as usize)),
            R if pos.1 < (self.width as i32) - 1 => Some(Pos(pos.0 as usize, (pos.1 + 1) as usize)),
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
    /// Returns the valid neighbors in the order Up, Down, Left, Right
    pub fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        DIRS.iter()
            .filter_map(|dir| self.neighbor(&pos, dir))
            .collect()
    }

    #[inline(always)]
    /// Returns the valid neighbors in the order Up, Down, Left, Right
    pub fn all_neighbors(&self, pos: Pos) -> [Option<Pos>; 4] {
        [
            self.neighbor(&pos, &DIRS[0]),
            self.neighbor(&pos, &DIRS[1]),
            self.neighbor(&pos, &DIRS[2]),
            self.neighbor(&pos, &DIRS[3]),
        ]
    }

    #[inline(always)]
    pub fn neighbors_iter(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        DIRS.iter().filter_map(move |dir| self.neighbor(&pos, dir))
    }

    #[inline(always)]
    pub fn ipos_neighbors(&self, pos: IPos) -> Vec<Pos> {
        DIRS.iter()
            .filter_map(|dir| self.ipos_neighbor(&pos, dir))
            .collect()
    }

    #[inline(always)]
    pub fn ipos_neighbors_iter(&self, pos: IPos) -> impl Iterator<Item = Pos> + '_ {
        DIRS.iter()
            .filter_map(move |dir| self.ipos_neighbor(&pos, dir))
    }

    #[inline(always)]
    /// Returns the valid neighbors from top left to bottom right
    pub fn diag_neighbors(&self, pos: Pos) -> Vec<Pos> {
        DIAG_DIRS
            .iter()
            .filter_map(|dir| self.neighbor_diag(&pos, dir))
            .collect()
    }

    #[inline(always)]
    /// Returns the valid neighbors in the order Up, Down, Left, Right
    pub fn all_diag_neighbors(&self, pos: Pos) -> [Option<Pos>; 8] {
        [
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
            self.neighbor_diag(&pos, &DIAG_DIRS[0]),
        ]
    }

    #[inline(always)]
    pub fn diag_neighbors_iter(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        DIAG_DIRS
            .iter()
            .filter_map(move |dir| self.neighbor_diag(&pos, dir))
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

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn find(&self, obj: &T) -> Option<(Pos, &T)> {
        if let Some((idx, t)) = self.grid.iter().enumerate().find(|(_i, t)| *t == obj) {
            Some((self.idx_to_pos(idx), t))
        } else {
            None
        }
    }
}

impl<T> Grid<T> {
    #[inline(always)]
    pub fn idx_to_pos(&self, val: usize) -> Pos {
        Pos(val / self.width, val % self.width)
    }
    #[inline(always)]
    pub fn pos_to_idx(&self, val: Pos) -> usize {
        val.0 * self.width + val.1
    }
    pub fn get(&self, line: usize, col: usize) -> Option<&T> {
        self.grid.get(line * self.width + col)
    }
    pub fn get_mut(&mut self, line: usize, col: usize) -> Option<&mut T> {
        self.grid.get_mut(line * self.width + col)
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

impl<T> Grid<T> {
    pub fn map_from(str_input: &str, f: fn(char) -> T) -> Self {
        let map: Vec<T> = str_input.lines().fold(vec![], |mut acc, s| {
            acc.extend(s.chars().map(f));
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

impl<T> From<&str> for Grid<T>
where
    T: From<char>,
{
    /// Assumes every newline is evenly spaced, so the rows are the same size.
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

impl<'a, T: 'a> Grid<T> {
    pub fn astar<P, FN, IN>(
        &self,
        start: &'a P,
        mut successors: FN,
        mut heuristic: impl FnMut(&'a Self, &P) -> usize,
        mut success: impl FnMut(&'a Self, &P) -> bool,
    ) -> Option<(Vec<P>, usize)>
    where
        P: Eq,
        FN: FnMut(&'a Self, &'a P) -> IN + 'a,
        IN: IntoIterator<Item = (P, usize)> + 'a,
    {
        None
    }
}

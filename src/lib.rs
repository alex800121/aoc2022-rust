use std::ops::Range;

pub trait EucVec {
    fn overlap(&self, other: &Self) -> Option<Self> where Self: Sized;
    fn subtract(&self, other: &Self) -> Self;
    fn union(&self, other: &Self) -> Self;
}

impl<Idx: Copy, const N: usize> EucVec for &[Range<Idx>; N] {
    fn overlap(&self, other: &Self) -> Option<Self>  {
        for range in self.iter() {

        }
        todo!()
    }

    fn subtract(&self, other: &Self) -> Self {
        todo!()
    }

    fn union(&self, other: &Self) -> Self {
        todo!()
    }
}

pub trait Transpose {
    fn transpose(&mut self) -> Self;
}

impl<T> Transpose for Vec<Vec<T>> {
    fn transpose(&mut self) -> Self {
        let mut new_vec: Vec<Vec<T>> = Vec::new();
        for row in self {
            for i in 0..row.len() {
                let e = row.remove(0);
                match new_vec.get_mut(i) {
                    Some(v) => v.push(e),
                    None => {
                        new_vec.push(vec![e]);
                    }
                }
            }
        }
        new_vec
    }
}

pub fn zip_with<T, S, U, F>(a: Vec<T>, b: Vec<S>, f: F) -> Vec<U>
    where F: Fn(T, S) -> U {
    let a = a.into_iter();
    let mut b = b.into_iter();
    let mut c = Vec::new();
    for x in a {
        if let Some(y) = b.next() {
            c.push(f(x, y));
        } else {
            break;
        }
    }
    c
}

pub trait Enum where Self: Sized {
    fn from_enum(&self) -> isize;
    fn to_enum(n: isize) -> Self;
    fn succ(&self) -> Self {
        Self::to_enum(self.from_enum() + 1)
    }
    fn pred(&self) -> Self {
        Self::to_enum(self.from_enum() - 1)
    }
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Enum for Direction {
    fn from_enum(&self) -> isize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
    fn to_enum(n: isize) -> Self {
        match n % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }
}

pub enum Turn {
    Right,
    Left,
}

impl Enum for Turn {
    fn from_enum(&self) -> isize {
        match self {
            Turn::Right => 0,
            Turn::Left => 1,
        }
    }
    fn to_enum(n: isize) -> Self {
        match n % 2 {
            0 => Turn::Right,
            _ => Turn::Left,
        }
    }
}

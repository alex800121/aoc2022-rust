pub trait Transpose {
    fn transpose(&mut self) -> Self;
}

impl<T> Transpose for Vec<Vec<T>> {
    fn transpose(&mut self) -> Self {
        let mut new_vec: Vec<Vec<T>> = Vec::new();
        for row in self {
            for i in 0..row.len() {
                let e = row.remove(i);
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

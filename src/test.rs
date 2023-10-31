use std::fmt::Debug;
use std::ops::Index;
use std::ops::IndexMut;

pub struct MatrixXd<T> {
    elements: Vec<Vec<T>>,
}

impl<T> MatrixXd<T> 
where
    T: Debug + Clone + Copy,
{
    pub fn full(num_rows: usize, num_cols: usize, fill_value: T) -> Self {
        let mut rows = Vec::with_capacity(num_rows);

        for _ in 0..num_rows {
            let mut new_row = Vec::with_capacity(num_cols);

            for _ in 0..num_cols {
                new_row.push(fill_value);
            }

            rows.push(new_row);
        }

        MatrixXd {
            elements: rows
        }
    }
    
}

impl<T: Debug> Debug for MatrixXd<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new(); 

        for e in &self.elements {
            string = format!("{}{:?}\n", string, e);
        }
        
        write!(f, "{}",string.as_str())
    }
}

impl<T> Index<usize> for MatrixXd<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index] 
    }
}

impl<T> IndexMut<usize> for MatrixXd<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index] 
    }
}

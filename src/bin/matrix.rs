extern crate rand;

use std::sync::mpsc::sync_channel;
use std::sync::Arc;
use std::thread;
use std::fmt::{Display,Formatter};
use std::iter::Iterator;

use rand::Rng;

#[derive(Clone)]
struct Matrix {
    pub row_count: usize,
    pub column_count: usize,
    pub values: Vec<u32>
}

impl Matrix {
    fn new(row_count: usize, column_count: usize) -> Matrix {
        let mut values = Vec::with_capacity(row_count*column_count);
        let mut rng = rand::thread_rng();
        for _ in 0..row_count*column_count {
            values.push(rng.gen_range(0, 2))
        }
        Matrix { row_count, column_count, values }
    }
    fn row(&self, row: usize) -> impl Iterator<Item=&u32> {
        self.values[self.column_count*row..self.column_count*(row+1)].iter()
    }
    fn column<'a>(&'a self, column: usize) -> impl Iterator<Item=&u32> + 'a {
        (0..self.row_count)
            .map(move |row| &self.values[row*self.column_count + column])
    }
    fn cell(&mut self, row: usize, column: usize) -> &mut u32 {
        &mut self.values[self.column_count*row + column]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in 0..self.row_count {
            for value in self.row(row) {
                write!(f, "{}\t", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // Constants
    const ROW_COUNT: usize = 4;
    const COLUMN_COUNT: usize = 4;
    const THREAD_COUNT: usize = 4;
    
    // Initialize matrices
    let matrix1 = Arc::new(Matrix::new(ROW_COUNT, COLUMN_COUNT));
    let matrix2 = Arc::new(Matrix::new(ROW_COUNT, COLUMN_COUNT));

    // Create channels for sending and receieving
    let (tx, rx) = sync_channel(10);

    // Spawn threads
    let rows_per_thread = ROW_COUNT / THREAD_COUNT;
    for thread_id in 0..4 {
        let tx = tx.clone();
        let matrix1 = matrix1.clone();
        let matrix2 = matrix2.clone();
        thread::spawn(move || {
            let rows = thread_id*rows_per_thread..(thread_id+1)*rows_per_thread;
            for row in rows {
                for column in 0..COLUMN_COUNT {
                    let mut value = 0;
                    for e in matrix1.row(row).zip(matrix2.column(column)) {
                        value += e.0 * e.1;
                    }
                    tx.send((row,column,value)).unwrap();
                }
            }
        });
    }
    // Drop original tx, as we are using clones on each thread
    drop(tx);

    // Print matrices
    println!("{}", matrix1);
    println!("{}", matrix2);

    // Collect computed cell values
    let mut result = Matrix::new(ROW_COUNT, COLUMN_COUNT);
    for c in rx {
        *result.cell(c.0, c.1) = c.2;
    }

    // Print result
    println!("{}", result);
}

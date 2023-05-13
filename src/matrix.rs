use std::thread;

use rand::Rng;

const ROWS: usize = 100;
const COLS: usize = 100;

pub fn run() {
    // Create two empty matrices with random values
    let matrix1 = generate_matrix();
    let matrix2 = generate_matrix();

    // Common mult
    let time = std::time::Instant::now();
    let _result = multiply_matrices(&matrix1, &matrix2);
    let elapsed = time.elapsed();

    println!("Time elapsed for common mult: {}ms", elapsed.as_millis());

    // Threaded mult
    let time = std::time::Instant::now();
    let _result_threaded = unsafe { multiply_matrices_threaded(&matrix1, &matrix2, true) };
    let elapsed = time.elapsed();

    println!(
        "Time elapsed for multithreaded mult: {}ms",
        elapsed.as_millis()
    );
}

fn generate_matrix() -> Vec<Vec<u32>> {
    let mut matrix = Vec::with_capacity(ROWS);
    let mut rng = rand::thread_rng();
    for _i in 0..ROWS {
        let row = (0..COLS)
            .map(|_| rng.gen_range::<u32, _>(0..1000))
            .collect();
        matrix.push(row);
    }

    matrix
}

fn _print_matrix(matrix: &[Vec<u32>]) {
    for row in matrix {
        for &val in row {
            print!("{:8} ", val);
        }
        println!();
    }
}

fn multiply_matrices(matrix1: &[Vec<u32>], matrix2: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut product = Vec::with_capacity(ROWS);
    for _i in 0..ROWS {
        let row = vec![0; COLS];
        product.push(row);
    }

    for i in 0..ROWS {
        for j in 0..COLS {
            for k in 0..COLS {
                product[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    product
}

unsafe fn multiply_matrices_threaded(
    matrix1: &[Vec<u32>],
    matrix2: &[Vec<u32>],
    print: bool,
) -> Vec<Vec<u32>> {
    struct ResultPtr(*mut u32);
    impl ResultPtr {
        unsafe fn set(&self, value: u32) {
            *self.0 = value;
        }
    }
    unsafe impl Send for ResultPtr {}
    unsafe impl Sync for ResultPtr {}

    let mut result = Vec::with_capacity(ROWS);
    for _i in 0..ROWS {
        let row = vec![0; COLS];
        result.push(row);
    }

    let mut handles = Vec::with_capacity(ROWS * COLS);

    for i in 0..ROWS {
        for j in 0..COLS {
            let matrix1_row = matrix1[i].clone();
            let matrix2_col = matrix2.iter().map(|row| row[j]).collect::<Vec<u32>>();
            let result_ptr = ResultPtr(&mut result[i][j] as *mut u32);

            let handle = unsafe {
                thread::spawn(move || {
                    let mut sum = 0;
                    for k in 0..COLS {
                        sum += matrix1_row[k] * matrix2_col[k];
                    }
                    if print {
                        println!("[{}, {}] = {}", i, j, sum);
                    }
                    result_ptr.set(sum);
                })
            };
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    result
}

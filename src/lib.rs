// mod impls;

use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
mod petunia {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    #[pyo3(name="sort")]
    fn sort_py(mut a: Vec<i64>) -> Vec<i64> {
        // At this point, we've already copied the vector.
        // Now we sort it in a stupid way.

        for ii in (0..a.len()).rev() {
            println!("ii = {}", ii);
            for jj in ii..(a.len()-1) {
                if a[jj] > a[jj+1] {
                    let tmp = a[jj];
                    a[jj] = a[jj+1];
                    a[jj+1] = tmp;
                }
            }
        }

        a
    }
}


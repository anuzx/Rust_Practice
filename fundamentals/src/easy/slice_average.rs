/*
  Problem 11: Slice Average

  Write a function that takes a slice of f64 values and returns the arithmetic mean.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test slice_average_test
*/

pub fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut sum: f64 = 0.0;
    let length = values.len();
    for val in values {
        sum += *val;
    }
    let avg = sum / length as f64;
    Some(avg)
}

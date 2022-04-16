// MergaSort - 1945 / divide - and - conquer type algorithm.

pub fn merge_sort(a: &[i32]) -> Vec<i32> {
    if a.len() < 2 {
        a.to_vec()
    } else {
        let n = a.len();
        let c = merge_sort(&a[0..n / 2]);
        let d = merge_sort(&a[n / 2..n]);
        merge(&c, &d)
    }
}

fn merge(c: &[i32], d: &[i32]) -> Vec<i32> {
    let n = c.len() + d.len();
    let mut i = 0;
    let mut j = 0;
    let mut b = vec![0; n];
    for k in 0..n {
        println!("compare: {} c[{}] vs {} d[{}]", c[i], i, d[j], j);
        if c[i] < d[j] {
            b[k] = c[i];
            i = i + 1;
            println!("{}: c[{}] c.len={},k={}", b[k], i, c.len(), k);
            if i == c.len() {
                b = finito(&b[..k + 1], &d[j..]);
                break;
            }
        } else {
            b[k] = d[j];
            j = j + 1;
            println!("{}: d[{}] d.len={}", b[k], j, d.len());
            if j == d.len() {
                b = finito(&b[..k + 1], &c[i..]);
                break;
            }
        }
    }
    b
}

fn finito(head: &[i32], tail: &[i32]) -> Vec<i32> {
    let mut v3 = vec![];
    v3.extend_from_slice(head);
    v3.extend_from_slice(tail);
    v3
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_merge() {
        let a = [1, 2];
        let b = [3, 4];
        let expected = [1, 2, 3, 4];
        let result = merge(&a, &b);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge2() {
        let a = [1, 4];
        let b = [2, 3];
        let expected = [1, 2, 3, 4];
        let result = merge(&a, &b);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_finito() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8];
        let result = finito(&a, &b);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_sort() {
        let a = [5, 4, 1, 8, 7, 2, 6, 3];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8];
        let result = merge_sort(&a);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_sort_odd() {
        let a = [5, 4, 1, 8, 7, 2, 6, 3, 11, 10, 9];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let result = merge_sort(&a);
        assert_eq!(result, expected);
    }
}

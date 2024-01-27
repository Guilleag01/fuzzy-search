#[inline]
pub fn fzs(pattern: &str, list: &mut Vec<String>, n: usize) {
    // list.sort_by_key(|x| distance(pattern, x));
    *list = sort_n_first(list, n, |x| distance(pattern, x.as_str()));
}

fn distance(pattern: &str, text: &str) -> usize {
    let mut matrix = vec![vec![0_usize; pattern.len() + 1]; text.len() + 1];

    for i in 0..(pattern.len().max(text.len()) + 1) {
        if i <= text.len() {
            matrix[text.len() - i][pattern.len()] = i;
        }
        if i <= pattern.len() {
            matrix[text.len()][pattern.len() - i] = i;
        }
    }

    for i in (0..text.len()).rev() {
        for j in (0..pattern.len()).rev() {
            matrix[i][j] = if text.as_bytes()[i] == pattern.as_bytes()[j] {
                matrix[i + 1][j + 1]
            } else {
                matrix[i + 1][j + 1]
                    .min(matrix[i][j + 1])
                    .min(matrix[i + 1][j])
                    + 1
            }
        }
    }

    matrix[0][0]
}

fn sort_n_first<F>(list: &mut [String], n: usize, mut key: F) -> Vec<String>
where
    F: FnMut(String) -> usize,
{
    let mut result = vec![String::from(""); n];
    let mut values = vec![0; n];

    for e in list {
        let e_val = key(e.clone());
        for (i, other) in result.clone().iter().enumerate() {
            if other.is_empty() {
                result[i] = e.to_string();
                values[i] = e_val;
                break;
            }
            if e_val < values[i] {
                result.insert(i, e.to_string());
                result.truncate(n);
                values.insert(i, e_val);
                values.truncate(n);
                break;
            }
        }
    }

    // result.reverse();
    result
}

#[inline]
pub fn fzs(pattern: &str, list: &mut Vec<String>) {
    list.sort_by_key(|x| distance(pattern, x));
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

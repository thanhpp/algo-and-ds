// 1s ms; 2.1 MB
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut l_mtx: usize = 0;
    let mut r_mtx: usize = matrix.len();

    loop {
        let m_mtx = (l_mtx + r_mtx) / 2;
        // target maybe in this vector
        if matrix[m_mtx][0] <= target && matrix[m_mtx][matrix[m_mtx].len() - 1] >= target {
            let mut l: usize = 0;
            let mut r = matrix[m_mtx].len();

            loop {
                let m = (l + r) / 2;
                if matrix[m_mtx][m] == target {
                    return true;
                }

                if r - l <= 1 {
                    return false;
                }

                if matrix[m_mtx][m] < target {
                    l = m
                } else {
                    r = m
                }
            }
        }

        if r_mtx - l_mtx <= 1 {
            return false;
        }

        // find another vector
        if target < matrix[m_mtx][0] {
            r_mtx = m_mtx;
        } else {
            l_mtx = m_mtx
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_search_matrix() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3,
            ),
            true
        );
    }

    #[test]
    fn test_search_matrix_2() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                8,
            ),
            false
        )
    }
}

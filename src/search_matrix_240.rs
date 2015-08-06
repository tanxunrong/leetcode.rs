
/// O(m + n)
pub fn search_matrix(ma : &Vec<Vec<usize>>, target : usize) -> bool {
    let yl = ma.len();
    match yl {
        0 => { false },
        _ => {
            let xl = ma[0].len();
            match xl {
                0 => { false },
                _ => {
                    let mut i = 0;
                    let mut j = yl-1;
                    loop {
                        if i < xl {
                            if target == ma[j][i] {
                                return true;
                            } else if target > ma[j][i] {
                                i += 1;
                            } else if j != 0 {
                                j -= 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    return false;
                }
            }
        }
    }
}

#[test]
fn test_search_matrix() {
    let ma = vec![
      vec![1,   4,  7, 11, 15],
      vec![2,   5,  8, 12, 19],
      vec![3,   6,  9, 16, 22],
      vec![10, 13, 14, 17, 24],
      vec![18, 21, 23, 26, 30]
    ];
    assert!(search_matrix(&ma, 5));
    assert!(!search_matrix(&ma, 20));
    assert!(search_matrix(&ma, 16));
}

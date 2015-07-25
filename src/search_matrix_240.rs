
pub type Matrix = Vec<Vec<usize>>;

pub fn search_matrix(ma : &Matrix, target : usize) -> bool {
    let yl = ma.len();
    match yl {
        0 => { false },
        //TODO BinarySearch
        1 => { ma[0].contains(&target) },
        _ => {
            let xl = ma[0].len();
            match xl {
                0 => { false },
                //TODO BinarySearch
                1 => { ma.iter().any(|col| col[0] == target) },
                _ => {
                    let ix = xl/2 - 1;
                    let iy = yl/2 - 1 ;
                    assert!(ix >= 0 && iy >= 0);
                    assert!(yl > iy+1);
                    assert!(xl > ix+1);

                    let upleft = ma[iy][ix];
                    let downright = ma[iy+1][ix+1];

                    if  upleft == target || downright == target {
                        return true;
                    } else if upleft > target {
                        let ma_upleft = split_ma(ma, 0, 0, ix+1, iy+1);
                         search_matrix(&ma_upleft, target)
                    } else if downright < target {
                        let ma_downRight = split_ma(ma, ix+1, iy+1, xl, yl);
                        search_matrix(&ma_downRight, target)
                    } else {
                        let ma_downLeft = split_ma(ma, 0, iy+1, ix+1, yl);
                        if search_matrix(&ma_downLeft, target) {
                            return true;
                        }
                        let ma_upRight = split_ma(ma, ix+1, 0, iy+1,  xl);
                        if search_matrix(&ma_upRight, target) {
                            return true;
                        }
                        return false;
                    }
                }
            }
        }
    }
}

pub fn split_ma(ma : &Matrix, start_x : usize, start_y : usize, end_x : usize, end_y : usize, ) -> Matrix {
    if end_y > ma.len() || start_y > end_y {
        panic!("invalid index");
    }
    let mut ret : Matrix = vec![];
    let mut iy = start_y;
    loop {
        if iy >= end_y {
            break;
        }

        ret.push(split_vec(&ma[iy], start_x, end_x));
        iy += 1;
    }
    return ret;
}

pub fn split_vec(col : &Vec<usize>, start_x : usize, end_x : usize) -> Vec<usize> {
    if end_x > col.len() || start_x > end_x {
        panic!("invalid index");
    }
    let mut subcol = vec![];
    let mut ix = start_x;

    loop {
        if ix >= end_x {
            break;
        }
        subcol.push(col[ix]);
        ix += 1;
    }
    return subcol;
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

#[test]
fn test_split_vec() {
    let v = vec![1,   4,  7, 11, 15];
    assert_eq!(split_vec(&v, 1, 4), vec![4,  7, 11]);
    assert_eq!(split_vec(&v, 1, 2), vec![4]);
}

#[test]
fn test_split_ma() {
    let ma = vec![
      vec![1,   4,  7, 11, 15],
      vec![2,   5,  8, 12, 19],
      vec![3,   6,  9, 16, 22],
      vec![10, 13, 14, 17, 24],
      vec![18, 21, 23, 26, 30]
    ];
    let ret1 = vec![
    vec![   5,  8, 12],
    vec![  6,  9, 16],
    vec![ 13, 14, 17],
    ];
    let ret2 =  vec![
    vec![   5]
    ];
    assert_eq!(split_ma(&ma, 1, 1, 4, 4), ret1);
    assert_eq!(split_ma(&ma, 1, 1, 2, 2), ret2);
}

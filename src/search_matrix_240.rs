

// class Solution:
//     # @param {integer[][]} matrix
//     # @param {integer} target
//     # @return {boolean}
//     def searchMatrix(self, matrix, target):
//         colLength = len(matrix)
//         if colLength == 0:  return False
//         rowLength = len(matrix[0])
//         if rowLength == 0:  return False
//         leftElem = [matrix[i][0] for i in range(0,len(matrix))]
//         rightElem = [matrix[i][-1] for i in range(0,len(matrix))]
//         leftIndex = self.binSearch(leftElem, target)+1
//         rightIndex = self.binSearch(rightElem, target)
//         if leftIndex == 0 or rightIndex == len(matrix) - 1:
//             if target != matrix[-1][-1] and target != matrix[0][0]:
//                 return False
//             else:   return True
//         if leftIndex < rightIndex:  return False
//         if rightIndex >= 0 and matrix[rightIndex] == target:    return True
//         for i in xrange(rightIndex, leftIndex):
//             index = self.binSearch(matrix[i], target)
//             if index >= 0 and matrix[i][index] == target:   return True
//         return False

pub fn bin_search(ve : &Vec<usize>, target: usize, start : usize) -> usize {
    let l = ve.len();
    match l {
        0 => { return start; },
        1 | 2 | 3 => {
            let mut j = 0;
            for x in 0..l {
                if ve[x] > target {
                    break;
                } else {
                    j = x;
                }
            }
            return j+start;
        },
        _ => {
            let x = l/2;
            if ve[x] == target {
                return x+start;
            } else if ve[x] > target {
                if ve[x-1] <= target {
                    return x-1+start;
                } else {
                    let ve_left = split_vec(ve, 0, x);
                    return bin_search(&ve_left, target, start);
                }
            } else {
                    let ve_right = split_vec(ve, x+1, l);
                    return bin_search(&ve_right, target, start+x+1);
                }
            }
        }
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

// #[test]
// fn test_search_matrix() {
//     let ma = vec![
//       vec![1,   4,  7, 11, 15],
//       vec![2,   5,  8, 12, 19],
//       vec![3,   6,  9, 16, 22],
//       vec![10, 13, 14, 17, 24],
//       vec![18, 21, 23, 26, 30]
//     ];
//     assert!(search_matrix(&ma, 5));
//     assert!(!search_matrix(&ma, 20));
//     assert!(search_matrix(&ma, 16));
// }

#[test]
fn test_bin_search() {
    let ve = vec![1,  4,  7, 11, 15, 23, 34, 35, 41, 47, 55, 59, 80];
    assert_eq!(bin_search(&ve, 15, 0), 4);
    assert_eq!(bin_search(&ve, 39, 0), 7);
    assert_eq!(bin_search(&ve, 5, 0), 1);
}

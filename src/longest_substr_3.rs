use std::cmp;

pub fn longest_substr(slice : &[u8], hint : usize) -> usize {
    let len = slice.len();

    match len {
        0 | 1 => { len },
        2 => {
            if slice[0] == slice[1] {
                2
            } else {
                1
            }
        },
        _ => {
            let mut i = 0;
            let mut j = hint+1;
            loop {
                if j >= len {
                    return len;
                }
                if i + j < len {
                    println!("{} {}", i, j);
                    if slice[i] == slice[i+j] {
                            let (left, _ ) = slice.split_at(i+j);
                            let (_ , right) = slice.split_at(i+1);
                            return cmp::max(longest_substr(left, j), longest_substr(right, 0));
                    } else {
                        i += 1;
                    }
                } else {
                    println!("=====");
                    j += 1;
                    i = 0;
                }
            }
        }
    }
}
// defaultsforthatoptionunlessotherwisespecifie
#[test]
fn test_longest_substr() {
    let abc = "defaultsforthatoptionunlessotherwisespecified";
    let size = longest_substr(abc.as_bytes(), 0);
    assert_eq!(size, 8);
}

// search (a[len])
// len = 0 || 1 :
// return len
// k = 1
// k -> len-2:
//    i = start
//    i -> len-1:
//         if a[i] == a[i+k]:
//                return max(search(a[0,i+k-1]), search(a[i+1:len]))

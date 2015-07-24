pub fn two_sum(nums : &Vec<isize>, target : isize) -> (isize, isize) {
	// let nl = nums.len();
	// for x in 0..(nl-1) {
	// 	let nx = nums[x];
	// 	for y in (x+1)..nl {
	// 		let ny = nums[y];
	// 		if nx + ny == target {
	// 			return (nx, ny);
	// 		}
	// 	}
	// }

    let mut data : Vec<isize> = nums.clone();
    // O(n * lg(n))
    data.sort_by(|a, b| a.cmp(b));

    let mut x = 0;
    let mut y = data.len() - 1;
    // O(n)
    loop {
        if x == y {
        	panic!("can't resolve");
        }
        let sum = data[x] + data[y];
        if sum == target {
            return (data[x], data[y]);
        } else if sum > target {
            y -= 1;
        } else {
            x += 1;
        }
    }
}

#[test]
fn test_two_sum() {
	let nums = vec![2, 11, 15, 28, 7, 39, 1, -3];
	let target = 4;
    let (x, y) = two_sum(&nums, target);
    assert_eq!(x+y, target);
}

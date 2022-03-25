use std::time;

fn test_bin_search() {
    let nums: Vec<i32> = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
	let search_for: i32 = 5; 
    let t1 = time::Instant::now();
    let _r1: i32 = binary_search(&nums, search_for);
    println!("Binary search rev 2 took {:?}", t1.elapsed());
    std::mem::drop(t1);

    let t2 = time::Instant::now(); 
    let _r2 = binary_search_2(&nums, 0, (nums.len() - 1) as i32, search_for);
    println!("Binary search rev 2 took {:?}", t2.elapsed());
    std::mem::drop(t2);

    println!("The results were {} and {}", _r1, _r2); 
    println!();

    let t3 = time::Instant::now(); 
    println!("Minimum recordable time is {:?}", t3.elapsed());
}

fn binary_search(nums: &[i32], target: i32) -> i32 {
	let mut left: usize = 0;
	let mut right: usize = nums.len() - 1;

	while left <= right {
		let mid = (left + right) / 2;
		if nums[mid] == target {
			return mid as i32;
		} else if nums[mid] > target {
			right = mid - 1;
		} else {
			left = mid + 1;
		}
	}

	-1
}

fn binary_search_2(nums: &[i32], low: i32, high: i32, x: i32) -> i32 {
	if low > high {
		return -1;
	};

	let middle: i32 = low + ((high - low) / 2);

	if nums[middle as usize] == x {
		return middle;
	};

	if x < nums[middle as usize] {
		return binary_search_2(nums, low, middle - 1, x);
	};

	if x > nums[middle as usize] {
		return binary_search_2(nums, middle + 1, high, x);
	};

	-1
}

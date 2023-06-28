mod single_number;
use single_number::single_number;

use crate::single_number::{single_number_v2, single_number_v3};

fn main() {

	let nums =vec![2,2,1];
	dbg!(single_number(nums.clone()));

	/*
	Problem: Leetcode 136. Single Number
	Author: jechamt
	Date: 2022-02-14
	Runtime: 3ms 62.43% of Rust submissions
	Memory: 2.1 MB 84.13% of Rust submissions
	Successful Attempt No: 2 (1-indexed, 0 for unsuccesful versions)
	*/

	dbg!(single_number_v2(nums.clone()));
	dbg!(single_number_v3(nums));

}


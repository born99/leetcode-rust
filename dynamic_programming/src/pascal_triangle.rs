pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        return vec![vec![1]];
    }
    let mut result: Vec<Vec<i32>> = vec![vec![1]];

    for i in 1..=num_rows {
        let mut row: Vec<i32> = vec![];
        for j in 0..=i {
            let previous_row = &result[(i-1) as usize];

            let element = match j as usize {
                j @ 0 => 1,
                j if j == (i as usize) => 1,
                _ => previous_row[j as usize]+ previous_row[(j-1) as usize]
            };
            row.push(element);
        }
        result.push(row);
    }
    result
}
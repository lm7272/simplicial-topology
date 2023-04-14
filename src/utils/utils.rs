pub fn alternating_sum(v: &Vec<i32>) -> i32{
    let mut running_sum: i32 = 0;
    for i in 0..v.len(){
        running_sum += (-1 as i32).pow(i as u32) * v[i] as i32;
    }
    running_sum
}
use std::collections::HashSet;
use rand::{Rng, thread_rng};
use itertools::Itertools;

pub fn alternating_sum(v: &Vec<i32>) -> i32{
    let mut running_sum: i32 = 0;
    for i in 0..v.len(){
        running_sum += (-1 as i32).pow(i as u32) * v[i] as i32;
    }
    running_sum
}

pub fn filter_maximal_sets(mut s: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    s.sort_by_key(|x| x.len());
    let mut subsets: Vec<HashSet<usize>> = s.into_iter().map(HashSet::from_iter).collect();
    let mut result: Vec<HashSet<usize>> = Vec::new();
    while let Some(set) = subsets.pop() {
        let is_maximal = result
            .iter()
            .all(|maximal: &HashSet<usize>| !set.is_subset(maximal));
        if is_maximal {
            result.push(set);
        }
    }
    result.into_iter().map(|set| set.into_iter().collect()).collect()
}

pub fn filter_downward_closed_sets(mut s: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    s.sort_by_cached_key(|x| !x.len());
    let mut subsets: Vec<HashSet<usize>> = s.into_iter().map(HashSet::from_iter).collect();
    let mut result: Vec<HashSet<usize>> = vec![HashSet::new()];
    while let Some(set) = subsets.pop() {
        let boundary_present = set.iter().map(|elem| {
            let mut subset = set.clone();
            subset.remove(elem);
            subset
        }).collect::<Vec<HashSet<usize>>>().iter().all(|subset| {

            result.contains(&subset)});
        if boundary_present {
            result.push(set);
        }
    }
    filter_maximal_sets(result.into_iter().map(|set| set.into_iter().collect()).collect::<Vec<Vec<usize>>>())
}

pub fn randomly_select_items_from_vec<T: Clone>(v: &Vec<T>, p: f64) -> Vec<T> {
    let mut rng = thread_rng();
    v.iter()
        .filter(|_| rng.gen_bool(p))
        .cloned()
        .collect()
}

pub fn subvectors(v: &Vec<usize>, k: usize) -> Vec<Vec<usize>> {
    v.into_iter().combinations(k)
    .map(|subvec| subvec.into_iter().cloned().collect())
    .collect()
}
use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rand::{Rng, thread_rng};
use itertools::Itertools;
use rayon::prelude::*;

pub fn alternating_sum(v: &Vec<i32>) -> i32{
    let mut running_sum: i32 = 0;
    for i in 0..v.len(){
        running_sum += (-1 as i32).pow(i as u32) * v[i] as i32;
    }
    running_sum
}

/// Given a vector of vectors, keep only those vectors that are maximal with respect to subsets.
/// i.e, if s = [[1], [1,2], [1,2,3], [4,5]] the output would be [[1,2,3], [4,5]]
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

/// Given a vector of vectors return only those that are downward closed with respect to subsets.
/// i.e, if s = [[1], [1,2], [1,2,3], [4,5]] the output would be [[1,2,3]]
/// 
/// First group the vectors into a HashMap with key given by their length. The decision of whether to
/// include a given vector of length l is independent of all others of the same length l, so this step can be parallelised
/// for each l. Keep each vector iff all subvectors of length l-1 have already been kept.
/// 
/// The final step keeps only the maximal remaining vectors, for memory succinctness.
/// 
/// TODO: benchmark non parallel code to check this is better
pub fn filter_downward_closed_sets(s: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut subsets_by_length: HashMap<usize, Vec<HashSet<usize>>> = s
    .into_iter()
    .group_by(|subset| subset.len())
    .into_iter()
    .fold(HashMap::new(), |mut acc, (key, group)| {
        let vecs = group.map(|v| v.into_iter().collect_vec()).map(HashSet::from_iter).collect();
        acc.insert(key, vecs);
        acc
    });
    
    let  result = Arc::new(Mutex::new(vec![HashSet::new()]));
    for i in 1..(subsets_by_length.len()+1){
        let subsets = subsets_by_length.get_mut(&i).unwrap();
        let push_set = |set: &HashSet<usize>| {
            let boundary_present = set.par_iter().map(|elem| {
                let mut subset = set.clone();
                subset.remove(elem);
                subset
            }).collect::<Vec<HashSet<usize>>>().iter().all(|subset| {
                result.lock().unwrap().contains(subset)});
            if boundary_present {
                result.lock().unwrap().push(set.clone());
            }
        };
        subsets.par_iter().for_each(push_set);
    }
    let final_result = result.lock().unwrap();
    filter_maximal_sets(final_result.iter().map(|set| set.into_iter().cloned().collect()).collect::<Vec<Vec<usize>>>())
}

pub fn randomly_select_items_from_vec<T: Clone>(v: &Vec<T>, p: f64) -> Vec<T> {
    if p == 0.0 {
        return Vec::new()
    }
    else if p == 1.0 {
        return v.clone()
    }
    let mut rng = thread_rng();
    v.iter()
        .filter(|_| rng.gen_bool(p))
        .cloned()
        .collect()
}

/// Consume a vector and return a vector containing all subvectors of length k, preserving ordering
pub fn subvectors(v: &Vec<usize>, k: usize) -> Vec<Vec<usize>> {
    v.into_iter().combinations(k)
    .map(|subvec| subvec.into_iter().cloned().collect())
    .collect()
}
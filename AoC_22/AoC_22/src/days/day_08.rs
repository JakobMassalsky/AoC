use std::{slice::SliceIndex, iter::Enumerate};

use crate::{Solution, SolutionPair, etc::utils};
use itertools::Itertools;
use ndarray::{Array2, Axis, s};

///////////////////////////////////////////////////////////////////////////////

fn get_los(filler: &mut ndarray::ArrayBase<ndarray::OwnedRepr<usize>, ndarray::Dim<[usize; 2]>>, 
    iter: &mut dyn Iterator<Item = (usize, ndarray::ArrayBase<ndarray::ViewRepr<&i32>, ndarray::Dim<[usize; 1]>>)>,
    rev: bool) {
    let mut sizes_last_seen = [[0; 10]; 99];
    if rev {
        for (j, row) in iter {
            let mut size_last_seen = [0; 10];
            for (i, h) in row.iter().enumerate() {
                *filler.get_mut([j, i]).unwrap() 
                    *= (i - size_last_seen[(*h as usize)..].iter().max().unwrap())
                    * (j - sizes_last_seen[i][(*h as usize)..].iter().max().unwrap());
                size_last_seen[*h as usize] = i;
                sizes_last_seen[i][*h as usize] = j;
            }
        }
    } else {
        for (j, row) in iter {
            let mut size_last_seen = [0; 10];
            for (i, h) in row.iter().rev().enumerate() {
                *filler.get_mut([98 - j, 98 - i]).unwrap() 
                    *= (i - size_last_seen[(*h as usize)..].iter().max().unwrap())
                    * (j - sizes_last_seen[i][(*h as usize)..].iter().max().unwrap());
                size_last_seen[*h as usize] = i;
                sizes_last_seen[i][*h as usize] = j;
            }
        }
    }

}

fn test_view(acc: &mut ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>>, 
    iter: &mut dyn Iterator<Item = (usize, &i32)>, 
    isx: bool, 
    x: usize) {
    let mut max = -1;
    for (i, w) in iter {
        if *w > max {
            acc[match isx {
                true => [x, i],
                _=> [i, x]
            }] = 1;
            max = *w;
        }
        if max == 9 { break; }
    }
}

pub fn solve() -> SolutionPair {
    let lines = include_str!("../../input/input_08")
        .replace(&['\r', '\n'][..], "")
        .chars().map(|c| c.to_digit(10).unwrap() as i32)
        .collect_vec();
    let dims = (lines.len() as f64).sqrt() as usize;
    let a = Array2::<i32>::from_shape_vec((dims, dims), lines).unwrap();
    let mut b = Array2::<i32>::zeros(a.raw_dim());

    // Part 1
    for (x, row) in a.axis_iter(ndarray::Axis(0)).enumerate() {
        test_view(&mut b, &mut row.iter().enumerate(), true, x);
        test_view(&mut b, &mut row.iter().enumerate().rev(), true, x);
    }
    for (y, col) in a.axis_iter(ndarray::Axis(1)).enumerate() {
        test_view(&mut b, &mut col.iter().enumerate(), false, y);
        test_view(&mut b, &mut col.iter().enumerate().rev(), false, y);
    }
    let sol1: u64 = b.sum() as u64;
    
    // Part 2
    let mut b = Array2::<usize>::ones(a.raw_dim());
    get_los(&mut b, &mut a.axis_iter(Axis(0)).enumerate(), false);
    get_los(&mut b, &mut a.axis_iter(Axis(0)).rev().enumerate(), true);

    let sol2: u64 = *b.iter().max().unwrap() as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}

use std::{slice::SliceIndex, iter::Enumerate};

use crate::{Solution, SolutionPair, etc::utils};
use itertools::Itertools;
use ndarray::{Array2, Axis, s};

///////////////////////////////////////////////////////////////////////////////

fn get_los(filler: &mut ndarray::ArrayBase<ndarray::OwnedRepr<usize>, ndarray::Dim<[usize; 2]>>, 
    a: &ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>>,) {
    const SIZE: usize = 99;
    let mut sizes_last_seen = [[0; 10]; SIZE];
    let mut last_loss = [[false; 10]; SIZE];
    let mut size_last_seen;
    let mut last_los;
    for x in 0..SIZE {
        size_last_seen = [0; 10];
        last_los = [false; 10];
        for y in 0..SIZE {
            let h = a[[x, y]];
            *filler.get_mut([x, y]).unwrap() 
                *= (y - size_last_seen[(h as usize)..].iter().max().unwrap())
                * (x - sizes_last_seen[y][(h as usize)..].iter().max().unwrap());
            for (lh, l) in last_los[..=(h as usize)].iter_mut().enumerate() {
                if *l {
                    *filler.get_mut([x, size_last_seen[lh]]).unwrap()
                        *= y - size_last_seen[lh];
                    *l = false;
                }
            }
            for (lh, l) in last_loss[y][..=(h as usize)].iter_mut().enumerate() {
                if *l {
                    *filler.get_mut([sizes_last_seen[y][lh], y]).unwrap()
                        *= x - sizes_last_seen[y][lh];
                    *l = false;
                }
            }
            size_last_seen[h as usize] = y;
            sizes_last_seen[y][h as usize] = x;
            last_los[h as usize] = true;
            last_loss[y][h as usize] = true;
        }
        for lh in 0..=9 {
            if last_los[lh] {
                *filler.get_mut([x, size_last_seen[lh]]).unwrap()
                    *= SIZE - 1 - size_last_seen[lh];
            }
        }
    }
    for y in 0..SIZE {
        for lh in 0..=9 {
            if last_loss[y][lh] {
                *filler.get_mut([sizes_last_seen[y][lh], y]).unwrap()
                    *= SIZE - 1 - sizes_last_seen[y][lh];
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
    get_los(&mut b, &a);

    let sol2: u64 = *b.iter().max().unwrap() as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}

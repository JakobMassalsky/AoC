use crate::{Solution, SolutionPair, etc::utils};
use itertools::Itertools;
use  ndarray::Array2;

///////////////////////////////////////////////////////////////////////////////

fn get_los(trees: &ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>>, x: usize, y: usize, w: usize, h: usize) -> i32 {
    let th = trees[[x, y]];
    let s1 = trees.slice(ndarray::s![..x, y]).iter().rev().position(|v| *v>=th).unwrap_or(x-1)+1;
    let s2 = trees.slice(ndarray::s![x+1.., y]).iter().position(|v| *v>=th).unwrap_or(w - x-1)+1;
    let s3 = trees.slice(ndarray::s![x, ..y]).iter().rev().position(|v| *v>=th).unwrap_or(y-1)+1;
    let s4 = trees.slice(ndarray::s![x, y+1..]).iter().position(|v| *v>=th).unwrap_or(h - y-1)+1;
    (s1 * s2 * s3 * s4) as i32
}

fn test_view(acc: &mut ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>>, iter: &mut dyn Iterator<Item = (usize, &i32)>, isx: bool, x: usize) {
    let mut max = -1;
    let mut ind;
    for (i, w) in iter {
        if *w > max {
            if isx { ind = [x, i]; } 
            else { ind = [i, x]; } 
            acc[ind] = 1;
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
    let a = ndarray::Array2::<i32>::from_shape_vec((dims, dims), lines).unwrap();
    let mut b = ndarray::Array2::<i32>::zeros(a.raw_dim());

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
    let w = a.raw_dim()[0]-1;
    let h = a.raw_dim()[1]-1;
    for x in 1..w { 
        for y in 1..h {
            b[[x, y]] = get_los(&a, x, y, w, h);
        }
    }
    let sol2: u64 = *b.iter().max().unwrap() as u64;


    (Solution::U64(sol1), Solution::U64(sol2))
}

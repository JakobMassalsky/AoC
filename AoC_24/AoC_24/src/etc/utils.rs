use std::fmt::{Debug};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path> {
    let mut l: Vec<String> = Vec::new();
    if let Ok(file) = File::open(filename) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(s) = line {
                l.push(s);
            }
        }
    }
    return l;
}

pub fn extract_ints<T: FromStr>(line: &String, extra_chars: &[char]) -> Vec<T> where <T as FromStr>::Err: Debug {
    let mut num_buff = vec![];
    let mut nums = vec![];
    for c in line.chars() {
        if c.is_numeric() || extra_chars.contains(&c) {
            num_buff.push(c);
        } else if !num_buff.is_empty() {
            nums.push(num_buff.iter().collect::<String>().parse::<T>().unwrap());
            num_buff.clear();
        }
    }
    if !num_buff.is_empty() {
        nums.push(num_buff.iter().collect::<String>().parse::<T>().unwrap());
    }
    nums
}

// parse

pub fn parse_eager<T, I>(item: I) -> T
    where I: AsRef<str>,
        T: FromStr,
    {
        item.as_ref().parse().ok().unwrap()
    }

// ParseAll

pub trait ParseAll: Iterator {
    fn parse_all<T>(self) -> ParseAllIterator<Self, T>
    where
        Self: Sized,
        T: FromStr,
        Self::Item: AsRef<str>,
    {
        ParseAllIterator {
            iter: self,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<I: Iterator> ParseAll for I {}

#[derive(Clone)]
pub struct ParseAllIterator<I, T> {
    iter: I,
    _marker: std::marker::PhantomData<T>,
}

impl<I, T> Iterator for ParseAllIterator<I, T>
where
    I: Iterator,
    T: FromStr,
    I::Item: AsRef<str>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.as_ref().parse().ok().unwrap())
    }
}

// sumBy

pub trait SumBy: Iterator {
    fn sum_by<F, T>(self, f: F) -> T
    where
        Self: Sized,
        F: FnMut(Self::Item) -> T,
        T: std::iter::Sum,
    {
        self.map(f).sum()
    }
}

impl<I: Iterator> SumBy for I {}

// SkipRange

pub trait SkipRange: Iterator {
    fn skip_range(self, start: usize, count: usize) -> SkipRangeIterator<Self>
    where
        Self: Sized,
    {
        SkipRangeIterator {
            iter: self.enumerate(),
            start,
            count,
        }
    }
}

impl<I: Iterator> SkipRange for I {}

#[derive(Clone)]
pub struct SkipRangeIterator<I> {
    iter: std::iter::Enumerate<I>,
    start: usize,
    count: usize,
}

impl<I> Iterator for SkipRangeIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, value)) = self.iter.next() {
            if index < self.start || index >= self.start + self.count {
                return Some(value);
            }
        }
        None
    }
}

pub fn extract_ranges<T: Clone, R: std::ops::RangeBounds<usize>>(vec: &[T], ranges: &[R]) -> Vec<T> {
    ranges
        .iter()
        .flat_map(|range| {
            let start = match range.start_bound() {
                std::ops::Bound::Included(&s) => s,
                std::ops::Bound::Excluded(&s) => s + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                std::ops::Bound::Included(&e) => e + 1,
                std::ops::Bound::Excluded(&e) => e,
                std::ops::Bound::Unbounded => vec.len(),
            };
            vec[start..usize::min(end, vec.len())].to_vec()
        })
        .collect()
}

// diagonal Iter

// let indices = iter::repeat(0).take(l).chain(1..l)
//     .enumerate()
//     .map(|(i, x)| (x..=i).zip((x..=i.min(l-1)).rev()).take((1+i as i64).min((2*l -1) as i64 - i as i64) as usize).collect::<Vec<_>>())
//     .collect::<Vec<Vec<(usize, usize)>>>();

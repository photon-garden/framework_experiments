use crate::prelude::*;
use rayon::prelude::*;

pub fn zero_to_one(num_subdivisions: usize) -> impl ExactSizeIterator<Item = f32> {
    if num_subdivisions == 0 {
        panic!("Please call zero_to_one with a number of subdivisions greater than zero.");
    }

    let last_index = num_subdivisions - 1;

    (0..num_subdivisions).into_iter().map(move |index| {
        if num_subdivisions == 1 {
            0.0
        } else {
            index as f32 / last_index as f32
        }
    })
}

// The final item in this iterator is *not* one:
//
// zero_to_one_buckets(3) -> 0.0, 0.33, 0.66
pub fn zero_to_one_buckets(num_subdivisions: usize) -> impl Iterator<Item = f32> {
    if num_subdivisions == 0 {
        panic!("Please call zero_to_one with a number of subdivisions greater than zero.");
    }

    (0..num_subdivisions).into_iter().map(move |index| {
        if num_subdivisions == 1 {
            0.0
        } else {
            index as f32 / num_subdivisions as f32
        }
    })
}

pub fn zero_to_one_xy(
    num_subdivisions_x: usize,
    num_subdivisions_y: usize,
) -> impl Iterator<Item = Point2> {
    zero_to_one(num_subdivisions_y)
        .flat_map(move |y| zero_to_one(num_subdivisions_x).map(move |x| pt2(x, y)))
}

pub fn zero_to_one_xyz(
    num_subdivisions_x: usize,
    num_subdivisions_y: usize,
    num_subdivisions_z: usize,
) -> impl Iterator<Item = Point3> {
    zero_to_one(num_subdivisions_x).flat_map(move |x| {
        zero_to_one(num_subdivisions_y)
            .flat_map(move |y| zero_to_one(num_subdivisions_z).map(move |z| pt3(x, y, z)))
    })
}

pub fn par_zero_to_one(num_subdivisions: usize) -> impl ParallelIterator<Item = f32> {
    if num_subdivisions == 0 {
        panic!("Please call zero_to_one with a number of subdivisions greater than zero.");
    }

    let last_index = num_subdivisions - 1;

    (0..num_subdivisions).into_par_iter().map(move |index| {
        if num_subdivisions == 1 {
            0.0
        } else {
            index as f32 / last_index as f32
        }
    })
}

// pub fn zero_to_one(num_subdivisions: usize) -> ZeroToOne {
//     if num_subdivisions == 0 {
//         panic!("Please call zero_to_one with a number of subdivisions greater than zero.");
//     }

//     let last_index = num_subdivisions - 1;

//     ZeroToOne {
//         index: 0,
//         num_subdivisions,
//         last_index,
//     }
// }
// pub struct ZeroToOne {
//     index: usize,
//     num_subdivisions: usize,
//     last_index: usize,
// }

// impl Iterator for ZeroToOne {
//     type Item = f32;

//     fn next(&mut self) -> Option<Self::Item> {
//         let index = self.index;
//         let last_index = self.last_index;

//         if index > last_index {
//             return None;
//         }

//         self.index += 1;

//         // If num_subdivisions is 1, return 0.0
//         // then stop the iterator.
//         if self.num_subdivisions == 1 {
//             return Some(0.0);
//         }

//         let progress = index as f32 / last_index as f32;

//         Some(progress)
//     }
// }

use crate::prelude::*;
use std::ops::Add;
use std::ops::RangeInclusive;

pub trait VecExtension<T> {
    fn middle(&self) -> &T;
    fn looped_get(&self, index: usize) -> &T;
    fn looped_get_with_custom_loop_size(&self, index: usize, loop_size: usize) -> &T;
    fn normalized_get(&self, normalized_index: NormalizedF32) -> &T;
    fn normalized_get_with_index(&self, normalized_index: NormalizedF32) -> (usize, &T);
    fn weighted_get<GetWeight>(
        &self,
        normalized_index: NormalizedF32,
        get_weight: GetWeight,
    ) -> ElementLocation<T>
    where
        GetWeight: Fn(&T) -> f32;
    fn looped_normalized_get(&self, normalized_index: NormalizedF32) -> &T;
    fn denormalize_index(&self, normalized_index: NormalizedF32) -> usize;
    fn normalized_enumerate(&self) -> Vec<(f32, &T)>;
    fn last_index(&self) -> usize;
    fn next_to_last_index(&self) -> usize;
    fn neighbor_indexes_around(&self, center_index: usize, radius: usize) -> RangeInclusive<usize>;
    fn iter_adjacents(&self) -> IterAdjacents<'_, T>;
    fn find_min<GetValueToMinimize>(&self, get_value: GetValueToMinimize) -> Option<&T>
    where
        GetValueToMinimize: Fn(&T) -> f32;
    fn take_first(self) -> Option<T>;
}

impl<T> VecExtension<T> for Vec<T> {
    fn middle(&self) -> &T {
        let middle_index = self.len() / 2;
        self.get(middle_index).unwrap()
    }

    fn looped_get(&self, index: usize) -> &T {
        self.looped_get_with_custom_loop_size(index, self.len())
    }

    fn looped_get_with_custom_loop_size(&self, index: usize, loop_size: usize) -> &T {
        let looped_index = index % loop_size;
        self.get(looped_index).unwrap()
    }

    fn normalized_get(&self, normalized_index: NormalizedF32) -> &T {
        let index = self.denormalize_index(normalized_index);
        self.get(index).unwrap()
    }

    fn normalized_get_with_index(&self, normalized_index: NormalizedF32) -> (usize, &T) {
        let index = self.denormalize_index(normalized_index);
        let element = self.get(index).unwrap();
        (index, element)
    }

    fn weighted_get<GetWeight>(
        &self,
        normalized_index: NormalizedF32,
        get_weight: GetWeight,
    ) -> ElementLocation<T>
    where
        GetWeight: Fn(&T) -> f32,
    {
        if self.is_empty() {
            panic!("Can't call weighted_get on an empty Vec.");
        }

        let weight_total: f32 = self.iter().map(&get_weight).sum();
        let target_distance = normalized_index * weight_total;

        let mut distance_traveled = 0.0;
        let last_index = self.last_index();

        for (index, element) in self.iter().enumerate() {
            if index == last_index {
                return ElementLocation {
                    index,
                    element,
                    total_weight_at_previous_element: distance_traveled,
                };
            }

            let weight = get_weight(element);
            let distance_traveled_after_this_element = distance_traveled + weight;

            if distance_traveled_after_this_element > target_distance {
                return ElementLocation {
                    index,
                    element,
                    total_weight_at_previous_element: distance_traveled,
                };
            }

            distance_traveled += weight;
        }

        panic!("There's a bug in weighted_get and we got to the end of the function.");
    }

    fn looped_normalized_get(&self, normalized_index: NormalizedF32) -> &T {
        let index = self.denormalize_index(normalized_index);
        self.looped_get(index)
    }

    fn denormalize_index(&self, normalized_index: NormalizedF32) -> usize {
        if normalized_index >= 1.0 {
            return self.last_index();
        }

        let len = self.len() as f32;
        len.times(normalized_index).max(0.0).floor() as usize
    }

    fn normalized_enumerate(&self) -> Vec<(f32, &T)> {
        let last_index = self.len() - 1;

        self.iter()
            .enumerate()
            .map(|(index, element)| {
                let normalized_index = index as f32 / last_index as f32;
                (normalized_index, element)
            })
            .collect()
    }

    fn last_index(&self) -> usize {
        self.len() - 1
    }

    fn next_to_last_index(&self) -> usize {
        self.len() - 2
    }

    fn neighbor_indexes_around(&self, center_index: usize, radius: usize) -> RangeInclusive<usize> {
        let start_index = center_index.saturating_sub(radius);
        let end_index = center_index.add(radius).min(self.len());

        start_index..=end_index
    }

    fn iter_adjacents(&self) -> IterAdjacents<'_, T> {
        IterAdjacents {
            elements: self,
            index: 0,
        }
    }

    fn find_min<GetValueToMinimize>(&self, get_value: GetValueToMinimize) -> Option<&T>
    where
        GetValueToMinimize: Fn(&T) -> f32,
    {
        self.iter()
            .map(|element| (element, get_value(element)))
            .reduce(
                |(element_with_lowest_value_so_far, lowest_value_so_far),
                 (current_element, current_value)| {
                    if current_value < lowest_value_so_far {
                        (current_element, current_value)
                    } else {
                        (element_with_lowest_value_so_far, lowest_value_so_far)
                    }
                },
            )
            .map(|(element_with_lowest_value_so_far, _)| element_with_lowest_value_so_far)
    }

    fn take_first(self) -> Option<T> {
        self.into_iter().next()
    }
}

pub struct ElementLocation<'a, Element> {
    pub element: &'a Element,
    pub index: usize,
    pub total_weight_at_previous_element: f32,
}

pub struct IterAdjacents<'v, Element> {
    elements: &'v Vec<Element>,
    index: usize,
}

impl<'v, Element> Iterator for IterAdjacents<'v, Element> {
    type Item = [&'v Element; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.elements.next_to_last_index() {
            return None;
        }

        let current_element = self.elements.get(self.index).unwrap();
        let next_element = self.elements.get(self.index + 1).unwrap();

        self.index += 1;

        Some([current_element, next_element])
    }
}

mod tests {
    use super::*;

    #[test]

    fn test_weighted_get() {
        let vec = vec![25, 60, 15];

        let element_location = vec.weighted_get(0.0, |number| *number as f32);
        assert_eq!(*element_location.element, 25);
        assert_eq!(element_location.index, 0);
        assert_eq!(element_location.total_weight_at_previous_element, 0.0);

        let element_location = vec.weighted_get(0.25, |number| *number as f32);
        assert_eq!(*element_location.element, 60);
        assert_eq!(element_location.index, 1);
        assert_eq!(element_location.total_weight_at_previous_element, 25.0);

        let element_location = vec.weighted_get(0.40, |number| *number as f32);
        assert_eq!(*element_location.element, 60);
        assert_eq!(element_location.index, 1);
        assert_eq!(element_location.total_weight_at_previous_element, 25.0);

        let element_location = vec.weighted_get(0.85, |number| *number as f32);
        assert_eq!(*element_location.element, 15);
        assert_eq!(element_location.index, 2);
        assert_eq!(element_location.total_weight_at_previous_element, 85.0);

        let element_location = vec.weighted_get(1.0, |number| *number as f32);
        assert_eq!(*element_location.element, 15);
        assert_eq!(element_location.index, 2);
        assert_eq!(element_location.total_weight_at_previous_element, 85.0);
    }
}

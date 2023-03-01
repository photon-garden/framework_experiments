use crate::prelude::*;

pub trait VecWithOptionElementsExtension<Element> {
    fn filter_split(self) -> Vec<Vec<Element>>;
}

impl<Element> VecWithOptionElementsExtension<Element> for Vec<Option<Element>> {
    // let numbers = vec![
    //   Some(1), None,
    //   Some(1), Some(1), None,
    //   Some(1), Some(1), Some(1),
    // ];
    // let split = numbers.filter_split();
    // let expected = vec![
    //   vec![1],
    //   vec![1, 1],
    //   vec![1, 1, 1],
    // ];
    // assert_eq!(split, expected);
    fn filter_split(self) -> Vec<Vec<Element>> {
        let mut sub_vectors: Vec<Vec<Element>> = Vec::new();
        let mut current_sub_vector: Vec<Element> = Vec::new();
        for maybe_element in self.into_iter() {
            match maybe_element {
                Some(element) => {
                    current_sub_vector.push(element);
                }
                None => {
                    if !current_sub_vector.is_empty() {
                        sub_vectors.push(current_sub_vector);
                    }
                    current_sub_vector = Vec::new();
                }
            }
        }
        sub_vectors
    }
}

use crate::prelude::*;
use std::collections::HashMap;

pub type TokenMap = HashMap<usize, PointDirection>;

pub fn new(lindenmayer_system: &LindenmayerSystem) -> TokenMap {
    let mut new_token_map = HashMap::new();
    let root = &lindenmayer_system.root;

    let updater = TokenMapUpdater {
        branch: root,
        starting_point_direction: lindenmayer_system.starting_point_direction.clone(),
    };

    updater.process_tokens(&mut new_token_map);

    new_token_map
}

pub struct TokenMapUpdater<'a> {
    pub branch: &'a Branch,
    pub starting_point_direction: PointDirection,
}

impl<'a> TokenMapUpdater<'a> {
    pub fn process_tokens(&self, map: &mut TokenMap) {
        let mut current_point = self.starting_point_direction.point;
        let mut current_direction = self.starting_point_direction.direction;

        for token in &self.branch.tokens {
            match &token.name {
                TokenName::Forward(distance) => {
                    let movement = current_direction * *distance;
                    let next_point = current_point + movement;

                    current_point = next_point;
                }

                TokenName::Turn(angle) => {
                    let rotated = current_direction.rotate(angle.turns_to_radians());
                    current_direction = rotated;
                }

                TokenName::Branch(branch) => {
                    let new_starting_point_direction = PointDirection {
                        point: current_point,
                        direction: current_direction,
                    };

                    let updater = TokenMapUpdater {
                        starting_point_direction: new_starting_point_direction,
                        branch,
                    };

                    updater.process_tokens(map);
                }

                TokenName::Flower(_radius) => {
                    // let movement = current_direction * *radius;
                    // let center = current_point + movement;
                }

                TokenName::Leaf => {
                    // Do nothing.
                }
            }

            let location = PointDirection {
                point: current_point,
                direction: current_direction,
            };

            map.insert(token.id, location);
        }
    }
}

pub trait TokenMapExtension {
    fn find_token(&self, token: &Token) -> Option<&PointDirection>;
    fn find_token_point(&self, token: &Token) -> Option<Point2>;
}

impl TokenMapExtension for TokenMap {
    fn find_token(&self, token: &Token) -> Option<&PointDirection> {
        self.get(&token.id)
    }
    fn find_token_point(&self, token: &Token) -> Option<Point2> {
        self.find_token(token)
            .map(|point_direction| point_direction.point)
    }
}

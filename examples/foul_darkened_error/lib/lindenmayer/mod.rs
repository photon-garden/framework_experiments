use crate::prelude::*;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

mod token_map;
pub use token_map::*;

pub enum RuleResult {
    SkipRule,
    ReplaceWith(Vec<Token>),
}

use self::RuleResult::*;

type RuleFunction = fn(
    lindenmayer_system: &LindenmayerSystem,
    model: &Model,
    token_map: &TokenMap,
    current_branch: &Branch,
    token: &Token,
) -> RuleResult;

thread_local!(static ID_GENERATOR: IdGenerator = IdGenerator::new());

#[derive(Clone)]
pub struct LindenmayerSystem {
    pub root: Branch,
    pub starting_point_direction: PointDirection,
    pub state: LindenmayerSystemState,
    pub id: usize,
    rules: Vec<RuleFunction>,
    token_map: RefCell<Option<TokenMap>>,
}

impl LindenmayerSystem {
    pub fn new(
        rand: Rand,
        starting_point: Point2,
        starting_direction: Vec2,
        sun: Point2,
    ) -> LindenmayerSystem {
        let root = Branch {
            id: next_id(),
            tokens: vec![],
            rand,
            depth: 0,
        };

        let starting_point_direction = PointDirection {
            point: starting_point,
            direction: starting_direction,
        };

        let state = LindenmayerSystemState { sun };

        LindenmayerSystem {
            id: next_id(),
            rules: vec![],
            root,
            starting_point_direction,
            state,
            token_map: RefCell::new(None),
        }
    }

    pub fn set_initial_state(&mut self, tokens: Vec<Token>) {
        self.root.tokens = tokens;
    }

    pub fn add_rule(&mut self, rule_function: RuleFunction) {
        self.rules.push(rule_function);
    }

    pub fn apply_rules(&self, num_applications: usize, model: &Model) -> LindenmayerSystem {
        if num_applications == 0 {
            panic!("num_applications must be greater than 0.");
        }

        let mut current_system = self.apply_rules_once(model);
        for _ in 0..(num_applications - 1) {
            current_system = current_system.apply_rules_once(model);
        }

        current_system
    }

    pub fn apply_rules_once(&self, model: &Model) -> LindenmayerSystem {
        let new_tokens = self.replace_tokens_in_branch(model, &self.root);

        let new_root = Branch {
            id: self.root.id,
            tokens: new_tokens,
            depth: self.root.depth,
            rand: self.root.rand.clone(),
        };

        LindenmayerSystem {
            id: self.id,
            rules: self.rules.clone(),
            root: new_root,
            starting_point_direction: self.starting_point_direction.clone(),
            state: self.state.clone(),
            token_map: RefCell::new(None),
        }
    }

    fn replace_tokens_in_branch(&self, model: &Model, branch: &Branch) -> Vec<Token> {
        branch
            .tokens
            .iter()
            .flat_map(|token| self.replace_token(model, branch, token))
            .collect()
    }

    fn replace_token(&self, model: &Model, current_branch: &Branch, token: &Token) -> Vec<Token> {
        match &token.name {
            TokenName::Branch(child_branch) => {
                let replacement_tokens = self.replace_tokens_in_branch(model, &*child_branch);
                vec![self.branch(
                    child_branch.depth,
                    child_branch.rand.clone(),
                    replacement_tokens,
                )]
            }
            _ => self.replace_non_branch_token(model, current_branch, token),
        }
    }

    fn replace_non_branch_token(
        &self,
        model: &Model,
        branch: &Branch,
        token: &Token,
    ) -> Vec<Token> {
        let token_map = self.token_map();

        for rule in &self.rules {
            let result = rule(self, model, &token_map, branch, token);
            if let ReplaceWith(tokens) = result {
                return tokens;
            }
        }

        vec![token.clone()]
    }

    // pub fn token_map(&self) -> Ref<TokenMap> {
    pub fn token_map(&self) -> TokenMap {
        // Make LindenmayerSystem.token_map compute the token map if it doesn't exist, otherwise generate it and cache it.
        // Then use that to make our crowded rule iterate through all other lindenmayer systems in the model and turn the
        // current flower into a leaf if it's too close to a point on another branch.
        // Use the any? method equivalent to be more efficient.
        token_map::new(self)
        // self.token_map
        //     .borrow_mut()
        //     .get_or_insert_with(|| token_map::new(self));

        // Ref::map(self.token_map.borrow(), |token_map| {
        //     token_map.as_ref().unwrap()
        // })
    }

    // pub fn iter_branches(&self) -> Box<dyn Iterator<Item = &Branch> + '_> {
    pub fn iter_branches(&self) -> Vec<&Branch> {
        self.root.branches_including_self()
    }

    // Methods for making new tokens.

    pub fn forward(&self, distance: NormalizedF32) -> Token {
        let name = TokenName::Forward(distance);
        let id = next_id();
        Token { id, name }
    }

    pub fn turn(&self, angle: NormalizedF32) -> Token {
        let name = TokenName::Turn(angle);
        let id = next_id();
        Token { id, name }
    }

    pub fn flower(&self, radius: NormalizedF32) -> Token {
        let name = TokenName::Flower(radius);
        let id = next_id();
        Token { id, name }
    }

    fn branch(&self, depth: usize, rand: Rand, tokens: Vec<Token>) -> Token {
        let id = next_id();

        let new_branch = Branch {
            id,
            depth,
            rand,
            tokens,
        };

        let name = TokenName::Branch(Box::new(new_branch));

        Token { id, name }
    }

    pub fn child_branch(&self, parent: &Branch, tokens: Vec<Token>) -> Token {
        let rand = parent.rand.new_with_random_seed();
        let depth = parent.depth + 1;

        self.branch(depth, rand, tokens)
    }

    pub fn leaf(&self) -> Token {
        let name = TokenName::Leaf;
        let id = next_id();
        Token { id, name }
    }
}

fn next_id() -> usize {
    let mut next_id: usize = 0;
    ID_GENERATOR.with(|id_generator| next_id = id_generator.next());
    next_id
}

impl PartialEq for LindenmayerSystem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for LindenmayerSystem {}

impl PartialEq for Branch {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Branch {}

#[derive(Clone, Debug)]
pub struct Branch {
    pub id: usize,
    pub rand: Rand,
    pub depth: usize,
    pub tokens: Vec<Token>,
}

impl Branch {
    // pub fn branches_including_self(&self) -> Box<dyn Iterator<Item = &Branch> + '_> {
    pub fn branches_including_self(&self) -> Vec<&Branch> {
        let child_branches_iterator = self
            .tokens
            .iter()
            .filter_map(|token| {
                let name = &token.name;
                match name {
                    TokenName::Branch(boxed_branch) => {
                        let branch = &*boxed_branch;
                        Some(branch)
                    }
                    _ => None,
                }
            })
            .flat_map(|branch| branch.branches_including_self());

        let mut children: Vec<_> = child_branches_iterator.collect();
        children.insert(0, self);
        children

        // let chained = self_iterator.chain(child_branches_iterator);

        // Box::new(chained)
    }
}

#[derive(Clone, Debug)]
pub enum TokenName {
    Forward(NormalizedF32),
    Turn(NormalizedF32),
    Flower(NormalizedF32),
    Branch(Box<Branch>),
    Leaf,
}

#[derive(Clone, Debug)]
pub struct Token {
    id: usize,
    pub name: TokenName,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct LindenmayerSystemState {
    pub sun: Point2,
}

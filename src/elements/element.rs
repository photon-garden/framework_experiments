use crate::prelude::*;
use std::cell::Cell;

pub struct Element {
    num_atoms_to_draw_at_once: usize,
    atoms_to_draw: Cell<Vec<Atom>>,
    all_atoms: Box<dyn Iterator<Item = Atom>>,
    has_drawn: Cell<bool>,
}

impl Element {
    pub fn once<Draw>(draw_closure: Draw) -> Element
    where
        Draw: Fn(&DrawParams) + 'static,
    {
        let atom = Atom::new(draw_closure);
        std::iter::once(atom).into_element(1)
    }

    pub fn new<AtomsIterator>(
        num_atoms_to_draw_at_once: usize,
        mut all_atoms: AtomsIterator,
    ) -> Element
    where
        AtomsIterator: Iterator<Item = Atom> + 'static,
    {
        let atoms_to_draw = all_atoms.take_to_vec(num_atoms_to_draw_at_once);

        Element {
            num_atoms_to_draw_at_once,
            atoms_to_draw: Cell::new(atoms_to_draw),
            all_atoms: all_atoms.into_box(),
            has_drawn: Cell::new(false),
        }
    }

    pub fn draw(&self, params: &DrawParams) {
        self.has_drawn.set(true);
        let atoms_to_draw = self.atoms_to_draw.take();
        for atom in atoms_to_draw.into_iter() {
            atom.draw(params);
        }
    }

    pub fn update(&mut self) -> DoneDrawing {
        let has_drawn = self.has_drawn.get();
        if !has_drawn {
            return DoneDrawing::No;
        }

        let new_atoms_to_draw = self.all_atoms.take_to_vec(self.num_atoms_to_draw_at_once);

        if new_atoms_to_draw.is_empty() {
            DoneDrawing::Yes
        } else {
            self.atoms_to_draw = Cell::new(new_atoms_to_draw);
            DoneDrawing::No
        }
    }
}

pub trait AtomIteratorExtension {
    fn into_element(self, num_atoms_to_draw_at_once: usize) -> Element;
}

impl<Itera> AtomIteratorExtension for Itera
where
    Itera: Iterator<Item = Atom> + 'static,
{
    fn into_element(self, num_atoms_to_draw_at_once: usize) -> Element {
        Element::new(num_atoms_to_draw_at_once, self)
    }
}

type DrawFunction = Box<dyn FnOnce(&DrawParams)>;

pub struct Atom {
    draw_closure: DrawFunction,
}

impl Atom {
    pub fn new<Draw>(draw_closure: Draw) -> Atom
    where
        Draw: FnOnce(&DrawParams) + 'static,
    {
        Atom {
            draw_closure: Box::new(draw_closure),
        }
    }

    pub fn draw(self, params: &DrawParams) {
        (self.draw_closure)(params);
    }
}

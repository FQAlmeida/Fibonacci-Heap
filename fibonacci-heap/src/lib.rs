use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use heap_operations::{HeapOperations, PriorityItem};

#[derive(Debug, Clone)]
struct Item<'self_lf, T> {
    key: T,
    parent: Option<&'self_lf Item<'self_lf, T>>,
    left: Option<&'self_lf Item<'self_lf, T>>,
    right: Option<&'self_lf Item<'self_lf, T>>,
    children: Vec<&'self_lf Item<'self_lf, T>>,
}

impl<'self_lf, T> Item<'self_lf, T> {
    pub fn new(value: T) -> Self {
        let mut this = Self {
            key: value,
            parent: None,
            left: None,
            right: None,
            children: Vec::new(),
        };
        return this;
    }
}

impl<'self_lf, T> PriorityItem<T> for Item<'self_lf, T>
where
    T: Eq + Ord + PartialEq + PartialOrd,
{
    fn get_key(self) -> T {
        return self.key;
    }
}

pub struct FibonacciHeap<'item_lf, T>
where
    T: Eq + Ord + PartialEq + PartialOrd,
{
    min: Option<&'item_lf Item<'item_lf, T>>,
}

impl<'item_lf, T> FibonacciHeap<'item_lf, T>
where
    T: Eq + Ord + PartialEq + PartialOrd,
{
    pub fn new() -> Self {
        return Self { min: None };
    }
}

impl<'item_lf, T> FibonacciHeap<'item_lf, T>
where
    T: Eq + Ord + PartialEq + PartialOrd,
{
    fn insert_not_empty(&mut self, item: &Item<T>) {}
}

impl<'min_lf, 'item_lf, T> HeapOperations<'min_lf, T> for FibonacciHeap<'item_lf, T>
where
    T: Eq + Ord + PartialEq + PartialOrd,
{
    fn insert(&mut self, value: T) {
        let item: Item<T> = Item::new(value);
        // match &self.min {
        //     Some(_) => self.insert_not_empty(&item),
        //     None => self.min = Some(&item),
        // }
    }

    fn find_min(&self) -> &'min_lf T {
        todo!()
    }

    fn delete_min(&mut self) -> T {
        todo!()
    }

    fn decrease_key(&mut self, item: &T) {
        todo!()
    }

    fn meld(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {}

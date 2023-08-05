pub trait PriorityItem<T>
where
    T: PartialOrd + PartialEq + Ord + Eq,
{
    fn get_key(self) -> T;
}

pub trait HeapOperations<'min_lf, T>
where
    T: PartialOrd + PartialEq + Ord + Eq,
{
    fn find_min(&self) -> &'min_lf T;
    fn delete_min(&mut self) -> T;
    fn insert(&mut self, item: T);
    fn decrease_key(&mut self, item: &T);
    fn meld(&mut self);
}

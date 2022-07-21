use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

pub type InnerPopulation<T> = Vec<T>;
#[derive(Clone, Debug)]
pub struct Population<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    list: InnerPopulation<T>,
    capacity: usize,
}

impl<V> Extend<V> for Population<V>
where
    V: PartialOrd + Clone,
{
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        for element in iter {
            self.push(element)
        }
    }
}

impl<T> Population<T>
where
    T: PartialOrd + Clone,
{
    pub fn with_capacity(capacity: usize) -> Self {
        let list = Vec::with_capacity(capacity);
        Population { list, capacity }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.list.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.list.get_mut(index)
    }

    pub fn sort(&mut self) -> () {
        self.list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }

    pub fn first(&self) -> Option<&T> {
        self.list.first()
    }

    pub fn middle(&self) -> Option<&T> {
        let middle_index = (((self.list.len() - 1) as f64) / 2.).floor() as usize;
        self.list.get(middle_index)
    }

    pub fn last(&self) -> Option<&T> {
        self.list.last()
    }

    pub fn push(&mut self, value: T) -> () {
        self.list.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        self.list.iter()
    }

    pub fn into_iter(self) -> IntoIter<T> {
        self.list.into_iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<T> {
        self.list.iter_mut()
    }
}

impl<T> IntoIterator for Population<T>
where
    T: PartialOrd + Clone,
{
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl<E> FromIterator<E> for Population<E>
where
    E: Clone + PartialOrd,
{
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut population = Population::with_capacity(100);
        for elem in iter {
            population.push(elem)
        }
        population
    }
}

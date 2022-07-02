use std::collections::{
    vec_deque::{IntoIter, Iter, IterMut},
    VecDeque,
};

use crate::{measure::benchmarks::Benchmark, utils::common_traits::Compare};

use super::characteristics::{FitnessScore, Organism};

type InnerPopulation<T> = VecDeque<T>;
#[derive(Debug, Clone)]
pub struct Population<T>
where
    T: Compare,
{
    list: InnerPopulation<T>,
    capacity: usize,
}

impl<T> Population<T>
where
    T: Compare,
{
    pub fn new_with_capacity(population_size: usize) -> Self {
        let list = VecDeque::with_capacity(population_size);
        Population {
            list,
            capacity: population_size,
        }
    }

    pub fn new() -> Self {
        let list = VecDeque::with_capacity(10);
        Population { list, capacity: 10 }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.list.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.list.get_mut(index)
    }

    pub fn sort(&mut self) -> () {
        self.list.make_contiguous().sort();
    }

    pub fn first(&self) -> Option<&T> {
        self.list.get(0)
    }

    pub fn last(&self) -> Option<&T> {
        self.list.get(self.list.len() - 1)
    }

    pub fn middle(&self) -> Option<&T> {
        self.list
            .get(math::round::floor(self.list.len() as f64 / 2f64, 1) as usize)
    }

    pub fn push_front(&mut self, value: T) -> () {
        self.list.push_front(value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn push_back(&mut self, value: T) -> () {
        self.list.push_back(value)
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
    T: Compare,
{
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<E> FromIterator<E> for Population<E>
where
    E: Compare,
{
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut population = Population::new();
        for elem in iter {
            population.push_back(elem)
        }
        population
    }
}

impl<'a, T> Benchmark for Population<T>
where
    T: Organism<'a>,
{
    type InputType = FitnessScore;

    fn get_worst(&self) -> Option<Self::InputType> {
        self.first().unwrap().get_fitness()
    }

    fn get_median(&self) -> Option<Self::InputType> {
        self.middle().unwrap().get_fitness()
    }

    fn get_best(&self) -> Option<Self::InputType> {
        self.last().unwrap().get_fitness()
    }
}

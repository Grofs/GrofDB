use std::{collections::HashSet, hash::Hash};

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub trait simple_IteratorExt: Iterator {
    fn simple_unique(self) -> simple_Unique_Iterator<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        simple_Unique_Iterator{
            originalIterator:self,
            seenitems: HashSet::new()
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct simple_Unique_Iterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    originalIterator: I,
    seenitems: HashSet<I::Item>,
}

impl <I> Iterator for simple_Unique_Iterator<I>
where 
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item>{
        self.originalIterator.find(|item| self.seenitems.insert(item.clone()))
    }
}

impl <I:Iterator> simple_IteratorExt for I {}

pub fn select_unique() -> Vec<u64>{
    let nos = vec![];
    let res:Vec<u64> = nos.into_iter().simple_unique().collect();
    return res;
}

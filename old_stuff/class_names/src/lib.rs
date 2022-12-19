#![no_std]
use core::cell::RefCell;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub trait AddClassList<'a> {
    fn insert_into_list(&'a self, list: &'a ClassList<'a>);
}

impl<'a> AddClassList<'a> for &str {
    fn insert_into_list(&'a self, list: &'a ClassList<'a>) {
        list.classes.borrow_mut().push(self);
    }
}

impl<'a> AddClassList<'a> for Option<&str> {
    fn insert_into_list(&'a self, list: &'a ClassList<'a>) {
        match self {
            Some(t) => list.classes.borrow_mut().push(t),
            None => (),
        };
    }
}

impl<'a> AddClassList<'a> for Option<String> {
    fn insert_into_list(&'a self, list: &'a ClassList<'a>) {
        match self {
            Some(t) => list.classes.borrow_mut().push(&t),
            None => (),
        };
    }
}

impl<'a> AddClassList<'a> for String {
    fn insert_into_list(&'a self, list: &'a ClassList<'a>) {
        list.classes.borrow_mut().push(&self);
    }
}

pub struct ClassList<'a> {
    classes: RefCell<Vec<&'a str>>,
}

impl<'a> ClassList<'a> {
    pub fn new() -> ClassList<'a> {
        ClassList {
            classes: RefCell::new(Vec::new()),
        }
    }
    pub fn add<T>(&'a self, item: &'a T)
    where
        T: AddClassList<'a>,
    {
        (&item).insert_into_list(self);
    }
    pub fn to_string(&self) -> String {
        self.classes.borrow_mut().join(" ")
    }
}

#[macro_export]
macro_rules! class_names {
    // `()` indicates that the macro takes no argument.
    ($($element:expr),*) => {
        {
            let class_list = ClassList::new();
            $(
                let e = $element;
                class_list.add(&e);
            )*
            class_list.to_string()
        }
    };
}

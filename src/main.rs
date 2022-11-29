use core::cell::RefCell;
use core::fmt::Debug;

trait MyTrait: Debug {
    fn do_something(&mut self);
}

#[derive(Debug)]
struct MyStruct {
    counter: usize,
}

impl MyStruct {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl MyTrait for MyStruct {
    fn do_something(&mut self) {
        self.counter += 1;
        println!("{self:?}");
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum MyEnum<'a> {
    VarOne(RefCell<&'a dyn MyTrait>),
    VarTwo(RefCell<&'a dyn MyTrait>),
}

fn main() {
    let data = MyStruct::new();
    let foo = MyEnum::VarOne(RefCell::new(&data));
    let mut foo_vec: heapless::LinearMap<u16, MyEnum, 8> = heapless::LinearMap::new();
    let key = 42;
    foo_vec.insert(key, foo).unwrap();

    match foo_vec.get_mut(&key).unwrap() {
        MyEnum::VarOne(data) => {
            let mut data_mut = data.borrow_mut();
            // cannot borrow data in dereference of `RefMut<'_, &dyn MyTrait>` as mutable
            // help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `RefMut<'_, &dyn MyTrait>`
            data_mut.do_something();
        }
        MyEnum::VarTwo(data) => {
            println!("{:?}", data);
        }
    }
}

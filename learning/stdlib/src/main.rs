trait BasicInfo {
    // contains only function declarations
    // these function declarations can have
    // different definitions for different types

    // trait functions (can be called using scoping trait::method)
    // fn basics() -> Self;

    // these are trait methods (have self in parameters)
    fn name(&self) -> &'static str;
    fn age(&self) -> u8;
}

// `Person` and `Dog` attributes are same except one
// TODO: optional trait methods possible in rust?
struct Person {
    name: &'static str,
    age: u8,
    // school: String,
}

struct Dog {
    name: &'static str,
    age: u8,
    // breed: String,
}

impl BasicInfo for Person {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn age(&self) -> u8 {
        return self.age;
    }
}

impl BasicInfo for Dog {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn age(&self) -> u8 {
        return self.age;
    }
}

trait MyTrait<'a, T> {
    fn func1(arg: T);
    fn func2(arg: &'a u32);
    fn func3(arg: &'a T);
}

struct SomeType;

impl<'a> MyTrait<'a, i32> for SomeType {
    fn func1(_arg: i32) {}
    fn func2(_arg: &'a u32) {}
    fn func3(_arg: &'a i32) {}
}

impl<'a> MyTrait<'a, i64> for SomeType {
    fn func1(_arg: i64) {}
    fn func2(_arg: &'a u32) {}
    fn func3(_arg: &'a i64) {}
}

fn main() {
    let dog: Dog = Dog {
        name: "german",
        age: 10,
    };
    let person: Person = Person {
        name: "nachiket",
        age: 22,
    };

    // we had single trait 'BasicInfo' which had common function definitions
    // for two different types: 'Person' and 'Dog'
    // TODO: optional trait methods possible in rust?

    dbg!(dog.name(), dog.age(), person.name(), person.age());
}

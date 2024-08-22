trait MyTrait {
    type AssociatedType; // Object-safe, non-generic associated type

    fn create_associated_type(&self, value: i32) -> Self::AssociatedType;
}

struct MyStruct {
    data: i32,
}

impl<'a> MyTrait for MyStruct where Self: 'a {
    type AssociatedType = AssociatedStruct<'a>;

    fn create_associated_type(&self, value: i32) -> Self::AssociatedType {
        AssociatedStruct {
            my_struct_ref: self,
            value,
        }
    }
}

struct AssociatedStruct<'a> {
    my_struct_ref: &'a MyStruct,
    value: i32,
}

fn main() {
    let my_struct = MyStruct { data: 10 };
    let associated_struct = my_struct.create_associated_type(20);

    println!("MyStruct data: {}", associated_struct.my_struct_ref.data);
    println!("AssociatedStruct value: {}", associated_struct.value);
}
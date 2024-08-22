trait MyTrait {
    type AssociatedType<'a> where Self: 'a;

    fn create_associated_type<'a>(&'a self, value: i32) -> Self::AssociatedType<'a>;
}

struct MyStruct {
    data: i32,
}

impl MyTrait for MyStruct {
    type AssociatedType<'a> = AssociatedStruct<'a>;

    fn create_associated_type<'a>(&'a self, value: i32) -> Self::AssociatedType<'a> {
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
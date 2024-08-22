trait MyTrait<'a> {
    type AssociatedType;

    fn create_associated_type(&'a self, value: i32) -> Self::AssociatedType;
}

struct MyStruct {
    data: i32,
}

impl<'a> MyTrait<'a> for MyStruct {
    type AssociatedType = AssociatedStruct<'a>;

    fn create_associated_type(&'a self, value: i32) -> Self::AssociatedType {
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

    let boxed_dyn: Box<dyn MyTrait<AssociatedType = AssociatedStruct>> = Box::new(my_struct);
}

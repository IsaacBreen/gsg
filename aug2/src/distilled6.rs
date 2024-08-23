trait MyTrait<'a> {
    type AssociatedType: AssociatedTrait;

    fn create_associated_type(&'a self, value: i32) -> Self::AssociatedType;
}

trait AssociatedTrait {
    fn associated_method(&self);
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

impl AssociatedTrait for AssociatedStruct<'_> {
    fn associated_method(&self) {
        println!("AssociatedStruct method");
    }
}

fn main() {
    let my_struct = MyStruct { data: 10 };
    let associated_struct = my_struct.create_associated_type(20);

    println!("MyStruct data: {}", associated_struct.my_struct_ref.data);
    println!("AssociatedStruct value: {}", associated_struct.value);

    let boxed_dyn: Box<dyn MyTrait<AssociatedType = AssociatedStruct>> = Box::new(my_struct);

    fn opaque<'a>(x: impl MyTrait<'a>) -> impl MyTrait<'a> {
        x
    }

    let my_struct = MyStruct { data: 10 };
    let o = opaque(my_struct);
    // let associated_struct = o.create_associated_type(20);
}

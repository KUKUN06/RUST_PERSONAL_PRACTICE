// // // // // //function
// // // // // fn display
// // // // // < t:std::fmt::Debug>
// // // // // (value:t){
// // // // //     println!("{:?}",value);

// // // // // }

// // // // // fn main(){
// // // // //     let q=31;
// // // // //     let p=2.2;
// // // // //     let s="hello";
// // // // //     display(q);
// // // // //     display(p);
// // // // //     display(s);
// // // // // }

// // // // // //point
// // // // // struct point<T>{
// // // // //     x:T,
// // // // //     y:T,
// // // // // }

// // // // // impl<T> point<T>{
// // // // //     fn new(x:T,y:T)->Self{
// // // // //         Self{
// // // // //             x,
// // // // //             y,
// // // // //         }
// // // // //     }
// // // // //     fn get_x(&self)->&T{
// // // // //         &self.x
// // // // //     }
// // // // // }

// // // // // fn main(){
// // // // //     let p=point::new(5,6);
// // // // //     let q=point::new(5.1,5.55);
// // // // //     println!("x of p-> {}",p.get_x());
// // // // //     println!("x of q-> {}",q.get_x());
// // // // // }

// // // // //Generic struct
// // // // use std::fmt::Display;

// // // // #[derive(Debug)]
// // // // struct container<T:Display>{
// // // //     value:T,
// // // // }

// // // // impl<T:Display> container<T>{
// // // //     fn return_value(&self)->&T{
// // // //         &self.value
// // // //     }
// // // //     }


// // // // fn main(){
// // // //     let p=container{value:5};
// // // //     let q=container{value:2.2};
// // // //     let s=container{value:"Hello".to_string()};
// // // //     println!("{}",p.return_value());
// // // //     println!("{}",q.return_value());
// // // //     println!("{}",s.return_value());
// // // // }

// // // //STRUCT WITH TWO GENERICS

// // // use std::fmt::Debug;

// // // #[derive(Debug)]
// // // struct pair<T,U>{
// // //     first:T,
// // //     second:U,
// // // }

// // // impl <T,U> pair<T,U>{
// // //     fn new(first:T,second:U)->Self{
// // //         Self{
// // //             first,
// // //             second,
// // //         }
// // //     }
// // // }

// // // fn main(){
// // //     let p=pair::new("Hii",5);
// // //     let q=pair::new(7,7.3);
// // //     println!("P-->{:?}",p);
// // //     println!("Q-->{:?}",q);
// // // }

// // //GENERIC FUNCTION WITH TRAIT BOUND

// // fn compare<T:PartialOrd>(a:T,b:T)->T{
// //     if a>b{
// //         a
// //     }
// //     else {
// //         b
// //     }

// // }

// // fn main(){
// //     println!("{}",compare(5,6));
// // }

// //Generic on enums

// enum option<T>{
//     some(T),
//     none,
// }

// impl <T> option<T>{
// fn is_some(&self)->bool{
//     match self{
//         option::some(_)=>true,
//         option::none=>false,
//     }
// }
// }

// fn main(){
//     let p=option::some(5);
//     let q:option<i32>=option::none;

//     println!("{}",p.is_some());
//     println!("{}",q.is_some());
// }

// //When using an enum with generics, the compiler can infer the type <T> 
// if the variant holds a value(e.g, some(42)tells rust that T=i32).
// But for variants without data (like none),there's nothing for rust to infer
// from, so you must explicitly specify the type(e.g., let x:option<i32>=option::none)
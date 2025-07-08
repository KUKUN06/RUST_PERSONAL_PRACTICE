// //function
// fn display
// < t:std::fmt::Debug>
// (value:t){
//     println!("{:?}",value);

// }

// fn main(){
//     let q=31;
//     let p=2.2;
//     let s="hello";
//     display(q);
//     display(p);
//     display(s);
// }

// //point
// struct point<T>{
//     x:T,
//     y:T,
// }

// impl<T> point<T>{
//     fn new(x:T,y:T)->Self{
//         Self{
//             x,
//             y,
//         }
//     }
//     fn get_x(&self)->&T{
//         &self.x
//     }
// }

// fn main(){
//     let p=point::new(5,6);
//     let q=point::new(5.1,5.55);
//     println!("x of p-> {}",p.get_x());
//     println!("x of q-> {}",q.get_x());
// }
// //circle
// struct circle{
//     radius:f32,
// }

// impl circle{
//     fn new(radius:f32)->Self{
//         Self{
//             radius,
//         }
//     }
//     fn circumference(&self)->f32{
//         2.0*3.12*self.radius
//     }
// }

// fn main(){
//     let p=circle::new(5.0);
//     println!("Radius is {}\n Circumference is{}",p.radius,p.circumference());
// }

// //counter
// struct counter{
//     count:u8,
// }

// impl counter{
//     fn increment(&mut self){
//         self.count+=1;
//     }
//     fn reset(&mut self){
//         self.count=0;
//     }
// }

// fn main(){
//     let mut p=counter{count:5};
//     println!("Counter before is {}",p.count);
//     p.increment();
//     println!("Counter after increment is {}",p.count);
//     p.reset();
//     println!("Counter after reset is {}",p.count);
// }

// //derive trait
// #[derive(Debug)]
// struct Book{
//     title:String,
//     author:String,
//     pages:u32,
// }

// impl Book{
//     fn new(title:String,author:String,pages:u32)->Self{
//         Self{
//             title,
//             author,
//             pages,
//         }
//     }
//     fn description(&self){
//         println!("{:?}",self);
//     }
// }

// fn main(){
//     let b=Book::new("RustBook".to_string(),"Kukun".to_string(),100);
//     b.description();
// }

// //rectangle
// struct rectangle{
//     length:u8,
//     breadth:u8,
// }

// impl rectangle {
//     fn area(&self)->u8{
//         self.length*self.breadth
//     }
// }

// fn main(){
//     let p= rectangle{length:3,breadth:4};
//     println!("Length is {}\nBreadth is{}\nArea is {}",p.length,p.breadth,p.area());
// }

// //return self
// struct point{
//     x:i32,
//     y:i32,
// }

// impl point{
//     fn new(x:i32,y:i32)->Self{
//         Self{
//             x,
//             y,
//         }
//     }
//     fn move_by(&mut self,dx:i32,dy:i32)->Self{
//         Self{
//             x:self.x+dx,
//             y:self.y+dy,
//         }
//     }
// }

// fn main(){
//     let mut p=point::new(5,6);
//     println!("Point is ({},{})",p.x,p.y);
//     p=p.move_by(1,2);
//     println!("After moving ({},{})",p.x,p.y);
// }
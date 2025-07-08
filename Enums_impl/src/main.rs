// //message
// #[derive(Debug)]
// enum message{
//     text(String),
//     image{url:String,height:i32,width:i32}
// }

// impl message{
//     fn summary(&self){
//         match self{
//             message::text(s)=>println!("{:?}-->{}",self,s),
//             message::image{url:a,height:b,width:c}=>println!("{:?}-->({},{},{})",self,a,b,c),
//         }
//     }
// }

// fn main(){
//     let i=message::image{url:"/url/image.jpg".to_string(),height:30,width:100};
//     let t=message::text("this is a text".to_string());
//     i.summary();
//     t.summary();
// }

// //operation
// enum operation{
//     add(i32,i32),
//     subtract(i32,i32),
//     multiply(i32,i32),

// }

// impl operation{
//     fn execute(&self){
//         match self{
//             operation::add(a,b)=>println!("{}",a+b),
//             operation::subtract(a,b)=>println!("{}",a-b),
//             operation::multiply(a,b)=>println!("{}",a*b),
//         }
//     }
// }

// fn main(){
//     let o=operation::add(5,2);
//     o.execute();
// }

// //result
// #[derive(Debug)]
// enum result{
//     success(String),
//     error(i32),
// }

// impl result{
//     fn report(&self){
//         match self{
//             result::success(s)=>println!("{:?}->{}",self,s),
//             result::error(t)=>println!("{:?}->Error {}",self,t),
//         }
//     }
// }

// fn main(){
//     let r1=result::success("This is success".to_string());
//     let r2=result::error(0);
//     r1.report();
//     r2.report();
// }

// //state
// #[derive(Debug)]
// enum state{
//     start,
//     running(i32),
//     stop,
// }

// impl state{
//     fn describe(&self){
//         println!("State is --> {:?}",self);
//     }

//     fn next(&self)->Self{
//         match self{
//             state::start=>state::running(1),
//             state::running(counter)=>
//             if (*counter<3){
//                 state::running(counter+1)
//             }
//             else{
//                 state::stop
//                 },
//             state::stop=>state::stop,
//         }
//     }
// }

// fn main(){
//     let mut s=state::start;
//     s.describe();
//     s=s.next();
//     s.describe();
//     s=s.next();
//     s.describe();
//     s=s.next();
//     s.describe();
//     s=s.next();
//     s.describe();

// }

// //traffic
// enum traffic{
//     red,
//     yellow,
//     green,
// }
// impl traffic{
//     fn print_status(&self){
//     match self{
//             traffic::red=>println!("This is red light"),
//             traffic::yellow=>println!("This is yellow light"),
//             traffic::green=>println!("This is green light"),
//         }
//         }
// }
// fn main(){
//     let colour=traffic::red;
//     colour.print_status();
    
// } 
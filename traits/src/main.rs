// // //Trait animal

// // trait Animal{
// //     fn speak(&self);
// //     fn jump(&self);

// // }

// // struct dog;
// // struct cat;

// // impl Animal for dog{
// //     fn speak(&self){
// //         println!("Woof Woof!");
// //     }
// //     fn jump(&self){
// //         println!("Dog jump");
// //     }
// // }

// // impl Animal for cat{
// //     fn speak(&self){
// //         println!("Meow meow");
// //     }
// //     fn jump(&self){
// //         println!("Cat is jumping")
// //     }
// // }

// // fn main(){
// //     let tommy=dog;
// //     let kitty=cat;
// //     tommy.speak();
// //     kitty.speak();
// //     tommy.jump();
// //     kitty.jump();
// // }

// //PRINTABLE TRAIT

// trait printable{
//     fn print_info(&self);
// }

// #[derive(Debug)]
// struct book{
//     title:String,
//     author:String,
// }

// #[derive(Debug)]
// struct movie{
//     name:String,
//     director:String,
// }

// impl printable for book{
//    fn print_info(&self){
//         println!("{:?}",self);
//     }
// }

// impl printable for movie{
//    fn print_info(&self){
//         println!("{:?}",self);
//     }
// }
// fn main(){
//     let book1=book{title:"Brief History of Time".to_string(),author:"Stephen Hawkings".to_string()};
//     let movie1=movie{name:"Oppenheimer".to_string(),director:"Christopher Nolan".to_string()};
//     book1.print_info();
//     movie1.print_info();
// }

//Trait DRIVABLE

trait drivable{
    fn start_engine(&mut self);
    fn drive(&mut self);
    fn stop_engine(&mut self);
}

#[derive(Debug)]
enum state{
    start,
    running,
    stopped,
    off,
}

#[derive(Debug)]
struct car{
    state_field:state,
}

#[derive(Debug)]
struct bike;

impl drivable for car{
    fn start_engine(&mut self){
        self.state_field=state::start;
        println!("{:?}",&self);
    }

    fn drive(&mut self){
        self.state_field=state::running;
        println!("{:?}",&self);
    }

    fn stop_engine(&mut self){
        self.state_field=state::stopped;
        println!("{:?}",&self);
    }
}

impl drivable for bike{
    fn start_engine(&mut self){
        println!("Bike doesn't have engine");
    }

    fn drive(&mut self){
        println!("BIke driving");
    }

    fn stop_engine(&mut self){
        println!("Bike stopped though it doesn't have a engine");
    }
}

fn main(){
    let mut maruti=car{state_field:state::off};
    maruti.start_engine();
    maruti.drive();
    maruti.stop_engine();
    let mut ladybird=bike;
    ladybird.start_engine();
    ladybird.drive();
    ladybird.stop_engine();
}
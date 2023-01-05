
mod front_of_house{

    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
    
    fn serve_order(){}
    pub mod back_of_house{
        
        pub fn fix_incorrect_order(){
            super::serve_order();
            serve_order2();
        }

        fn serve_order2(){}
    }

}

use self::front_of_house::hosting;
// pub use self::front_of_house::hosting;
// use std::io::{self, Write}; 将std::io和std::io::Write同时引入作用域；
pub fn some_fn(){
    front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
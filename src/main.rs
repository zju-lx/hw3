#[macro_use]
mod hashmap;
mod stack;
mod myrc;

use myrc::MyRc;
use stack::SimpleStack;
fn main() {
    // Part 1: macro
    let map = hash_map!("red" => 1, "green" => 2, "blue" => 3);
    
    println!("{:?}", map);

    // Part2: MyRc
    println!();
    let rc1 = MyRc::new("Hello");
    println!("counter = {}", rc1.strong_count());
    {
        {
            let rc2 = MyRc::clone(&rc1);
            println!("counter = {}", rc1.strong_count());
        }
        {
            let rc3 = MyRc::clone(&rc1);
            println!("counter = {}", rc1.strong_count());
        }
    }
    println!("counter = {}", rc1.strong_count());

    // Part3: Stack
    println!();
    let mystack = SimpleStack::new();
    mystack.push(1);
    mystack.push(2);
    mystack.push(3);

    println!("Popped value: {:?}", mystack.pop());
    println!("Popped value: {:?}", mystack.pop());

    mystack.push(4);

    println!("Popped value: {:?}", mystack.pop());
    println!("Popped value: {:?}", mystack.pop());
    println!("Popped value: {:?}", mystack.pop());
}

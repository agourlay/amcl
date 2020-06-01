use amcl::Controller;

// example
pub fn main() {
    let mut aimd = Controller::aimd(1.0, 2.0).unwrap();
    for i in 0..=100 {
        if i == 50 {
            println!("congestion detected, going down by half!");
            aimd.update(false);
        } else {
            aimd.update(true);
        }
        println!("{}", aimd.current())
    }
}
use demo_app_kockice::{Cubes};

fn main() {
    let mut cubes = Cubes::new();

    let response = cubes.check([4,2,2]);

    println!(" {} ",  response);
}

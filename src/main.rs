mod coco;
use coco::Coco;

fn main() {
    let c: Coco = match Coco::new("assets/data/instances_val2017.json") {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    println!("{:?}", c.categories);
}

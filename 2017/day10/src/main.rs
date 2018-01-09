#![feature(slice_rotate)]

type Knot = Vec<Increment>;
type Increment = usize;
type Skip = usize;
type Index = usize;

fn main() {
    let input = "46,41,212,83,1,255,157,65,139,52,39,254,2,86,0,204";

    println!("Answer #1: {:?}", tie_knot(input).into_iter().take(2).collect::<Vec<Increment>>());

}

fn tie_knot(input: &str) -> Knot {
    let mut knot: Knot = (0..256).collect();
    let mut skip: Skip = 0;
    let mut index: Index = 0;

    knot
}

fn reverse_slice(index: Index, length: Increment, knot: &mut Knot) -> () {
    knot.rotate(index);

    {
        let z = &mut knot[0..length];
        z.reverse();
    }

    knot.rotate(5 - index);
    println!("{:?}", knot);
}

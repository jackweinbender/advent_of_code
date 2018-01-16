fn main() {
    part_1();
    part_2();
}
fn part_1(){
    let mut whirl = vec![0];
    let skip = 324;
    let mut insert = 1;
    let mut index = 0;

    for i in 0..2017 {
        index = (index + skip) % whirl.len();
        index += 1;
        whirl.insert(index, insert);
        insert += 1;
    }
    println!("Answer #1: {}", whirl[index + 1]);
}
fn part_2(){
    let skip = 324;
    let mut whirl_len = 1;
    let mut index = 0;
    let mut after_zero = 0;

    for i in 1..50000000 {
        index = (index + skip) % whirl_len;
        if index == 0 { after_zero = i }
        index += 1;

        whirl_len += 1;
    }
    println!("Answer #2: {}", after_zero);
}
mod test {
    #[test]
    fn test_whirl(){
        let mut whirl = vec![0];
        let skip = 3;
        let mut insert = 1;
        let mut index = 0;

        println!("");
        for i in 0..40 {
            println!("{:?}", whirl);
            index = (index + skip) % (whirl.len());

            index += 1;
            whirl.insert(index, insert);

            insert += 1;
        }

        assert_eq!(whirl[index + 1], 638);
    }

}
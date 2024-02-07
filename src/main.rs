fn main(){

    let list = vec![1, 2, 3, 4, 32, 12];
    println!("{:#?}", get_largest(list));

}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>)-> T{
    let mut largest = list[0];
    for entry in list{
        if entry > largest{
            largest = entry;
        }
    };
    largest
}
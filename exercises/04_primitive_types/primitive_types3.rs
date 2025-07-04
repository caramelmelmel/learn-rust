fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,13,120,384,24,2942,2432842,249,2482,248,1,2,2,3,3,4,5,9,3,64,4,4,2,4,2,3,3,1,1,2,3,3,3,67,7,23,23,3,4,4,5,5,66,4,6,4,6,6,4,6,4,6,6,6,4,46,46,64,4,6,6,6,4,6,4,4,4,4,4,6,6,6,7,8,8,3,2,3,4,5,6,7,8,9,93,5,5,3,3];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}

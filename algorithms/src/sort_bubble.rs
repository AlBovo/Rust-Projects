/*
* Params : list -> vector of i32 numbers to sort
* Return : vector of i32 number sorted
*/
pub fn bubble_sort(mut list: Vec<i32>) -> Vec<i32> {
    loop{
        let mut finished = false; // variabile di lavoro

        for i in 0..list.len()-1 {
            if list[i] > list[i+1] { // the values must be switched
                let copy = list[i+1];
                list[i+1] = list[i];
                list[i] = copy;
                finished = true;
            }
        }

        if !finished { break; } // if I haven't made any change to the vector
    }
    return list;
}
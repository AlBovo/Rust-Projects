/*
* Params : list -> vector of i32 numbers to sort
* Return : vector of i32 number sorted
*/
pub fn selection_sort(mut list: Vec<i32>) -> Vec<i32> {
    for i in 0..list.len() {
        let mut sel = list[i];  // selected value

        for e in i+1..list.len() {
            if sel > list[e] {  // switch of the values
                let switch = sel;
                sel = list[e];
                list[e] = switch;
            }
        }
        list[i] = sel;  // update value in list[i]
    }
    return list;
}
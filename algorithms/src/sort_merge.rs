/*
* Params : vec of i32 values to sort
* Return : vec of i32 sorted values
*/
pub fn merge_sort(mut list: Vec<i32>) -> Vec<i32> {
    if list.len() == 1 {
        return list;
    }

    let mut list1 = list.split_off(list.len() / 2); // divide the list
    let mut new_list = Vec::new();      // create a new list to return
    list = merge_sort(list.clone());    // sort the first half of the list
    list1 = merge_sort(list1.clone());  // sort the second half of the list

    let mut point_l1 = 0; // logic pointer to the first list
    let mut point_l2 = 0; // logic pointer to the second list
    for _ in 0..(list.len()+list1.len()) {
        if point_l1 == list.len() {         // end of the first list
            new_list.push(list1[point_l2]);
            point_l2 += 1;
            continue;
        }
        else if point_l2 == list1.len() {   // end of the second list
            new_list.push(list[point_l1]);
            point_l1 += 1;
            continue;
        }

        if list[point_l1] < list1[point_l2] { // first list value is better then second
            new_list.push(list[point_l1]);
            point_l1 += 1;
        }
        else {
            new_list.push(list1[point_l2]);
            point_l2 += 1;
        }
    }
    return new_list; // return sorted vec
}
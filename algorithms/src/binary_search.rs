/*
* Params : vector of sorted i32 values sorted in ascended order and i32 value to find 
* Return : index 0-based of the value if its found else size of the vector
*/
pub fn binary_search(lista: Vec<i32>, value: i32) -> usize {
    let mut left = 0;
    let mut right = lista.len();

    while left < right {
        let mid = (left + right) / 2;
        
        if lista[mid] < value {
            left = mid + 1;
        }
        else if lista[mid] > value {
            right = mid;
        }
        else {
            return mid;
        }
    }

    return lista.len();
}
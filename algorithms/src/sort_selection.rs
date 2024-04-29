/*
* Params : lista -> vector of i32 numbers to sort
* Return : vector of i32 number sorted
*/
pub fn selection_sort(mut lista: Vec<i32>) -> Vec<i32> {
    for i in 0..lista.len() {
        let mut sel = lista[i];

        for e in i+1..lista.len() {
            if sel > lista[e] {
                let switch = sel;
                sel = lista[e];
                lista[e] = switch;
            }
        }
        lista[i] = sel;
    }
    return lista;
}
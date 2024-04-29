/*
* Params : lista -> vector of i32 numbers to sort
* Return : vector of i32 number sorted
*/
pub fn bubble_sort(mut lista: Vec<i32>) -> Vec<i32> {
    loop{
        let mut finished = false; // variabile di lavoro

        for i in 0..lista.len()-1 {
            if lista[i] > lista[i+1] {
                let copy = lista[i+1];
                lista[i+1] = lista[i];
                lista[i] = copy;
                finished = true;
            }
        }

        if !finished { break; }
    }
    return lista;
}
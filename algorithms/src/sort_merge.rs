/*
* Params : vec of i32 values to sort
* Return : vec of i32 sorted values
*/
pub fn merge_sort(mut lista: Vec<i32>) -> Vec<i32> {
    if lista.len() == 1 {
        return lista;
    }

    let mut lista1 = lista.split_off(lista.len() / 2);
    let mut new_lista = Vec::new();
    lista = merge_sort(lista.clone());
    lista1 = merge_sort(lista1.clone());

    let mut point_l1 = 0;
    let mut point_l2 = 0;
    for _ in 0..(lista.len()+lista1.len()) {
        if point_l1 == lista.len() {
            new_lista.push(lista1[point_l2]);
            point_l2 += 1;
            continue;
        }
        else if point_l2 == lista1.len() {
            new_lista.push(lista[point_l1]);
            point_l1 += 1;
            continue;
        }

        if lista[point_l1] < lista1[point_l2] {
            new_lista.push(lista[point_l1]);
            point_l1 += 1;
        }
        else {
            new_lista.push(lista1[point_l2]);
            point_l2 += 1;
        }
    }
    return new_lista;
}
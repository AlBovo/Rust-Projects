use std::io;
use rand::{thread_rng, Rng};

mod sort_selection;
mod sort_bubble;
mod sort_merge;
mod binary_search;

use crate::{
    sort_bubble::bubble_sort, 
    sort_selection::selection_sort,
    sort_merge::merge_sort,
    binary_search::binary_search
};

fn main() {
    let mut lista: Vec<i32> = Vec::new();

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Impossibile leggere l'input");

    let dimensione: i32 = match input.trim().parse() {
        Ok(dimensione) => dimensione,
        Err(_) => {
            println!("Input non valido, inserisci un numero intero!");
            return; // Esce dal programma in caso di input non valido
        }
    };

    for _ in 0..dimensione {
        input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Impossibile leggere l'input");

        let numero: i32 = match input.trim().parse() {
            Ok(numero) => numero,
            Err(e) => {
                println!("Input non valido, inserisci un numero intero!, {}", e.to_string());
                return; // Esce dal programma in caso di input non valido
            }
        };

        lista.push(numero);
    }

    let mut lista_sorted = lista.clone();
    lista_sorted.sort();
    let mut rand = thread_rng();
    let random_index = rand.gen_range(0..lista_sorted.len());
    let random_value = lista_sorted[random_index];

    let lista_bubble = bubble_sort(lista.clone());
    let lista_selection = selection_sort(lista.clone());
    let lista_merge = merge_sort(lista.clone());
    let index_search = binary_search(lista_sorted.clone(), random_value);

    assert_eq!(lista_sorted, lista_bubble);
    assert_eq!(lista_sorted, lista_selection);
    assert_eq!(lista_sorted, lista_merge);
    assert_eq!(index_search, random_index);

    println!("OK");
}

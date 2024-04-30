use std::io;

mod sort_selection;
mod sort_bubble;
mod sort_merge;

use crate::{
    sort_bubble::bubble_sort, 
    sort_selection::selection_sort,
    sort_merge::merge_sort
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
    
    let lista_bubble = bubble_sort(lista.clone());
    let lista_selection = selection_sort(lista.clone());
    let lista_merge = merge_sort(lista.clone());

    assert_eq!(lista_sorted, lista_bubble);
    assert_eq!(lista_sorted, lista_selection);
    assert_eq!(lista_sorted, lista_merge);

    println!("OK");
}

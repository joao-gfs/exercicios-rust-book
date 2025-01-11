/*
Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3, 4, 1, 1];
    let mut contagem: HashMap<i32, i32> = HashMap::new();

    let length = v.len();

    v.sort();
    let median: f64;
    let idx1;
    let idx2;

    if length % 2 == 0 {
        idx1 = length / 2;
        idx2 = (length / 2) - 1;
        median = (v[idx1] as f64 + v[idx2] as f64) / 2.0;
    } else {
        idx1 = (length - 1) / 2;
        idx2 = 0;
        median = v[idx1] as f64;
    }

    println!("{v:?}, len = {length}");
    println!("idx1 = {idx1}, idx2 = {idx2}");
    println!("Mediana: {median}");

    for i in &v {
        let count = contagem.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut maior: (i32, i32) = (0, 0);
    for (key, value) in contagem {
        if value > maior.1 {
            maior.0 = key;
            maior.1 = value;
        }
    }

    let (key_maior, value_maior) = maior;
    println!("O numero que mais aparece é o {key_maior}, com {value_maior}");
}
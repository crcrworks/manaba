use std::collections::HashMap;

pub fn timetable(timetable: &HashMap<String, String>) {
    if timetable.is_empty() {
        println!("Timetable is empty");
        return;
    }

    let mut s = String::new();

    let mut numeric_keys = Vec::new();
    let mut non_numeric_keys = Vec::new();

    for key in timetable.keys() {
        if let Ok(n) = key.parse::<u32>() {
            numeric_keys.push((n, key.clone()));
        } else {
            non_numeric_keys.push(key.clone());
        }
    }

    numeric_keys.sort_by_key(|&(n, _)| n);

    non_numeric_keys.sort();

    for (_, num) in numeric_keys {
        s.push_str(&format!("{num}: {}\n", timetable.get(&num).unwrap()));
    }

    for num in non_numeric_keys {
        s.push_str(&format!("{num}: {}\n", timetable.get(&num).unwrap()));
    }

    println!("==== TIMETABLE ====");
    println!("{s}");
}

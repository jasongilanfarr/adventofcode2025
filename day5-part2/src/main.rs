use std::collections::{HashMap, HashSet};

fn sort(page_set: &Vec<u32>, sorting_rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut sorted = Vec::new();

    for entry in page_set {
        if let Some(rules) = sorting_rules.get(entry) {
            if sorted.is_empty() {
                sorted.insert(0, *entry);
                continue;
            }
            let mut inserted = false;
            for i in 0..sorted.len() {
                let current = sorted[i];
                if rules.contains(&current) {
                    continue;
                } else {
                    inserted = true;
                    sorted.insert(i, *entry);
                    break;
                }
            }
            if !inserted {
                sorted.push(*entry);
            }
        } else {
            sorted.insert(0, *entry);
        }
    }

    sorted
}

fn main() {
    let mut has_custom_rule: HashSet<u32> = HashSet::new();
    // key must be _after_ values in the list.
    let mut sorting_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut page_sets: Vec<Vec<u32>> = Vec::new();
    let mut seen_blank = false;

    std::io::stdin().lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            seen_blank = true;
            return;
        }
        if !seen_blank {
            let mut parts = line.split('|');
            let a = parts.next().unwrap().parse().unwrap();
            let b = parts.next().unwrap().parse().unwrap();
            has_custom_rule.insert(a);
            has_custom_rule.insert(b);
            sorting_rules.entry(b).or_insert(Vec::new()).push(a);
        } else {
            let page_set: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            page_sets.push(page_set);
        }
    });

    let mut bad_sets: Vec<Vec<u32>> = Vec::new();
    for page_set in page_sets {
        let mut bad_if: HashSet<u32> = HashSet::new();
        let mut is_bad = false;

        for entry in &page_set {
            // the entry doesn't matter since it doesn't have any rules
            if !has_custom_rule.contains(entry) {
                continue;
            }
            if bad_if.contains(entry) {
                is_bad = true;
                break;
            }
            if let Some(rules) = sorting_rules.get(entry) {
                bad_if.extend(rules);
            }
        }
        if is_bad {
            bad_sets.push(page_set);
        }
    }

    let mut bad_midpoint_sum = 0;
    for page_set in bad_sets {
        let page_set = sort(&page_set, &sorting_rules);
        bad_midpoint_sum += page_set.get(page_set.len() / 2).unwrap();
    }
    println!("{}", bad_midpoint_sum);
}

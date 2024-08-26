use std::collections::HashSet;

// Function to compute the divisors of n
fn divisors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            divs.push(i);
        }
    }
    divs
}

// Function to generate a subgroup of Z_n given a generator
fn generate_subgroup(n: u64, generator: u64) -> HashSet<u64> {
    let mut subgroup = HashSet::new();
    let mut current = 0;
    while !subgroup.contains(&current) {
        subgroup.insert(current);
        current = (current + generator) % n;
    }
    subgroup
}

// Function to generate all subgroups of Z_n
fn generate_all_subgroups(n: u64) -> Vec<HashSet<u64>> {
    let mut subgroups = Vec::new();
    for d in divisors(n) {
        let generator = n / d;
        let subgroup = generate_subgroup(n, generator);
        subgroups.push(subgroup);
    }
    subgroups
}

fn main() {
    let n = 24; // Replace with the desired value of n
    let subgroups = generate_all_subgroups(n);
    println!("Subgroups of Z_{}:", n);
    for subgroup in subgroups {
        let mut subgroup_vec: Vec<u64> = subgroup.into_iter().collect();
        subgroup_vec.sort_unstable();
        println!("{:?}", subgroup_vec);
    }
}




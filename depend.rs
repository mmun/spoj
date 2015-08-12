use std::io::Read;
use std::collections::{HashMap,HashSet};

#[derive(PartialEq)]
enum State { Invalid, Waiting, Visiting, Visited }

type PackageIndex = usize;

struct Package {
    state: State,
    deps: Vec<PackageIndex>
}

fn visit(pkgs:&mut Vec<Package>, index: PackageIndex) {
    match pkgs[index].state {
        State::Invalid | State::Visited => return,
        State::Visiting => pkgs[index].state = State::Invalid,
        State::Waiting => {
            pkgs[index].state = State::Visiting;

            for i in 0..pkgs[index].deps.len() {
                let dep_index = pkgs[index].deps[i];
                visit(pkgs, dep_index);
                if pkgs[dep_index].state == State::Invalid {
                    pkgs[index].state = State::Invalid;
                    return;
                }
            }

            pkgs[index].state = State::Visited;
        },
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();

    let mut pkg_indexes:HashMap<&str, PackageIndex> = HashMap::new();
    let mut pkgs:Vec<Package> = Vec::new();
    
    for line in input.lines() {
        let parts:Vec<&str> = line.split(" ").collect();
        let parts = &parts[0..parts.len()-1];

        for name in parts {
            pkg_indexes.entry(name).or_insert_with(|| {
                pkgs.push(Package { state: State::Invalid, deps: Vec::new() });
                pkgs.len() - 1
            });
        }

        let mut pkg = &mut pkgs[pkg_indexes[parts[0]]];
        let mut deps_index_set:HashSet<PackageIndex> = HashSet::new();
        for part in &parts[1..parts.len()] {
            deps_index_set.insert(pkg_indexes[part]);
        }
        for dep_index in deps_index_set {
            pkg.deps.push(dep_index);
        } 

        pkg.state = State::Waiting;
    }

      for i in 0..pkgs.len() {
          visit(&mut pkgs, i);
      }

    println!("{}", pkgs.iter().filter(|p| p.state == State::Visited).count());
}
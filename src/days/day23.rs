use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::days::Day;

pub const DAY23: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let connections = input.lines().filter_map(|l| l.parse().ok()).collect::<Vec<Connection>>();
    let triplets = Connection::get_triplets(&connections);
    let result = triplets.iter().filter(|[a, b, c]| a.starts_with("t") || b.starts_with("t") || c.starts_with("t")).count();
    println!("There are {} triplets with a t* computer.", result);
}

fn puzzle2(input: &String) {
    let connections = input.lines().filter_map(|l| l.parse().ok()).collect::<Vec<Connection>>();

    let password = Connection::get_lan_password(&connections);
    println!("LAN password: {}", password);
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Connection {
    a: String,
    b: String,
}

impl Connection {
    fn create_connection_map(connections: &Vec<Connection>) -> HashMap<String, HashSet<String>> {
        let mut map = HashMap::new();

        // Note: adding connections to self to make other algorithm slightly simpler (hopefully)
        connections.iter().flat_map(|c| vec![c.a.clone(), c.b.clone()]).for_each(|c| { map.insert(c.clone(), HashSet::from([c.clone()])); });

        for connection in connections {
            if let Some(list) = map.get_mut(&connection.a) {
                list.insert(connection.b.clone());
            }
            if let Some(list) = map.get_mut(&connection.b) {
                list.insert(connection.a.clone());
            }
        }

        map
    }

    fn get_triplets(connections: &Vec<Connection>) -> Vec<[String; 3]> {
        let mut result = vec![];

        let mut computers = connections.iter().flat_map(|c| vec![c.a.clone(), c.b.clone()]).collect::<Vec<_>>();
        computers.sort();
        computers.reverse();
        computers.dedup();

        let mut handled = HashSet::new();

        while let Some(computer) = computers.pop() {
            let mut connected_computers = connections.iter().filter(|c| computer == c.a || computer == c.b).map(|c| if c.a == computer { c.b.clone() } else { c.a.clone() }).filter(|c| !handled.contains(c)).collect::<Vec<_>>();
            connected_computers.sort();
            connected_computers.reverse();
            connected_computers.dedup();

            while let Some(connected_computer) = connected_computers.pop() {
                // Find all computers connected to both
                let mut options = connections.iter().filter(|c| connected_computer == c.a || connected_computer == c.b).map(|c| if c.a == connected_computer { c.b.clone() } else { c.a.clone() }).filter(|c| !handled.contains(c)).collect::<Vec<_>>();
                options.sort();
                options.dedup();

                options.iter().filter(|o| connected_computers.contains(o)).for_each(|o| result.push([computer.to_string(), connected_computer.to_string(), o.to_string()]));
            }

            handled.insert(computer);
        }

        result
    }

    fn get_lan_password(connections: &Vec<Connection>) -> String {
        // Find the largest group of connected computers. Alphabetically join those names with ','
        let mut cliques: Vec<HashSet<String>> = vec![];

        let connection_map = Self::create_connection_map(connections);
        let mut entries = connection_map.iter().collect::<Vec<_>>();
        entries.sort_by(|(l, _), (r, _)| r.cmp(l));

        while let Some((computer, connections)) = entries.pop() {
            // We either find a clique which this computer connects to all of, or we create a new one
            // Since we sorted the list alphabetically, we should end up with the right clique (either intended or by accident, I'll take either).
            let clique_opt = cliques.iter_mut().find(|c| c.intersection(connections).count() == c.len());
            if let Some(clique) = clique_opt {
                clique.insert(computer.clone());
            } else {
                cliques.push(HashSet::from([computer.clone()]));
            }
        }

        let mut group = cliques.iter().max_by(|l, r| l.len().cmp(&r.len())).unwrap().iter().cloned().collect::<Vec<_>>();
        group.sort();
        group.join(",")
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day23::Connection;

    #[test]
    fn test_get_triplets() {
        let connections = TEST_INPUT.lines().filter_map(|l| l.parse().ok()).collect::<Vec<Connection>>();

        assert_eq!(Connection::get_triplets(&connections), vec![
            ["aq","cg","yn"],
            ["aq","vc","wq"],
            ["co","de","ka"],
            ["co","de","ta"],
            ["co","ka","ta"],
            ["de","ka","ta"],
            ["kh","qp","ub"],
            ["qp","td","wh"],
            ["tb","vc","wq"],
            ["tc","td","wh"],
            ["td","wh","yn"],
            ["ub","vc","wq"],
        ])
    }

    #[test]
    fn test_get_lan_password() {
        let connections = TEST_INPUT.lines().filter_map(|l| l.parse().ok()).collect::<Vec<Connection>>();

        assert_eq!(Connection::get_lan_password(&connections), String::from("co,de,ka,ta"));
    }

    const TEST_INPUT: &str = "\
        kh-tc\n\
        qp-kh\n\
        de-cg\n\
        ka-co\n\
        yn-aq\n\
        qp-ub\n\
        cg-tb\n\
        vc-aq\n\
        tb-ka\n\
        wh-tc\n\
        yn-cg\n\
        kh-ub\n\
        ta-co\n\
        de-co\n\
        tc-td\n\
        tb-wq\n\
        wh-td\n\
        ta-ka\n\
        td-qp\n\
        aq-cg\n\
        wq-ub\n\
        ub-vc\n\
        de-ta\n\
        wq-aq\n\
        wq-vc\n\
        wh-yn\n\
        ka-de\n\
        kh-ta\n\
        co-tc\n\
        wh-qp\n\
        tb-vc\n\
        td-yn\n\
    ";
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [a, b] = s.split('-').collect::<Vec<&str>>()[..] else { return Err(format!("Could not parse '{}'", s))};

        Ok(Self { a: a.to_string(), b: b.to_string() })
    }
}
use std::collections::BTreeMap;
use std::iter::successors;

#[derive(Debug)]
pub struct Graph<'a> {
    pub tree: BTreeMap<&'a str, &'a str>,
}

pub fn parse_file(s: &str) -> Graph<'_> {
    let tree = s
        .lines()
        .filter_map(|line| Some((line.find(')')?, line)))
        .map(|(idx, line)| (&line[idx + 1..], &line[..idx]))
        .collect();
    Graph { tree: tree }
}

impl<'a> Graph<'a> {
    fn get_trace(&'a self, to: &'a str) -> impl Iterator<Item = &'a str> {
        successors(Some(to), move |&to| self.tree.get(to).copied()).skip(1)
    }

    pub fn get_nb_orbits(&self) -> usize {
        self.tree
            .keys()
            .fold(0, |x, to| x + self.get_trace(to).count())
    }

    pub fn mesure_path_between(&self, from: &str, to: &str) -> Option<usize> {
        let orbits_to_to: Vec<_> = self.get_trace(to).collect();
        self.get_trace(from)
            .enumerate()
            .filter_map(|(d, p)| Some(d + orbits_to_to.iter().rposition(|&x| x == p)?))
            .next()
    }
}

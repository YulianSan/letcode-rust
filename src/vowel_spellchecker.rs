use std::collections::{HashMap, HashSet};

pub struct Solution;

#[derive(Clone, PartialEq)]
enum Status {
    NotFound,
    SameWord,
    CaseInsensitive,
    RemovedVowel,
}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut ew: HashSet<String> = HashSet::new();
        let mut ci = HashMap::new();
        let mut rv = HashMap::new();

        let mut r = vec![];

        for w in &wordlist {
            ew.insert(w.to_string());

            let lw = w.to_lowercase();
            ci.entry(lw.to_string()).or_insert(w);

            let mut nw = String::new();

            for c in lw.chars() {
                if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                    nw.push('_');
                    continue;
                }
                nw.push(c);
            }

            rv.entry(nw).or_insert(w);
        }

        for q in &queries {
            if ew.contains(q) {
                r.push(q.to_string());
                continue;
            }

            let lq = q.to_lowercase();

            if let Some(lq) = ci.get(&lq) {
                r.push(lq.to_string());
                continue;
            }

            let mut wq = String::new();

            for c in lq.chars() {
                if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                    wq.push('_');
                    continue;
                }
                wq.push(c);
            }

            if let Some(t) = rv.get(&wq) {
                r.push(t.to_string());
                continue;
            }

            r.push(String::new());
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests() {
        let r = Solution::spellchecker(
            vec![
                "KiTe".to_string(),
                "kite".to_string(),
                "hare".to_string(),
                "Hare".to_string(),
            ],
            vec![
                "kite".to_string(),
                "Kite".to_string(),
                "KiTe".to_string(),
                "Hare".to_string(),
                "HARE".to_string(),
                "Hear".to_string(),
                "hear".to_string(),
                "keti".to_string(),
                "keet".to_string(),
                "keto".to_string(),
            ],
        );

        dbg!(r);
    }
}

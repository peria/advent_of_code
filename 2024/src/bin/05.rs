use std::io::BufRead;

fn main() {
    let (rules, updates) = input();
    println!("{}", solve1(&rules, &updates));
    println!("{}", solve2(&rules, &updates));
}

fn input() -> (RuleSet, Vec<Update>) {
    let mut rules = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        if line.len() == 0 {
            break;
        }
        let rule = Rule::from(&line as &str);
        rules.push(rule);
    }

    let mut updates = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        if line.len() == 0 {
            break;
        }
        let update = Update::from(&line as &str);
        updates.push(update);
    }

    (RuleSet { rules }, updates)
}

#[derive(Debug, Copy, Clone)]
struct Rule {
    before: usize,
    after: usize,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let vars: Vec<_> = value.split('|').map(|x| x.parse().unwrap()).collect();
        Rule {
            before: vars[0],
            after: vars[1],
        }
    }
}

impl Rule {
    fn is_correct_order(&self, update: &Update) -> bool {
        let bpos = update.pages.iter().position(|&x| x == self.before);
        let apos = update.pages.iter().position(|&x| x == self.after);

        bpos.is_none() || apos.is_none() || bpos.unwrap() < apos.unwrap()
    }

    fn correct(&self, update: &mut Update) {
        let bpos = update.pages.iter().position(|&x| x == self.before).unwrap();
        let apos = update.pages.iter().position(|&x| x == self.after).unwrap();
        update.pages.swap(bpos, apos);
    }
}

#[derive(Debug, Clone)]
struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    fn is_correct_order(&self, update: &Update) -> bool {
        self.rules.iter().all(|rule| rule.is_correct_order(update))
    }

    fn sort(&self, update: &Update) -> Update {
        let mut update = update.clone();
        loop {
            if let Some(broken_rule) = self.rules.iter().find(|r| !r.is_correct_order(&update)) {
                broken_rule.correct(&mut update);
                continue;
            }
            break;
        }
        update
    }
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<usize>,
}

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        let pages: Vec<_> = value.split(',').map(|x| x.parse().unwrap()).collect();
        Update { pages }
    }
}

impl Update {
    fn get_middle(&self) -> usize {
        let n = self.pages.len();
        self.pages[n / 2]
    }
}

fn solve1(rules: &RuleSet, updates: &Vec<Update>) -> usize {
    updates
        .iter()
        .filter(|x| rules.is_correct_order(x))
        .map(|x| x.get_middle())
        .sum()
}

fn solve2(rules: &RuleSet, updates: &Vec<Update>) -> usize {
    updates
        .iter()
        .filter(|x| !rules.is_correct_order(x))
        .map(|x| rules.sort(x))
        .map(|x| x.get_middle())
        .sum()
}

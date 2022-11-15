pub struct LookAndSeeIterator {
    current: Vec<u8>,
}

impl LookAndSeeIterator {
    pub fn _new(current: Vec<u8>) -> Self {
        if current.iter().any(|&d| d >= 10) {
            panic!("only digits are supported");
        }
        Self { current }
    }

    pub fn new() -> Self {
        Self::_new(vec![1])
    }

    pub fn start_from(start: Vec<u8>) -> Self {
        Self::_new(start)
    }

    fn get_groups_for_current(&self) -> Vec<Vec<u8>> {
        let mut groups: Vec<Vec<u8>> = Vec::new();
        let mut last: Option<u8> = None;
        for &digit in &self.current {
            if last.map(|last| last != digit).unwrap_or(true) {
                // create a new group
                groups.push(vec![digit]);
            } else {
                // use the last group
                let len = groups.len();
                groups[len - 1].push(digit);
            }
            last = Some(digit);
        }
        groups
    }

    fn create_new_from_groups(groups: Vec<Vec<u8>>) -> Vec<u8> {
        let mut numbers = Vec::new();
        for group in groups {
            let count =
                u8::try_from(group.len()).expect(&format!("group had length {}", group.len()));
            let elem = group[0];
            numbers.push(count);
            numbers.push(elem);
        }
        numbers
    }
}

impl Iterator for LookAndSeeIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();
        let counts = self.get_groups_for_current();

        let next = Self::create_new_from_groups(counts);

        self.current = next;

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_n_strings(iter: LookAndSeeIterator, num: usize) -> Vec<String> {
        iter.take(num)
            .map(|v| {
                let strings: Vec<_> = v.into_iter().map(|n| n.to_string()).collect();
                strings.join("")
            })
            .collect()
    }

    #[test]
    fn look_and_see_iterator_test() {
        let first_15: Vec<String> = first_n_strings(LookAndSeeIterator::new(), 15);

        assert_eq!(
            first_15,
            vec![
                "1",
                "11",
                "21",
                "1211",
                "111221",
                "312211",
                "13112221",
                "1113213211",
                "31131211131221",
                "13211311123113112211",
                "11131221133112132113212221",
                "3113112221232112111312211312113211",
                "1321132132111213122112311311222113111221131221",
                "11131221131211131231121113112221121321132132211331222113112211",
                "311311222113111231131112132112311321322112111312211312111322212311322113212221",
            ]
        );
    }

    #[test]
    fn look_and_see_iterator_start_from_test() {
        let degenerate = first_n_strings(LookAndSeeIterator::start_from(vec![2, 2]), 10);

        assert_eq!(
            degenerate,
            vec!["22", "22", "22", "22", "22", "22", "22", "22", "22", "22",]
        );

        let iter1 = first_n_strings(LookAndSeeIterator::start_from(vec![2, 2, 3, 6]), 10);

        assert_eq!(
            iter1,
            vec![
                "2236",
                "221316",
                "2211131116",
                "2231133116",
                "221321232116",
                "22111312111213122116",
                "22311311123112111311222116",
                "221321133112132112311321322116",
                "22111312212321121113122112132113121113222116",
                "2231131122111213122112311311222112111312211311123113322116"
            ]
        );
    }

    #[test]
    #[should_panic]
    fn validation_test() {
        let _iter = LookAndSeeIterator::start_from(vec![1, 2, 3, 11]);
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut l), None) | (None, Some(mut l)) => {
                let (_, rest) = Solution::return_values(l.val, 0);

                if let Some(next) = &mut l.next {
                    next.val += rest;
                } else if rest > 0 {
                    l.next = Some(Box::new(ListNode::new(rest)));
                }

                Some(Box::new(ListNode {
                    val: l.val,
                    next: Solution::add_two_numbers(l.next, None),
                }))
            }
            (Some(mut l1), Some(mut l2)) => {
                let (val, rest) = Solution::return_values(l1.val, l2.val);

                if let Some(next) = &mut l1.next {
                    next.val += rest;
                } else if let Some(next) = &mut l2.next {
                    next.val += rest;
                } else if rest > 0 {
                    l1.next = Some(Box::new(ListNode::new(rest)));
                }

                Some(Box::new(ListNode {
                    val,
                    next: Solution::add_two_numbers(l1.next, l2.next),
                }))
            }
            (None, None) => None,
        }
    }

    pub fn return_values(val1: i32, val2: i32) -> (i32, i32) {
        ((val1 + val2) % 10, (val1 + val2) / 10)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! list {
        () => {
            None
        };
        ($e:expr $(, $rest:expr)*) => {
            Some(Box::new(ListNode {
                val: $e,
                next: list!($($rest),*)
            }))
        };
    }

    #[test]
    fn tests() {
        let x = list![2, 4, 3];
        let y = list![5, 6, 4];
        let z = list![7, 0, 8];

        assert_eq!(Solution::add_two_numbers(x, y), z);

        let x = list![2, 4, 5];
        let y = list![5, 6, 4];
        let z = list![7, 0, 0, 1];

        assert_eq!(Solution::add_two_numbers(x, y), z);
    }
}

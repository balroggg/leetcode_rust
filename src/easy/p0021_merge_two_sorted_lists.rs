// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(x), Some(y)) => Some(Box::new(if x.val <= y.val {
            ListNode {
                val: x.val,
                next: merge_two_lists(x.next, Some(y)),
            }
        } else {
            ListNode {
                val: y.val,
                next: merge_two_lists(Some(x), y.next),
            }
        })),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: l1 = [1,2,4], l2 = [1,3,4]
        // Output: [1,1,2,3,4,4]
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l3 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(merge_two_lists(l1, l2), l3);
    }
    #[test]
    fn test2() {
        // Input: l1 = [], l2 = []
        // Output: []
        assert_eq!(merge_two_lists(None, None), None);
    }
    #[test]
    fn test3() {
        // Input: l1 = [], l2 = [0]
        // Output: [0]
        let l1 = None;
        let l2 = Some(Box::new(ListNode::new(0)));
        let l3 = Some(Box::new(ListNode::new(0)));
        assert_eq!(merge_two_lists(l1, l2), l3);
    }
    #[test]
    fn test4() {
        // Input: l1 = [1,2], l2 = []
        // Output: [1,2]
        let mut l1 = Box::new(ListNode::new(1));
        let n1 = Box::new(ListNode::new(2));
        l1.next = Some(n1);

        let l2 = None;
        let l3 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        assert_eq!(merge_two_lists(Some(l1), l2), l3);
    }
}

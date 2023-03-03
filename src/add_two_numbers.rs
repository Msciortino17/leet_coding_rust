// https://leetcode.com/problems/add-two-numbers/description/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    fn append(&mut self, value: i32) {
        match self.next {
            Some(ref mut next_node) => {
                next_node.append(value);
            }
            None => {
                self.next = Some(Box::new(ListNode::new(value)));
            }
        }
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut vector = Vec::new();

        let mut current = self;
        loop {
            vector.push(current.val);
            if let Some(next_node) = &current.next {
                current = next_node;
            }
            else {
                break;
            }
        }
        vector
    }

    fn from_vec(vector: &Vec<i32>) -> ListNode {
        let mut start_node = ListNode::new(vector[0]);
        if vector.len() == 1 {
            return start_node;
        }

        let mut iterator = vector.iter();
        iterator.next();
        for value in iterator {
            start_node.append(*value);
        }

        start_node
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sum_start_node = Box::new(ListNode::new(0));
    let mut sum_current_node = &mut sum_start_node;

    let mut l1_node_has_value = true;
    let mut l2_node_has_value = true;

    let mut l1_current = l1.unwrap();
    let mut l2_current = l2.unwrap();

    let mut carry_over = 0;

    loop {
        let mut sum = carry_over;
        if l1_node_has_value {
            sum += l1_current.val;
            match l1_current.next {
                Some(l1_next) => l1_current = l1_next,
                None => l1_node_has_value = false
            }
        }
        if l2_node_has_value {
            sum += l2_current.val;
            match l2_current.next {
                Some(l2_next) => l2_current = l2_next,
                None => l2_node_has_value = false
            }
        }

        if sum > 9 {
            sum -= 10;
            carry_over = 1;
        }
        else {
            carry_over = 0;
        }

        sum_current_node.val = sum;

        if !l1_node_has_value && !l2_node_has_value && carry_over == 0 {
            break;
        }
        else {
            sum_current_node.next = Some(Box::new(ListNode::new(0)));
            sum_current_node = sum_current_node.next.as_mut().unwrap();
        }
    }

    Some(sum_start_node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converters() {
        let test_vec = vec![0, 1, 2, 3, 4];
        let node_list = ListNode::from_vec(&test_vec);
        let new_vec = node_list.to_vec();
        assert_eq!(test_vec, new_vec);
    }

    #[test]
    fn example_one() {
        let l1 = Some(Box::new(ListNode::from_vec(&vec![2, 4, 3])));
        let l2 = Some(Box::new(ListNode::from_vec(&vec![5,6,4])));

        if let Some(result) = add_two_numbers(l1, l2) {

            assert_eq!(result.to_vec(), vec![7,0,8]);
        }
    }

    #[test]
    fn example_two() {
        let l1 = Some(Box::new(ListNode::from_vec(&vec![0])));
        let l2 = Some(Box::new(ListNode::from_vec(&vec![0])));

        if let Some(result) = add_two_numbers(l1, l2) {
            assert_eq!(result.to_vec(), vec![0]);
        }
    }

    #[test]
    fn example_three() {
        let l1 = Some(Box::new(ListNode::from_vec(&vec![9,9,9,9,9,9,9])));
        let l2 = Some(Box::new(ListNode::from_vec(&vec![9,9,9,9])));

        if let Some(result) = add_two_numbers(l1, l2) {
            assert_eq!(result.to_vec(), vec![8,9,9,9,0,0,0,1]);
        }
    }
}
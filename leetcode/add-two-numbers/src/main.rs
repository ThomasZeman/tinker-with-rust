#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
    ListNode {
      next,
      val
    }
  }
}

pub struct A {
    pub val: i32
}

impl A {
    fn new(val: i32) -> Self {
        A { val }
    }
}

pub struct Solution {
}

impl Solution {

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let zero: ListNode = ListNode::new( 0, None);
        let mut p1 = l1;
        let mut p2 = l2;
        let mut start: Option<Box<ListNode>> = None;
        let mut curr: Option<Box<ListNode>> = None;
        let mut carryOver = 0;
        while p1.is_some() || p2.is_some() {
            let v1 = p1.unwrap_or(Box::new(ListNode::new( 0, None)));
            let v2 = p2.unwrap_or(Box::new(ListNode::new( 0, None)));
            let sum = v1.val + v2.val + carryOver;
            carryOver = sum / 10;
            let digit = sum % 10;
            print!("{} ", digit);
            let newElement = Box::new(ListNode::new(digit, None));
            let mut ff = Option::Some(newElement);
            let gg = curr.take();
            if gg.is_none() {
                curr = ff;
                start = curr;
            } else {
                gg.unwrap().next = ff;
            }
            p1 = v1.next;
            p2 = v2.next;
        }
        if carryOver != 0 {
            print!("{}", carryOver);
        }
      //  printList(&(*start.unwrap()));
        return start;
    }

    // pub fn foo() -> Option<A> {
    //     let f: &A = &A::new(1);
    //     return Some(*f);
    // }

    pub fn foo2() -> Option<A> {
        let f: A = A::new(1);
        return Some(f);
    }

//     pub fn foo3() -> A {
//         let f: &A = &A::new(1);
//         return (*f).clone();
//     }
}

fn printList(list: &ListNode) {
    print!("{}", list.val);
    match &list.next {
        Some(x) => printList(&x),
        None => return
    }

}

fn main() {
    let l1 = ListNode::new(3, Some(Box::new(ListNode::new(3, None))));
    let l2 = ListNode::new(1, Some(Box::new(ListNode::new(6, None))));
 //   printList(&l1);
    let result = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    printList(&*result.unwrap());
}

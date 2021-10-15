use std::cell::RefCell;
use std::rc::Rc;

type Item<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T>
where
    T: Sized + Clone,
{
    value: T,
    next: Item<T>,
}

impl<T> Node<T>
where
    T: Sized + Clone,
{
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

#[derive(Clone)]
pub struct List<T>
where
    T: Sized + Clone,
{
    first: Item<T>,
    last: Item<T>,
    pub count: usize,
}

impl<T> List<T>
where
    T: Sized + Clone,
{
    pub fn new() -> List<T> {
        List {
            first: None,
            last: None,
            count: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.first.take() {
            Some(there) => there.borrow_mut().next = Some(new_node.clone()),
            None => self.first = Some(new_node.clone()),
        };
        self.count += 1;
        self.last = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|item| {
            if let Some(next) = item.borrow_mut().next.take() {
                self.first = Some(next);
            } else {
                self.last.take();
            }
            self.count -= 1;
            Rc::try_unwrap(item)
                .ok()
                .expect("Bi kedi gördüm san...Bi hata gördüm sanki.")
                .into_inner()
                .value
        })
    }
}
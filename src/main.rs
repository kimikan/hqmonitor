
use std::rc::Rc;
use std::cell::RefCell;

trait I {
    fn write(&mut self, _ : Rc<RefCell<Context>>);
}

struct S;
impl I for S {
    fn write(&mut self, c : Rc<RefCell<Context>>) {
        c.borrow_mut().dump();
        let ref s = c.borrow()._str;
        println!("{}", s);

    }
}

struct Context<'a> {
    _ptr:Rc<RefCell<&'a mut I>>,
    _str:String,
}

trait Fake {
    fn test(self);
}

impl<'a> Fake for Rc<RefCell<Context<'a>>> {
    fn test(self) {
        let ptr = self.borrow().get_ptr();
        ptr.borrow_mut().write(self);
    }
}

impl<'a> Context<'a> {
    fn dump(&mut self) {
        self._str = "final value".to_owned();
    }

    fn get_ptr(&self) -> Rc<RefCell<&'a mut I>> {
        self._ptr.clone()
    }
}

fn main() {
    let mut x = S{};
    let s = Rc::new(RefCell::new(&mut x as &mut I));

    let c = Context{_str:"first value".to_owned(), _ptr:s};

    let ptr = Rc::new(RefCell::new(c));
    ptr.test();
    //x.write(Rc::new(RefCell::new(c)));
}
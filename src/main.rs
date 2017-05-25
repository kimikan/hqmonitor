
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

struct Context {
    _ptr:Rc<RefCell<I>>,
    _str:String,
}

trait Fake {
    fn test(self);
}

impl Fake for Rc<RefCell<Context>> {
    fn test(self) {
        let ptr = self.borrow().get_ptr();
        ptr.borrow_mut().write(self);
    }
}

impl Context {
    fn dump(&mut self) {
        self._str = "final value".to_owned();
    }

    fn get_ptr(&self) -> Rc<RefCell<I>> {
        self._ptr.clone()
    }
}

fn main() {
    let x = S{};
    let s = Rc::new(RefCell::new(x));

    let c = Context{_str:"first value".to_owned(), _ptr:s};

    let ptr = Rc::new(RefCell::new(c));
    ptr.test();
    //x.write(Rc::new(RefCell::new(c)));
}

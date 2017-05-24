# rc-refcell
A callback sample( like: self.member.write(&amp; mut self))

due to the lifetime restriction.  it's a little complicated to write the callback mechanism code like c++ as below.

but we can approch this with Rc<RefCell<T>>.

in c++.
struct A {
   void do() {}
};

sturct B {
    void set_a(A*a) { this->a = a; }
    void do() {
        if (a) { a->do();}
    }
    A *a;
};

int main() {
    A a;
    B b;
    b.set_a(&a);
    b.do();
    return 0;
}

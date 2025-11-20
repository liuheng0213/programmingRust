fn main() {
    println!("Hello, world!");
    // sharedownershipDemo();
    // multiThreadsDemo();

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729); //rust will deref &1009 automatically
    println!("OK");
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;
fn sharedownershipDemo() {
    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    //Cloning an Rc<T> value does not copy the T; instead, it simply
    //creates another pointer to it and increments the reference count.
    let t: Rc<String> = s.clone();//same as Rc::clone(&s)
    let u: Rc<String> = s.clone();
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    //Rust’s memory and thread-safety guarantees depend on ensuring that no value is ever
    //simultaneously shared and mutable. Rust assumes the referent of an Rc pointer might
    //in general be shared, so it must not be mutable.

    // s.push_str(" noodles");
}
fn multiThreadsDemo() {
    println!("=== 1) Rc<T>: single-threaded shared, read-only ===");
    {
        let a = Rc::new(String::from("hello"));
        println!("a contents: {}", a);
        println!("Rc::strong_count(a) = {}", Rc::strong_count(&a)); // 1

        let b = Rc::clone(&a); // increment strong count (another owner)
        println!("b sees: {}", b);
        println!("Rc::strong_count(a) = {}", Rc::strong_count(&a)); // 2

        drop(b); // one owner goes away
        println!(
            "after drop(b), Rc::strong_count(a) = {}",
            Rc::strong_count(&a)
        ); // 1
        // NOTE: Rc<T> gives shared ownership; mutation requires interior mutability (see next).
    }

    println!("\n=== 2) Rc<RefCell<T>>: single-threaded shared, with mutation ===");
    {
        let n = Rc::new(RefCell::new(0));
        let n2 = Rc::clone(&n);
        let n3 = Rc::clone(&n);

        // Borrow mutably through RefCell; all owners see the change
        *n2.borrow_mut() += 5;
        *n3.borrow_mut() += 10;

        println!("value via n = {}", n.borrow()); // 15
        println!("Rc::strong_count(n) = {}", Rc::strong_count(&n)); // 3
        // WARNING: RefCell enforces borrow rules at runtime (panics on violations).
    }

    println!("\n=== 3) Arc<Mutex<T>>: multi-threaded shared, with mutation ===");
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..10 {
            let c = Arc::clone(&counter); // share across threads
            handles.push(thread::spawn(move || {
                let mut guard = c.lock().unwrap(); // lock to mutate
                *guard += 1;
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
        println!("final counter = {}", *counter.lock().unwrap()); // 10
        // Arc = atomic ref-count; Mutex = safe mutation across threads.
    }

    println!("\n=== 4) Weak<T>: break Rc cycles to avoid leaks ===");
    {
        #[derive(Debug)]
        struct Node {
            value: i32,
            next: RefCell<Option<Rc<Node>>>,   // strong to next
            prev: RefCell<Option<Weak<Node>>>, // WEAK to prev to avoid cycle
        }

        let a = Rc::new(Node {
            value: 1,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        });
        let b = Rc::new(Node {
            value: 2,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        });

        // a -> b (strong)
        *a.next.borrow_mut() = Some(Rc::clone(&b));
        // b -> a (weak)
        *b.prev.borrow_mut() = Some(Rc::downgrade(&a));

        println!(
            "a: strong={}, weak={}",
            Rc::strong_count(&a),
            Rc::weak_count(&a)
        );
        println!(
            "b: strong={}, weak={}",
            Rc::strong_count(&b),
            Rc::weak_count(&b)
        );

        // If we had used Rc both ways (a.prev = Rc<a>), that would create a strong cycle -> memory leak.
        // Using Weak breaks the cycle so both can be dropped when last strong owner goes away.
    }
}

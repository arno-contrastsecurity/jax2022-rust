

pub fn intro() {
    let mut a = 5;

    println!("{}", a);

    let mut a = 5;

    for _ in 0..100 {
        a *= 2;

        if a > 1_000_000 && a > 2_000_000 { break; }
    }

    println!("a = {:?}", a);

    let mut arr = [1, 2, 3, 4];
    arr[1] = 7;
    arr = [3, 4, 5, 6];

    let arr = [0;4];

    println!("{:?}", arr);

    let mut v = Vec::new();
    v.push(123);
    println!("{:?}", v);


    let a = vec![6];
    let mut b = a;
    b.push(123);

    println!("{:?}", b);

    let mut s = String::new();
    s.push_str("Hallo ");
    s.push_str("Jax");
    println!("{}", s);

    let s_ref = &s;

    let lit = "Hallo".to_string();

    let name = "ihr alle".to_string();

    println!("{}", greeting(&name));
    println!("{}", greeting(&name));
    println!("{}", greeting("in Mainz und zu Hause"));
}

fn greeting(name: &str) -> String {
    format!("Hallo {}", name)
}

#[cfg(test)]
mod test {
    use super::greeting;

    #[test]
    fn test_greeting() {
        assert_eq!("Hallo a", &greeting("a"));
        assert_eq!("Hallo b", &greeting("b"));
    }
}
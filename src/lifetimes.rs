

pub fn lifetimes() {
    let p1 = Person { name: "a".to_string() };
    let p2 = Person { name: "b".to_string() };

    let family = as_family(&p1, &p2, vec![]);
    println!("{:?}", family);
}

#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct FamilySummary<'a> {
    spouse1: &'a str,
    spouse2: &'a str,
    children: Vec<&'a str>,
}

fn as_family_summary<'a> (spouse1: &'a Person, spouse2: &'a Person, children: Vec<&'a Person>) -> FamilySummary<'a> {
    FamilySummary {
        spouse1: &spouse1.name,
        spouse2: &spouse2.name,
        children: children.iter().map(|p| {
            let s: &str = &p.name;
            s
        }).collect()
    }
}

#[derive(Debug)]
struct Family<'a> {
    spouse1: &'a Person,
    spouse2: &'a Person,
    children: Vec<&'a Person>,
}

fn as_family<'a> (spouse1: &'a Person, spouse2: &'a Person, children: Vec<&'a Person>) -> Family<'a> {
    Family {
        spouse1,
        spouse2,
        children,
    }
}
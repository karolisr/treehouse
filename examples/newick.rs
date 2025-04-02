use treehouse::{Tree, flatten_tree, max_name_len, parse_newick};

fn main() {
    // let data = "(((пять:0.5,Four:0.4,(Two:0.2,One:0.1)Three:0.3)Six:0.6,Seven:0.7)Aštuoni:0.8,九つ:0.9)十:1.0;";
    // let data = "(((One:0.2,Two:0.3)A:0.3,XXX:0.7,(Three:0.5,Four:0.3)B:0.2)C:0.3,пять:0.7,YšY九Y:0.7)D:0.0;";
    let data = "(((five:0.5,four:0.4,(two:0.2,one:0.1)three:0.3)six:0.6,seven:0.7)eight:0.8,nine:0.9)ten:1.0;";
    let data = String::from(data);
    println!("{data}");
    let mut tree = match parse_newick(data) {
        Some(t) => t,
        None => Tree::new(),
    };
    tree.sort(false);
    println!("{}", &tree);
    let chunks = flatten_tree(&tree, 1);
    for chunk in chunks {
        println!("{}", "-".repeat(46));
        for e in chunk {
            println!(
                "{:>3} {:>3} {:<5} {:<10} {:>.4} {:>.4} {:>.4} {}",
                e.parent,
                e.child,
                e.is_tip,
                match e.name {
                    Some(name) => name,
                    None => "None".into(),
                },
                e.x0,
                e.x1,
                e.y,
                match e.y_prev {
                    Some(y_prev) => format!("{:>.4}", y_prev),
                    None => format!("{:<}", "-"),
                },
            );
        }
    }

    let ml = max_name_len(&tree);
    println!("\nmax_name_len = {ml}");
}

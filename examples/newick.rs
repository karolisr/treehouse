use treehouse::{Tree, flatten_tree, max_name_len, parse_newick};

fn main() {
    // let data = "(((пять:0.5,Four:0.4,(Two:0.2,One:0.1)Three:0.3)Six:0.6,Seven:0.7)Aštuoni:0.8,九つ:0.9)十:1.0;";
    // let data = "(((One:0.2,Two:0.3)A:0.3,XXX:0.7,(Three:0.5,Four:0.3)B:0.2)C:0.3,пять:0.7,YšY九Y:0.7)D:0.0;";
    // let data = "(((five:0.5,four:0.4,(two:0.2,one:0.1)three:0.3)six:0.6,seven:0.7)eight:0.8,nine:0.9)root;";
    // let data = "((Five:0.5,Four:0.4,(Two:0.2,One:0.1)Three:0.3)Six:0.6,Seven:0.7,Nine:1.7)unroot:0.0;";
    let data = "(((Five:0.5,Four:0.4,(Two:0.2,One:0.1)Three:0.3)Six:0.6,Seven:0.7)Eight:0.8,Nine:0.9)root;";
    // let data = "((1:0.1,2:0.2)A:0.1,(3:0.3,4:0.4)B:0.1)root:0.0;";
    // let data = "((1:0.1,2:0.2)A:0.1,3:0.3)root:0.0;";
    // let data = "(,,);";
    let data = String::from(data);
    println!("{data}");
    let mut tree = match parse_newick(data) {
        Some(t) => t,
        None => Tree::new(),
    };
    // dbg!(&tree);
    tree.sort(false);
    tree.unroot();
    let ml = max_name_len(&tree);
    println!("\nmax_name_len = {ml}");
    println!("Rooted: {} {:?}", &tree.is_rooted(), &tree.root_node_id());
    println!("{}", &tree);
    let chunks = flatten_tree(&tree, 1);
    for chunk in chunks {
        println!("{}", "-".repeat(96));
        for e in chunk {
            println!(
                "{:>25} {:>25} {:<5} {:<10} {:>.4} {:>.4} {:>.4} {}",
                format!("{:?}", e.parent),
                format!("{:?}", e.child),
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
}

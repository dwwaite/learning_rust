fn main() {

    /* What happens when we want to declare a struct that contains a reference to a different
     *    instance of the same struct type?
     * We cannot just use the Node implementation below directly, as the size of a Node depends
     *    on the size of two more Nodes...cannot compute it.
     */

     //let n = Node();
     let mut n = Node::new("Help");
     n.set_left(Node::new("me"));
     n.set_right(Node::new("Obi wan!"));
     println!("{:#?}", n);

     /* Now we can extend this further, with automatic internal sorting on the entries as more are added
      *    See the implementation below for how if/match are used to create a recursive lookup through the
      *    previously-instantiated chain of Nodes.
      */
    let mut new_node = Node::new("cccc");
    new_node.insert("ddd");
    new_node.insert("bbb");
    new_node.insert("aaa");
    println!("{:#?}", new_node);

    // That works, but it's not super easy to read the output. Can implement an in-order traversal function
    //    that just prints the `word` value, rather than the full debug.
    println!("In-order traversal:");
    print!("\t");
    new_node.visit();
    println!("");
}

//region Node and NodeBox

// Here's the basic struct, which contains references to different struct instances
#[derive(Debug)]
struct Node {
    word: String,
    left: NodeBox,
    right: NodeBox
}

/* To launch it, we have to define a new type that wraps it in a Box, which is a point
 *    with a known size at compile time.
 */
type NodeBox = Option<Box<Node>>;

impl Node {
    fn new(s: &str) -> Node {
        Node{word: s.to_string(), left: None, right: None}
    }

    /* How we work this is to make a function which takes a Node, but internally Boxes it
     *    for storage within the root Node object.
     */
    fn box_node(n: Node) -> NodeBox {
        Some(Box::new(n))
    }

    fn set_left(&mut self, n: Node) {
        self.left = Self::box_node(n);
    }

    fn set_right(&mut self, n: Node) {
        self.right = Self::box_node(n);
    }

    fn insert(&mut self, new_word: &str) {
        if new_word < &self.word {
            /* A recursive implementation, where if the value should sort to the left of the current
             *    Node then we try to `insert()` on the left to the Node.
             */
            match self.left {
                Some(ref mut n) => n.insert(new_word),
                None => self.set_left(Self::new(new_word))
            }
        } else {
            // Otherwise, we shunt to the right in the same manner
            match self.right {
                Some(ref mut n) => n.insert(new_word),
                None => self.set_right(Self::new(new_word))
            }
        }
    }

    fn visit(&self) {
        // Same kind of approach - if values either side exist we recursively visit them.
        if let Some(ref left_node) = self.left {
            left_node.visit();
        }
        print!("{}, ", self.word);
        if let Some(ref right_node) = self.right {
            right_node.visit();
        }
    }
}

//endregion
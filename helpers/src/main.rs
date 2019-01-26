use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    freq: i32,
    ch: Option<char>,
    code: bool,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

fn new_node(freq: i32, ch: Option<char>) -> Node {
    Node {
        freq: freq,
        ch: ch,
        code: false,
        left: None,
        right: None
    }
}

fn new_box(node: Node) -> Box<Node> {
    Box::new(node)
}

fn frequency(word: &str) -> HashMap<char, i32> {
    // returns map
    let mut occurences = HashMap::new();
    for c in word.chars() {
        let occurence = occurences.entry(c).or_insert(0);
        *occurence += 1;
    }
    occurences
}

fn assign_codes(node: &Box<Node>, codes: &mut HashMap<char, String>, s: String) {
    if let Some(ch) = node.ch {
        codes.insert(ch, s);
    } else {
        if let Some(ref left) = node.left {
            assign_codes(left, codes, s.clone() + "0");
        }
        if let Some(ref right) = node.right {
            assign_codes(right, codes, s.clone() + "1");
        }
    }
}

fn huffman(original_string: &str) {
    // Huffman Algorithm

    // calculate occurences of each character
    let freq = frequency(original_string);
    let mut freq_vec: Vec<Node> = freq.iter()
        .map(|node| new_node(*(node.1), Some(*(node.0))))
        .collect();

    while freq_vec.len() != 1 {
        freq_vec.sort_by(|a, b| (&b.freq).cmp(&a.freq));
        // take two ones with lowest occurences
        let a = freq_vec.pop().unwrap();
        let b = freq_vec.pop().unwrap();
        // sum their occurences and treat them as single one
        let mut c = new_box(new_node(a.freq + b.freq, None));
        (*c).left = Some(new_box(a));
        (*c).right = Some(new_box(b));
        freq_vec.push(*c);
    }

    let root = new_box(freq_vec.pop().unwrap());
    let mut codes: HashMap<char, String> = HashMap::new();
    assign_codes(&root, &mut codes, String::new());
    let efficency = calculate_encoding_efficency(original_string.len(), &freq, &codes);

    let encoded = encode_string(&original_string, &codes);
    println!("Encoding `{}` using Huffman Algorithm.", original_string);
    println!("{:?}", codes);
    println!("{}", encoded);
    println!("Encoding finished, with efficency {}%", efficency);
}

fn encode_string(original_string: &str, codes: &HashMap<char, String>) -> String {
    let mut encoded = String::new();
    for char in original_string.chars() {
        encoded = encoded + " " + codes.get(&char).unwrap();
    }

    encoded
}

fn calculate_encoding_efficency(
    total: usize, freq: &HashMap<char, i32>, codes: &HashMap<char, String>
) -> f64 {
    // calculate entropy
    let mut entropy: f64 = 0.0;
    for &freq_val in freq.values() {
        let probability = freq_val as f64 / total as f64;
        entropy -= probability * probability.log2();
    }

    let mut expected_bit_length: f64 = 0.0;
    for (key, &freq_val) in freq.iter() {
        let probability = freq_val as f64 / total as f64;
        let length = codes.get(key).unwrap().chars().count() as f64;
        dbg!(key);
        dbg!(probability);
        dbg!(length);
        expected_bit_length += probability * length;
    }

    dbg!(entropy);
    dbg!(expected_bit_length);

    entropy / expected_bit_length * 100.0
}

fn main() {
    let string_1 = "abbcaaacccdabbca";
    huffman(string_1);
    println!("\n");

    // let string_2 = "145213211122235412";
    // huffman(string_2);
    // println!("\n");
    //
    // let string_3 = "zxcvvxzcxvzxcxczvx";
    // huffman(string_3);
    // println!("\n");
}

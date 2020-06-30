
mod post;

fn main() {
    let text = "Hello world.";
    println!("{}", text);

    let post = post::new();
    let valid = post.is_valid();
    println!("{}", valid);
}


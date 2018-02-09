extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let mut post = post.reject();

    post.add_text(" and it was tasty.");

    let post = post.request_review();
    
    let post = post.approve();

    let post = post.final_approve();

    println!("{}", post.content());

    assert_eq!("I ate a salad for lunch today and it was tasty.", post.content());
}

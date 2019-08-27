use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch today");

    let approved_post = post.request_review().approve();

    assert_eq!("I ate salad for lunch today", approved_post.content());
}

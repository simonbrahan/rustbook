use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch").expect("Unable to write to new post");
    assert_eq!("Unpublished draft", post.content());

    post.request_review();
    assert_eq!("Draft pending review", post.content());

    post.reject();
    assert_eq!("Unpublished draft", post.content());

    post.request_review();
    assert_eq!("Draft pending review", post.content());

    post.approve();
    assert_eq!("Draft pending review", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch", post.content());

    match post.add_text("I ate more salad for dinner") {
        Ok(_) => println!("Shouldn't have worked"),
        Err(_) => println!("Already approved, so can't add text")
    };
}

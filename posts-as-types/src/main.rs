use blog::Post;

fn main() {
    let mut draft = Post::new();

    draft.add_text("I ate salad for lunch today");

    let pending_review = draft.request_review();

    let rejected_draft = pending_review.reject();

    let pending_review = rejected_draft.request_review();

    let approved_post = pending_review.approve();

    assert_eq!("I ate salad for lunch today", approved_post.content());
}

use blog::{MaybePublished, Post};

fn main() {
    let mut draft = Post::new();

    draft.add_text("I ate salad for lunch today");

    let pending_review = draft.request_review();

    let rejected_draft = pending_review.reject();

    let pending_review = rejected_draft.request_review();

    let approved_by_one = match pending_review.approve() {
        MaybePublished::NotPublished(post) => post,
        MaybePublished::Published(_) => panic!("Shouldn't be published as only approved once."),
    };

    let approved_by_two = match approved_by_one.approve() {
        MaybePublished::NotPublished(_) => panic!("Should be published now."),
        MaybePublished::Published(post) => post,
    };

    assert_eq!("I ate salad for lunch today", approved_by_two.content());
}

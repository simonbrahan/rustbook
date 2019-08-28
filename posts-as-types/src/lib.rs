pub enum MaybePublished {
    Published(Post),
    NotPublished(PendingReviewPost),
}

pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            num_approvals: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    num_approvals: u32,
}

impl PendingReviewPost {
    pub fn approve(self) -> MaybePublished {
        if self.num_approvals > 0 {
            MaybePublished::Published(Post {
                content: self.content,
            })
        } else {
            MaybePublished::NotPublished(PendingReviewPost {
                content: self.content,
                num_approvals: self.num_approvals + 1,
            })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

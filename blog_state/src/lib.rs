/* Blog post's state management */
/* Here is an example of it by using State trait */
// the downsides:
// a bit redundant with some logic duplicated across implementations for every struct that does so Post
// states are inextricably connected to each other so later modification will be a lot of hassle

#![allow(unused)]

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // create a new post whose state is Draft
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // added the unwrap because you know it will never panic (state will always contain a Some value)
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // to get ownership of the state val, set state to None temporarily rather than setting it directly
        // this ensures the state will never change backwards (e.g. Published => Draft)
        // take(): (&mut self) -> Option<T>
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // this needs to take ownership of the state val
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // in effect, no effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // in effect, no effect
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    // in effect, no effect
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // in effect, no effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // overrides the default implementation
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

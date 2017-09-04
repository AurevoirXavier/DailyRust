//trait State {
//    fn request_review(self: Box<Self>) -> Box<State>;
//
//    fn approve(self: Box<Self>) -> Box<State>;
//
//    fn content<'a>(&self, post: &'a Post) -> &'a str {
//        ""
//    }
//}

pub struct Post {
    //    state: Option<Box<State>>,
    content: String
}

//struct Draft {}

//struct PendingReview {}

//struct Published {}

pub struct DraftPost {
    content: String
}

pub struct PendingReviewPost {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            //            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    //    pub fn add_text(&mut self, text: &str) {
    //        self.content.push_str(text);
    //    }

    pub fn content(&self) -> &str {
        &self.content
    }

    //    pub fn request_review(&mut self) {
    //        if let Some(s) = self.state.take() {
    //            self.state = Some(s.request_review())
    //        }
    //    }
    //
    //    pub fn approve(&mut self) {
    //        if let Some(s) = self.state.take() {
    //            self.state = Some(s.approve())
    //        }
    //    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}

//impl State for Draft {
//    fn request_review(self: Box<Self>) -> Box<State> {
//        Box::new(PendingReview {})
//    }
//
//    fn approve(self: Box<Self>) -> Box<State> {
//        self
//    }
//}

//impl State for PendingReview {
//    fn request_review(self: Box<Self>) -> Box<State> {
//        self
//    }
//
//    fn approve(self: Box<Self>) -> Box<State> {
//        Box::new(Published {})
//    }
//}

//impl State for Published {
//    fn request_review(self: Box<Self>) -> Box<State> {
//        self
//    }
//
//    fn approve(self: Box<Self>) -> Box<State> {
//        self
//    }
//
//    fn content<'a>(&self, post: &'a Post) -> &'a str {
//        &post.content
//    }
//}

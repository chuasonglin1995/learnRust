
pub struct Post {
	content: String,
}

pub struct DraftPost {
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

	// pub fn add_text(&mut self, text: &str) {
	// 	self.content.push_str(text);
	// }

	// pub fn request_review(&mut self) {
	// 	// take() method to take the Some value out of the state field and leave a None in its place
	// 	if let Some(s) = self.state.take() { 
	// 		self.state = Some(s.request_review())
	// 	}
	// }

	// pub fn approve(&mut self) {
	// 	if let Some(s) = self.state.take() {
	// 		self.state = Some(s.approve())
	// 	}
	// }

	// pub fn content(&self) -> &str {
	// 	self.state.as_ref().unwrap().content(self)
	// }
}

impl DraftPost {
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}

	pub fn request_review(self) -> PendingReviewPost {
		PendingReviewPost {
			content: self.content,
		}
	}
}

pub struct PendingReviewPost {
	content: String,
}

impl PendingReviewPost {
	pub fn approve(self) -> Post {
		Post {
			content: self.content,
		}
	}
}

// ======= previous imeplementation =========
// trait State {
// 	fn request_review(self: Box<Self>) -> Box<dyn State>;
// 	fn approve(self: Box<Self>) -> Box<dyn State>;
// 	fn content<'a>(&self, post: &'a Post) -> &'a str {
// 		""
// 	}
// }

// struct Draft {}

// impl State for Draft {
// 	fn request_review(self: Box<Self>) -> Box<dyn State> {
// 		Box::new(PendingReview {})
// 	}

// 	fn approve(self: Box<Self>) -> Box<dyn State> {
// 		self
// 	}
// }

// struct PendingReview {}

// impl State for PendingReview {
// 	fn request_review(self: Box<Self>) -> Box<dyn State> {
// 			self // no transformation, just return itself
// 	}

// 	fn approve(self: Box<Self>) -> Box<dyn State> {
// 		Box::new(Published {})
// 	}
// }

// struct Published {}

// impl State for Published {
// 	fn request_review(self: Box<Self>) -> Box<dyn State> {
// 			self
// 	}

// 	fn approve(self: Box<Self>) -> Box<dyn State> {
// 			self
// 	}

// 	fn content<'a>(&self, post: &'a Post) -> &'a str {
// 		&post.content
// 	}
// }

// ====== implementation using enums =========

// Alternative: Using Enums
// Its possible. The disadvantage is that every place that checks the value of the enum will need a match expression or similar to handle evrey possible variant.

// enum PostState {
// 	Draft,
// 	PendingReview,
// 	Published,
// }

// pub struct Post {
// 	state: PostState,
// 	content: String,
// }

// impl Post {
// 	pub fn new() -> Post {
// 			Post {
// 					state: PostState::Draft,
// 					content: String::new(),
// 			}
// 	}

// 	pub fn add_text(&mut self, text: &str) {
// 			if let PostState::Draft = self.state {
// 					self.content.push_str(text);
// 			}
// 	}

// 	pub fn content(&self) -> &str {
// 			match self.state {
// 					PostState::Published => &self.content,
// 					_ => "",
// 			}
// 	}

// 	pub fn request_review(&mut self) {
// 			if let PostState::Draft = self.state {
// 					self.state = PostState::PendingReview;
// 			}
// 	}

// 	pub fn approve(&mut self) {
// 			if let PostState::PendingReview = self.state {
// 					self.state = PostState::Published;
// 			}
// 	}
// }
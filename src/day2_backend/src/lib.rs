use std::cell::{RefCell, RefMut};

thread_local! {
    static POSTS: RefCell<Vec<String>> = RefCell::default();
}


#[ic_cdk::query]
fn greet(name: String, birth_day: i8) -> String {
    format!("Hello, {} {}!", name, birth_day)
}

#[ic_cdk::update]
fn add_post(post: String) {
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        posts.borrow_mut().push(post)
    });
}

#[ic_cdk::query]
fn read_posts() -> Vec<String> {
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        posts.borrow().clone()
    })
}

#[ic_cdk::update]
fn clear_posts() {
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        *posts.borrow_mut() = Vec::new()
    });
}

#[ic_cdk::update]
fn delete_post(post_id: usize) {
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        posts.borrow_mut().remove(post_id)
    });
}

#[ic_cdk::update]
fn update_post(post_id: usize, new_value: String) {
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        let mut binding: RefMut<Vec<String>> = posts.borrow_mut();
        let post: Option<&mut String> = binding.get_mut(post_id);
        let old_post: &mut String = post.unwrap();
        *old_post = new_value;
    });
}
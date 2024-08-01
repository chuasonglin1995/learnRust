// qn: when is unit-like struct useful?

struct User {
    active: bool,
    username: String, // &str, Will error out because we want each instance of struct to own all of its data
    email: String,
    sign_in_count: u64,
}

fn main() {

    // ====== STRUCTS =======
    // Note that the entire instance must be mutable. 
    // Rust does not allow us to mark only certain fields as mutable
    let mut user1 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@exmaple.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("someone"),
        String::from("someone@exmaple.com"),
    );

    // Note that after we do this, we can no longer use user1 as a whole
    // Because the string in username field has been moved from user1 to user3
    // If we have given user3 new string value for email and username, reusing acgive and sign_in_count is okay
    // active and sign_in_count are types that implement the `COPY` trait. 
    // string uses MOVE trait
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // ====== TUPLE STRUCTS =======
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,10);

    // ====== UNIT-LIKE STRUCTS WITHOUT ANY FIELDS =======
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

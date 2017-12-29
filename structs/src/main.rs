 struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
struct Point(i32, i32, i32);
fn main() {
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    let mut user2 = build_user(String::from("juan@memes.com"), String::from("please123"));
    
    let user3 = User {
        email: String::from("fersot100@yahoo.com"),
        ..user2
    };
    
    let origin = Point(0,0,0);

    user2.username = String::from("JuanTheMagician");
    
    println!("{}", user2.username);
}

fn build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
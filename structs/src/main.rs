struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("xx@xx.xx"),
        username: String::from("user123").
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("yy@yy.zz");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("haha@haha.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("zz@zz.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User  {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_short(email: String, username: String) -> User {
    User {
        email,
        username, 
        active: true,
        sign_in_count: 1,
    }
}
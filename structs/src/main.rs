struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user("michaelbkats@gmail.com", "bearcat");
    println!(
        "\nemail: {} \nusername: {} \nsign in count: {} \nactive: {}",
        user2.email, user2.username, user2.sign_in_count, user2.active
    );

    // structure update syntax
    let user2 = User {
        email: String::from("newEmail@gmail.com"),
        ..user2
    };
    println!(
        "\nemail: {} \nusername: {} \nsign in count: {} \nactive: {}",
        user2.email, user2.username, user2.sign_in_count, user2.active
    );

    {
        // tuple structs
        // useful when you want to give the whole tuple a name and make the tupe a different typ from other tuples
        // and when naming each field as in a regular struct would be verbose or redundant
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        // tuple structs
        // a bit odd that it looks just like calling a function when using a Tuple struct
        // tuple structs are like structs without named fields
        let _black = Color(0, 0, 0);
        let _origin = Point(0, 0, 0);
    }

    {
        // unit like structs without any fields:
        struct AlwaysEqual;

        let _subject = AlwaysEqual;
    }
}

fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1,
    }
}

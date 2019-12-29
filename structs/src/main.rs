#[derive(Debug)]
struct User {
    active: bool,
    email: String,
    name: String,
    sign_in_count: u64,
}

fn main() {
    let mut user = create_user("random@email.com", "Neucoas", false);
    let clone = clone_user(&user);

    // activate user
    user.active = true;
    user.sign_in_count = user.sign_in_count + 1;

    append_text(&mut user.name, " Neuco");

    println!("User = {:?}", user);
    println!("Clone = {:?}", clone);
}

fn create_user(email: &str, name: &str, active: bool) -> User {
    User {
        active,
        email: String::from(email),
        name: String::from(name),
        sign_in_count: 0,
    }
}

fn clone_user(user: &User) -> User {
    // We can't use struct destructuring
    // because if we do, ..user
    // we "move" the user
    
    User {
        active: user.active,
        email: String::from(&user.email),
        name: String::from(&user.name),
        sign_in_count: 0,
    }
}

fn append_text(s: &mut String, data: &str) -> () {
    s.push_str(data);
}

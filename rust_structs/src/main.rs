/*struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rect1  = Rectangle {
        width: 3,
        height: 5
    };

    println!(
        "The area of the rectangle is {:#?} square pixels.",
        rect1
        //area(&rect1)
    );
}

/*fn area(dimensions: &Rectangle) -> u32 {
   dimensions.width * dimensions.height
}*/


/*fn struct_one() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    /*let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };*/
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
*/
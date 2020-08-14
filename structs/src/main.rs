// struct User{
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// let user1 = User {
//     email: String::from("carinha@nois.com"),
//     username: String::from("carinha"),
//     active: true,
//     sign_in_count: 1,
// };

// let user2 = User {
//     email: String::from("maisoutro@oi.com"),
//     username: String::from("user"),
//     active: user1.active,
//     sign_in_count:user1.sign_in_count,
// };

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);



// user1.email = String::from("mudando@hotmail.com");

// fn build_user(email: String, username : String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.",
            area(width,height))
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
pub fn authenticate(username: &str, password: &str) -> bool {

    if username == "admin" && password == "password" {

        true
    } else {

        false
    }
}

fn main() {
    let url = "http://localhost:8082/app/users/1/";
    let split_url: Vec<&str> = url.rsplitn(3, '/').collect();
    println!("{:?}", split_url);
    let c = "ddd";
    println!("{:?}", c.parse::<i64>())
}

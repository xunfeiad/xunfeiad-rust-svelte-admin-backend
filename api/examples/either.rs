macro_rules! blackhole {
    ($tt:tt) => {
      println!($tt)  
    };
}
fn main(){
    blackhole!("string"suffiex);
}
fn main() {
    let mut  delta:f32 = 0.0;
    let   chunknumber :i64 = 1000000;
    let   a:f32 = 10.0;
    let  b:f32 = 20.0;
    //a < b
    delta =   ((a-b) / chunknumber as f32).abs();
    let mut result : f32  = 0.0;
    for i in 1..chunknumber {
        result += delta *  thefunction(  a + delta * i as f32 )
    }
    println!("{}" , result);
}
fn thefunction(x:f32) -> f32 {
    return x * x;
}

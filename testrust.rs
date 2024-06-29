fn main() {
    println!("Domain expansion");
    let v : i8 = 123;
    println!("the value of v is {} ",v);
    const VINE : u32 =800;
    tup();
    array();
    mutable_tup()
} 
fn tup()
{
    let line: (char,i64,f32) =('a',553,69.99);
    println!("the values in the tupple are{:?}",line);
    let first=line.1;
    let second=line.0;
    let third= line.2;
    println!("the value of the first element is {}",first);
    println!("the value of the first element is {} {} {}",first, second, third);
    println!("the value of the first element is {}",second);
    println!("the value of the first element is {}",third);
}
fn mutable_tup()
{
    let mut aho:(i8,char,i32)=(56,'o',45);
    println!("the values in the tupple are{:?}",aho);
    aho.0 = 23;
    println!("the values in the tupple are{:?}",aho);
    aho.1= 'a';
    println!("the values in the tupple are{:?}",aho);
    aho.2= 89;
    println!("the values in the tupple are{:?}",aho);
}
fn array()
{
    let dark=[2,563636];
    println!("the values of the array is{:?}",dark);
    let dim=dark[0];
    let tim=dark[1];
    println!("the values of the array is{}",dim);
    println!("the values of the array is{}",tim);
}
//my contribution
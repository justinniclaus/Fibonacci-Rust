fn fibonacci(value: i32) -> i32
{
    if value <= 0 
    {
        return 0;
    }
    else if value == 1
    {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _i in 2..=value
    {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main()
{
    println!("The value is {}.", fibonacci(3));
}

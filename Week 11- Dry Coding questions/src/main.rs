//question 1 (failed) ans1 corr, ans 2 inc
//correct answer: (7, 14)
/* 
fn main()
{
    let mut wood:i32 = 35;
    bush(&mut wood);
    wood = wood * 2;
    println!("The value of wood is: {}", wood);
}

fn bush(plank:&mut i32)
{
    *plank = *plank/5;
    let wood = *plank/3;
    println!("The value of plank is {}", plank);
}
*/

//question 2 (correct) ans1, ans 2: (5, 75)

/*
fn main() 
{
    let mut link:i32 = 25;
    sledge(link);
    link = link * 3;
    println!("The value of link is {}", link);
}

fn sledge(mut go_link:i32)
{
    go_link = go_link / 5;
    println!("Go link value is {}", go_link);
}
*/

//question 3 (correct) ans1: (ta Clau)

/*
fn main()
{
    let first = "Santa Claus".to_string();

    let noel = &first[3..10];

    println!("{}", noel);
}
*/

//question 4 (Correct) ans1, ans2: (Yemi, Sam, Tola), 3
/*
fn main()
{
    let data = ["Ade", "Ola", "Joye", "Adam", "Yemi", "Sam", "Tola"];
    pass_me(&data[4..]);
}

fn pass_me(use_data:&[&str])
{
    println!("The length of the use_data is {:?}", use_data.len());
    println!("{:?}", use_data);
}
*/

//question 5 (correct), ans1, ans2, ans3: (23, 24, 25)
/*
fn main()
{
    for magic_key in 20..29
    {
        if magic_key <= 25
        {
            continue;
        }
        println!("Key is {}", magic_key-3);
    }
}
*/

//question 6 (failed) ans1, ans2, ans3, ans4 : (43, 36, 29, 22)

/*
fn main() 
{
    let mut lab = 15;
    let mut class = 50;
    let mut min = 4;
    let mut max = 7;

    while lab < class
    {
        lab += min;
        class -= max;
        println!("The value of class is {}", class);
    }
}
*/

//question 7 (failed) ans1, ans2, ans3, ans4: (47, 48, 48, 49)
/*
fn main()
{
    for x in 29..31
    {
        for mut m in 15..17
        {
            m+=3;
            let z = m + x;
            println!("The value of z is {}", z);
        }
    }
}
*/

//question 8 (correct) ans1,ans2,ans3,ans4: (39,40,40,31)

/*
fn main()
{
    for num1 in 8..10
    {
        for num2 in 16..17
        {
            for num3 in 15..17
            {
                let val = num1 + num2 + num3;
                println!("{}", val);
            }
        }
    }
}
*/

//question 9 (failed) ans1 corr, ans2, ans3: (41,20,60)
fn main()
{
    let mut game:i32 = 25;
    let mut two:i32 = 15;

    if game > 0
    {
        game+= 3;
        two -= 2;
        let grass = game + two;
        game = grass/2; //20.5 will be truncated to 20 not 21 semire!!!!
        two = game * 3;
        println!("grass, game and two are {}, {}, {}", grass, game,two);
    }
}
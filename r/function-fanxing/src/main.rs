fn main() {
    //验证 数组通过下标获取是不是可以获取所有权，上课是讲过，元素加入数组后，所有权移入数组，无法再取出来
    /*
        let list = vec![4, 2, 5, 3, 8, 0];
        let index_item = first(&list);
        println!("{index_item}");

        let mut studentA = Student {
            name: "john".to_string(),
            age: 18,
        };
        let mut students: Vec<Student> = Vec::new();
        students.push(studentA);
        studentA = first_student(&students);
        println!("{:?}", studentA);
    */

    //函数泛型1，返回不可变引用
    let list_number = vec![4, 2, 5, 3, 8, 0];
    let largest_number = largestT(&list_number);
    println!("{:?}", largest_number);
    let list_str = vec!["s", "z", "a", "t"];
    let largest_str = largestT(&list_str);
    println!("{:?}", largest_str);

    //函数泛型2，返回所有权
    let list_str = vec!["s", "z", "a", "t"];
    let largest_str = largest_tmut(&list_str);
    println!("{:?}", largest_str);
}

fn first(list: &Vec<u32>) -> u32 {
    return list[0]; //不是获取所有权，而是通过clone 拷贝了一份新的
}
/*
fn first_student(list: &Vec<Student>) -> Student {
    return list[0]; //如果Student不实现copy，编译不过，所以通过下标访问都是clone 语义
}
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}

*/

fn largest(list: &Vec<u32>) -> &u32 {
    let mut largest = &list[0]; //? 这个地方是拷贝后的引用，还是原来数组中元素的引用呢？
    for item in list {
        if item > largest {
            largest = item
        }
    }
    return largest;
}

fn largestT<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn largest_tmut<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];

    for item in list.into_iter() {
        if item > &largest {
            largest = *item; //?这里取指针为啥需要copy trait
        }
    }
    return largest;
}

use std::fmt::{Debug, Display};

// 定义云主机开机操作助手trait: 泛型参数版
trait CloudOperator {
    fn create_instance(&self) {
        todo!()
    }
}

trait PublicCloudOperator: CloudOperator {
    fn create_public_instance(&self);
}

trait PrivateCloudOperator: CloudOperator {
    fn create_private_instance(&self);
}

// 定义阿里云操作助手
struct AliOperator {}

impl CloudOperator for AliOperator {}

impl PublicCloudOperator for AliOperator {
    fn create_public_instance(&self) {
        todo!()
    }
}

// 定义腾讯云操作助手
struct TencentOperator {}

impl CloudOperator for TencentOperator {}

impl PublicCloudOperator for TencentOperator {
    fn create_public_instance(&self) {
        todo!()
    }
}

fn create_public_instance<T>(_operator: &T)
where
    T: PublicCloudOperator,
{
}

fn create_private_instance<T: PrivateCloudOperator>(_operator: &T) {}

//trait 增强泛型的约束实现

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: PartialOrd + Display> Point<T> {
    fn compare_display(&self) {
        if self.x > self.y {
            println!("the largest number is x = {}", self.x)
        } else {
            println!("the largest number is y = {}", self.y)
        }
    }
}

//trait 关联类型
trait StreamingIterator {
    type Item: Debug + Display; //关联类型添加约束
}

struct A;

impl StreamingIterator for A {
    type Item = String;
}

struct B<T>
where
    T: StreamingIterator<Item = String>,
{
    a: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ali_operator = AliOperator {};
        ali_operator.create_instance();

        let tencent_operator = TencentOperator {};
        tencent_operator.create_instance();

        let a = A;
        let b = B { a };
    }
}

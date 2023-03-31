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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ali_operator = AliOperator {};
        ali_operator.create_instance();

        let tencent_operator = TencentOperator {};
        tencent_operator.create_instance();
    }
}

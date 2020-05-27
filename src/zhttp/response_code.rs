macro_rules! generate_response_code {
    (
        $(
            ($code:expr,$key:ident,$msg:expr);
         )+
    ) => {
        pub enum ResponseCode{
        $(
            $key,
        )+
        }
        impl ResponseCode{

            pub fn as_str(&self)-> &'static str{
                match *self{
                    $(
                        ResponseCode::$key=>$msg,
                    )+
                }
            }

            pub fn as_code(&self)->usize{
                match *self{
                    $(
                        ResponseCode::$key=>$code,
                    )+
                }
            }
        }
    };
}

generate_response_code! {
(20000,Code20000,"request success");
(10001,Code10001,"系统已经初始化，不需再次初始化");
(10002,Code10002,"用户未登录");
(10003,Code10003,"服务错误");
(10004,Code10004,"用户名或密码错误");
(10005,Code10005,"缺少必要参数");
(10006,Code10006,"空间ID错误");
(10007,Code10007,"文件不存在");
(10008,Code10008,"文件ID不能为空");
(10009,Code10009,"请求参数错误");
}

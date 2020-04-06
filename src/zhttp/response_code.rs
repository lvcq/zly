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
    (10004,Code10004,"用户名或密码错误");}
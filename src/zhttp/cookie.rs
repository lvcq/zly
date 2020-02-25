pub struct Cookie {
  name: String,
  value: String,
  domain: Option<String>,
  max_age: Option<u64>,
  http_only: bool,
  secure: bool,
  path:Option<String>
}
impl Cookie {
  pub fn new(name: &str, value: &str) -> Self {
    Cookie {
      name: String::from(name),
      value: String::from(value),
      domain: None,
      max_age: None,
      http_only: false,
      secure: false,
      path:None,
    }
  }
  pub fn set_max_age(mut self, age: u64) -> Self {
    self.max_age = Some(age);
    self
  }

  pub fn set_http_only(mut self, is_http_only: bool) -> Self {
    self.http_only = is_http_only;
    self
  }

  pub fn set_path(mut self,path:&str)->Self{
      self.path = Some(String::from(path));
      self
  }

  pub fn set_domain(mut self,domain:&str)-> Self{
        self.domain = Some(String::from(domain));
        self
  }

  pub fn set_secure(mut self,secure:bool)->Self{
        self.secure = secure;
        self
  }


  /// ### 将cookie信息转化为字符串
  /// > cookie字符串的格式：`key=value; Expires=date/Max-Age=second; Path=path; Domain=domain; Secure; HttpOnly`
  pub fn to_string(&self) -> String {
    let mut res_str = format!("{}={}", self.name, self.value);
    if self.max_age.is_some() {
      res_str = format!("{}; Max-Age={}", res_str, self.max_age.unwrap());
    }
    if self.path.is_some(){
        res_str = format!("{}; Path={}",res_str, self.path.as_ref().unwrap());
    }
    if self.domain.is_some(){
        res_str = format!("{}; Domain={}",res_str, self.domain.as_ref().unwrap());
    }
    if self.secure{
        res_str = format!("{}; Secure",res_str);
    }
    if self.http_only {
      res_str = format!("{}; HttpOnly", res_str);
    }
    res_str
  }
}

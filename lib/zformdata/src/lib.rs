extern crate buf_read_ext;
extern crate chrono;
extern crate futures;
extern crate httparse;
extern crate hyper;
extern crate mime;
extern crate tokio;

use chrono::Utc;
use futures::{future, StreamExt};
use futures::stream::AndThen;
use futures::TryStreamExt as _;
use hyper::header::HeaderValue;
use hyper::header::{HeaderMap, CONTENT_DISPOSITION, CONTENT_TYPE};
use hyper::{Body, Request};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use futures::executor::block_on;

#[derive(Debug)]
pub struct FormValue {
    fields: HashMap<String, String>,
    files: HashMap<String, MultipartFile>,
}

#[derive(Clone, Debug)]
pub struct MultipartFile {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
enum BodyReadStatus {
    ReadBoundary,
    ReadHeader,
    ReadMultipartFile(String),
    ReadText(String),
}

pub fn read_formdata(req: Request<Body>) -> FormValue {
    let mut fv = FormValue::new();
    let (parts, body) = req.into_parts();
    let content_type_value: &HeaderValue = match parts.headers.get(CONTENT_TYPE) {
        Some(ctv) => ctv,
        None => return fv
    };
    match get_content_type(content_type_value) {
        Ok(mi) => {
            if mi == mime::MULTIPART_FORM_DATA {
                let boundary = get_boundary(content_type_value);
                fv = read_multipart_body(body, fv, boundary);
            } else if mi == mime::APPLICATION_JSON {
                let body_str = read_json_body(body);
                println!("content-type:::{:?}", &body_str);
            }
        }

        Err(_e) => {}
    };
    fv
}

fn read_multipart_body(body: Body, mut fv: FormValue, boundary: Vec<u8>) -> FormValue {
    let mut is_first_chunk = true;
    let mut residue: Vec<u8> = Vec::new();
    let mut lt: Vec<u8> = Vec::new();
    let mut ltlt: Vec<u8> = Vec::new();
    let mut lt_boundary: Vec<u8> = Vec::new();
    let mut read_status: BodyReadStatus = BodyReadStatus::ReadBoundary;
    let fut = body.try_for_each(|chunk| {
        println!("test2");
        let mut span: usize = 0;
        let mut chunk_vec: Vec<u8> = Vec::new();
        chunk_vec.extend(&residue);
        chunk_vec.extend(chunk.to_vec());
        residue.clear();
        let mut buf: Vec<u8>;
        if is_first_chunk {
            is_first_chunk = false;
            match first_boundary_match(&chunk_vec, &boundary) {
                Ok(res) => {
                    lt = res.0.clone();
                    ltlt = res.1;
                    lt_boundary = res.0.clone();
                    lt_boundary.extend(&boundary);
                    span = res.2 + lt.len() + 1;
                }
                Err(_) => {}
            }
            read_status = BodyReadStatus::ReadHeader;
        }
        loop {
            match &read_status {
                BodyReadStatus::ReadBoundary => match vec_match(&chunk_vec, &boundary, span) {
                    Ok(b_index) => {
                        if chunk_vec.get(b_index + 1..b_index + 3) == Some(&[b'-', b'-'][..]) {
                            break;
                        } else {
                            span = b_index + lt.len() + 1;
                            read_status = BodyReadStatus::ReadHeader;
                        }
                    }
                    Err(_) => {
                        break;
                    }
                },
                BodyReadStatus::ReadHeader => {
                    let header_end = match vec_match(&chunk_vec, &ltlt, span) {
                        Ok(end) => end,
                        Err(_) => {
                            break;
                        }
                    };
                    buf = chunk_vec[span..header_end + 1].to_vec();
                    span = header_end + 1;
                    let part_header = {
                        let mut header_memory = [httparse::EMPTY_HEADER; 4];
                        match httparse::parse_headers(&buf, &mut header_memory) {
                            Ok(httparse::Status::Complete((_, raw_headers))) => {
                                let mut map = HeaderMap::new();
                                for header in raw_headers {
                                    let value = HeaderValue::from_bytes(header.value).unwrap();
                                    match header.name {
                                        "Content-Disposition" => {
                                            map.insert(CONTENT_DISPOSITION, value);
                                        }
                                        "Content-Type" => {
                                            map.insert(CONTENT_TYPE, value);
                                        }
                                        _ => {}
                                    }
                                }
                                map
                            }
                            Ok(httparse::Status::Partial) => HeaderMap::new(),
                            Err(_err) => HeaderMap::new(),
                        }
                    };
                    if part_header.is_empty() {
                        break;
                    }

                    let cd: Option<&HeaderValue> = part_header.get(CONTENT_DISPOSITION);
                    if cd.is_none() {
                        break;
                    }
                    let cd_vec: Vec<u8> = cd.unwrap().as_bytes().to_vec();
                    let mut cdh: HashMap<&str, String> = HashMap::new();
                    let mut s_index: usize = 0;
                    match vec_match(&cd_vec, &vec![b';'], s_index) {
                        Ok(sz) => {
                            cdh.insert(
                                "cd",
                                String::from_utf8_lossy(&cd_vec[s_index..sz]).to_string(),
                            );
                            s_index = sz;
                        }
                        Err(_) => {
                            break;
                        }
                    };
                    match vec_match(&cd_vec, &vec![b'n', b'a', b'm', b'e', b'=', b'"'], s_index) {
                        Ok(sz) => {
                            let e_index = match vec_match(&cd_vec, &vec![b'"'], sz + 1) {
                                Ok(e_in) => e_in,
                                Err(_) => cd_vec.len(),
                            };
                            cdh.insert(
                                "name",
                                String::from_utf8_lossy(&cd_vec[sz + 1..e_index]).to_string(),
                            );
                            s_index = e_index + 2;
                        }
                        Err(_) => {
                            break;
                        }
                    };
                    match vec_match(
                        &cd_vec,
                        &vec![b'f', b'i', b'l', b'e', b'n', b'a', b'm', b'e', b'=', b'"'],
                        s_index,
                    ) {
                        Ok(sz) => {
                            let e_index = match vec_match(&cd_vec, &vec![b'"'], sz + 1) {
                                Ok(e_in) => e_in,
                                Err(_) => cd_vec.len(),
                            };
                            cdh.insert(
                                "filename",
                                String::from_utf8_lossy(&cd_vec[sz + 1..e_index]).to_string(),
                            );
                        }
                        Err(_) => {}
                    };
                    if cdh.contains_key("filename") {
                        let temp_dir = "/home/hk/project/zly/temp/";
                        let now = Utc::now().timestamp_millis();
                        let temp_file =
                            format!("{}{}_{}", temp_dir, now, cdh.get("filename").unwrap());
                        File::create(&temp_file).expect("create file fail.");
                        fv.files.insert(
                            cdh.get("name").unwrap().clone(),
                            MultipartFile {
                                name: cdh.get("filename").unwrap().clone(),
                                path: temp_file,
                            },
                        );
                        read_status =
                            BodyReadStatus::ReadMultipartFile(cdh.get("name").unwrap().clone())
                    } else {
                        read_status = BodyReadStatus::ReadText(cdh.get("name").unwrap().clone());
                    }
                }
                BodyReadStatus::ReadText(key) => match vec_match(&chunk_vec, &lt_boundary, span) {
                    Ok(b_index) => {
                        let end = b_index - lt_boundary.len();
                        fv.fields.insert(
                            key.to_string(),
                            String::from_utf8_lossy(&chunk_vec[span..end + 1]).to_string(),
                        );
                        span = b_index - boundary.len();
                        read_status = BodyReadStatus::ReadBoundary;
                    }
                    Err(_) => {
                        residue = chunk_vec[span..chunk_vec.len()].to_vec();
                        break;
                    }
                },
                BodyReadStatus::ReadMultipartFile(key) => {
                    let file = fv.files.get(key).unwrap();
                    let read_complete: bool;
                    let write_index: usize;
                    let mut spot = span;
                    loop {
                        if spot >= chunk_vec.len() {
                            write_index = chunk_vec.len() - 1;
                            read_complete = false;
                            break;
                        }
                        match vec_match(&chunk_vec, &lt, spot) {
                            Ok(lt_index) => {
                                if vec_slice_eq(
                                    &chunk_vec,
                                    &boundary,
                                    lt_index + 1,
                                    lt_index + boundary.len(),
                                ) {
                                    write_index = lt_index - lt.len();
                                    read_complete = true;
                                    break;
                                } else {
                                    spot = lt_index + 1;
                                }
                            }
                            Err(_) => {
                                write_index = chunk_vec.len() - 1;
                                read_complete = false;
                                break;
                            }
                        }
                    }
                    println!("span:{},write_index:{}", span, write_index);
                    let mut file_handler = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&file.path)
                        .expect("open fail.");
                    file_handler
                        .write_all(&chunk_vec[span..write_index + 1])
                        .expect("write fail.");
                    span = write_index;
                    if read_complete {
                        read_status = BodyReadStatus::ReadBoundary;
                    } else {
                        break;
                    }
                }
            }
        }
        future::ready(Ok(()))
    });
    match block_on(fut){
        Ok(_)=>fv,
        Err(_)=>fv
    }
}

fn read_json_body(body: Body) -> String {
    let mut res: Vec<u8> = Vec::new();
    body.and_then(|chunk| {
        res.extend(chunk.to_vec());
        future::ready(Ok(()))
    });
    String::from_utf8_lossy(&res[..]).to_string()
}

/// ####第一次匹配
/// 返回换行符和边界结束位置
///
fn first_boundary_match(
    origin: &Vec<u8>,
    boundary: &Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>, usize), ()> {
    match vec_match::<u8>(origin, boundary, 0) {
        Ok(end) => {
            if origin.get(end + 1) == Some(&('\r' as u8))
                && origin.get(end + 2) == Some(&('\n' as u8))
            {
                return Ok((vec![b'\r', b'\n'], vec![b'\r', b'\n', b'\r', b'\n'], end));
            } else if origin.get(end + 2) == Some(&('\n' as u8)) {
                return Ok((vec![b'\n'], vec![b'\n', b'\n'], end));
            }
            return Err(());
        }
        Err(_) => Err(()),
    }
}

fn vec_match<T: PartialEq>(origin: &Vec<T>, target: &Vec<T>, start: usize) -> Result<usize, ()> {
    let o_len = origin.len();
    let t_len = target.len();
    let mut o_index: usize = start;
    loop {
        if o_index > o_len - t_len {
            return Err(());
        }
        let mut t_index = 0;
        let mut p_index = o_index;

        while t_index < t_len && p_index < o_len && origin[p_index] == target[t_index] {
            p_index = p_index + 1;
            t_index = t_index + 1;
        }
        if t_index == t_len {
            return Ok(o_index + t_len - 1);
        }

        o_index = o_index + 1;
    }
}

/// #### 序列中一部分是否与目标相等
///
fn vec_slice_eq<T: PartialEq>(origin: &Vec<T>, target: &Vec<T>, start: usize, end: usize) -> bool {
    let t_len = target.len();
    let mut o_index: usize = 0;
    if t_len != end - start + 1 {
        return false;
    }
    while o_index < t_len {
        if origin[start + o_index] != target[o_index] {
            return false;
        }
        o_index = o_index + 1;
    }
    return true;
}

fn get_content_type(content_value: &HeaderValue) -> Result<mime::Mime, mime::FromStrError> {
    let bytes = content_value.as_bytes();
    let len = bytes.len();
    let mut index: usize = 0;
    while index < len && bytes[index] != ';' as u8 {
        index = index + 1;
    }
    String::from_utf8_lossy(&bytes[0..index])
        .trim()
        .parse::<mime::Mime>()
}

fn get_boundary(content_value: &HeaderValue) -> Vec<u8> {
    let bytes = content_value.as_bytes();
    let len = bytes.len();
    let target = "boundary=".as_bytes();
    let target_len = target.len();
    let mut index: usize = 0;
    while index < len && bytes[index] != ';' as u8 {
        index = index + 1;
    }
    index = index + 1;
    while index < len && bytes[index] != target[0] {
        index = index + 1;
    }
    index = index + target_len;
    let mut boundary: Vec<u8> = bytes[index..].to_vec();
    boundary.insert(0, '-' as u8);
    boundary.insert(0, '-' as u8);
    return boundary;
}

impl FormValue {
    pub fn new() -> FormValue {
        FormValue {
            fields: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn get_field(&self, key_name: &String) -> Option<String> {
        if self.fields.contains_key(key_name) {
            Some(self.fields.get(key_name).unwrap().clone())
        } else {
            None
        }
    }

    pub fn get_file(&self, key_name: &String) -> Option<MultipartFile> {
        if self.files.contains_key(key_name) {
            Some(self.files.get(key_name).unwrap().clone())
        } else {
            None
        }
    }
}

impl Drop for MultipartFile {
    fn drop(&mut self) {
        fs::remove_file(&self.path).expect("remove fail.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn vec_match_test() {
        let origin: Vec<usize> = vec![1, 2, 3, 4, 5];
        let target: Vec<usize> = vec![3, 4];
        assert_eq!(vec_match(&origin, &target, 0), Ok(3))
    }
}

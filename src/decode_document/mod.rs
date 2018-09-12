extern crate bytes;
use self::bytes::Bytes;

pub fn decode(data: Bytes) -> Result<(String, String, u8), String> {
    let split_word = Bytes::from(&b"kugiri"[..]);
    let mut iter = data.windows(split_word.len());
    let mut split_index_arr = vec![];
    for (i, _) in data.iter().enumerate() {
        match iter.next() {
            Some(iter_next) => if iter_next == split_word {
                split_index_arr.push(i);
                if split_index_arr.len() == 3{
                    break;
                }
            },
            None => return Err("not found kugiri word".to_string())
        }
    }
    let (url, other_than_url) = data.split_at(split_index_arr[0]);
    let (_, other_than_url) = other_than_url.split_at(split_word.len()); // remove "kugiri"
    let (indexing_doc, code_point) = other_than_url.split_at(split_index_arr[1] - url.len() - split_word.len());
    let (_, code_point) = code_point.split_at(split_word.len());
    let (code_point, is_get_all_text) = code_point.split_at(split_index_arr[2] - url.len() - split_word.len() - indexing_doc.len() - split_word.len());
    let (_, is_get_all_text) = is_get_all_text.split_at(split_word.len());
    let is_get_all_text = String::from_utf8_lossy(&is_get_all_text);

    let is_get_all_text: u8 = is_get_all_text.parse().unwrap_or(0);
    // let is_get_all_text: u8 = is_get_all_text.parse().map_err(|e| {
    //     Err("not found kugiri word".to_string())
    // })?;

    let url = String::from_utf8_lossy(&url);
    let code_point = String::from_utf8_lossy(&code_point);
    let code_point_chars = code_point.chars().collect::<Vec<char>>();
    let mut decoded_doc = String::from("");

    for c in indexing_doc {
        let index = *c as usize;
        if index == 255 {
            // push null char
        } else {
            decoded_doc.push(code_point_chars[index])
        }
    }

    Ok((url.to_string(), decoded_doc, is_get_all_text))
}
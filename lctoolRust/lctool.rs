use std::str::FromStr;
use std::fmt::Debug;

// 读取二维数组序列，生成相应类型的数组,感觉写的算是很简洁了
pub fn generate_matrix<T:FromStr>(sequence:&str) -> Vec<Vec<T>> 
    where <T as FromStr>::Err: Debug{
    let sequence = &sequence.trim()[2..sequence.len()-2];
    let sequence_list:Vec<String> = sequence.split("],[").map(|s| s.to_string()).collect();
    return sequence_list.iter()
        .map(|x| x.split(",").map(|s| T::from_str(s).unwrap()).collect::<Vec<T>>())
        .collect();
}
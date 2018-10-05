pub fn run(text_vec:&Vec<char>, query_vec:&Vec<char>, result:&mut Vec<usize>) {
    for idx1 in 0..(text_vec.len() - query_vec.len()) {
        let mut matched = true;
        for idx2 in 0..(query_vec.len()) { // ignore \n
            if text_vec[idx1 + idx2] != query_vec[idx2] {
                matched = false;
                break;
            }
        }
        if matched {
            result.push(idx1);
        }
    }
}
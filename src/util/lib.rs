pub fn convert_vec(strs: Vec<&str>) -> Vec<isize> {
    let mut output = Vec::new();
    for s in strs {
        let n = s.parse().expect("Failed to parse int");
        output.push(n);
    }

    output
}
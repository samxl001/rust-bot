struct AnalysisOp {
    query: Vec<String>,
    result: Vec<String>,
}
impl AnalysisOp {
    fn new() -> AnalysisOp {
        AnalysisOp {
            query: Vec::new(),
            result: Vec::new(),
        }
    }
}
pub fn search_titles(query: Vec<String>, info: &Vec<String>) -> Vec<String> {
    let mut analysis = AnalysisOp::new();
    analysis.query = query;
    for word in analysis.query.iter() {
        for sentence in info.iter() {
            let word = word.to_lowercase();
            if sentence.to_lowercase().contains(&word) {
                println!("{}", sentence);
                analysis.result.push(sentence.clone());
            }
        }
    }
    analysis.result
}

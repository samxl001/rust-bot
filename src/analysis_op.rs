use std::process::{Command, exit};
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
pub fn run_python_script() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the path to the virtual environment's Python executable
    let python_executable = "python_plot/.venv/bin/python"; // Change this based on your system (Windows: "./venv/Scripts/python.exe")

    // Path to your Python script
    let python_script = "python_plot/main.py";

    // Run the Python script using the virtual environment's Python executable
    let output = Command::new(python_executable)
        .arg(python_script)  // Add any additional arguments for the script here
        .output()  // Capture output (stdout and stderr)
        .expect("Failed to execute Python script");

    // Handle output from the Python script
    if !output.status.success() {
        eprintln!("Python script failed with status: {}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    println!("Python script executed successfully: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

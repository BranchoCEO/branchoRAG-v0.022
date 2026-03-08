use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

// 1. Define the Graph Structure (Nodes and Edges)
#[pyclass]
#[derive(Serialize, Deserialize)]
struct CodeGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

#[pymethods]
impl CodeGraph {
    #[new]
    fn new() -> Self {
        CodeGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    // 2. Add a file/function to the graph
    fn add_node(&mut self, name: String) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(name);
        idx
    }

    // 3. Connect two files together
    fn add_edge(&mut self, from_idx: usize, to_idx: usize) {
        self.edges.push((from_idx, to_idx));
    }

    // 4. THE SAVE FUNCTION: This makes it survive a PC shutdown
    fn save_to_disk(&self, filepath: &str) -> PyResult<()> {
        let json = serde_json::to_string(&self).unwrap();
        let mut file = File::create(filepath)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

// 5. Package it all up for Python
#[pymodule]
fn branchograph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CodeGraph>()?;
    Ok(())
}

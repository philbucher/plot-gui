#![warn(clippy::all, rust_2018_idioms)]

use std::collections::HashMap;

{
    strings: ["DEBUT", "glob.1", "11.export"]
    floats: [123.5 (VM_Curr), 999.8 (VM_Max), ]
}

// at the end of each TS, whether or not it is converged

{
    name: "DEBUT"
    path: "proc.0" // or "global"
    memory_current: 123.5 // VM_Curr => does not exist for "global" folder
    memory_peak: 123.5 // VM_Curr => does not exist for "global" folder
    files: {
        "glob.1" : 1236 // B
        "fort.20" : 547 // B
        "11.export" : 89 // B
        "REPE_OUT/proj.med" : 159 // B
    }
    date_time: ...
    counter: ... // not  consecutive, needs to be updated during reading
}

// this is parsed from the logs
struct DataPoint
{
    memory_current: Option<f64>,
    memory_peak: Option<f64>,
    files: HashMap<String, f64>,
    counter: usize // reordered consecutive (must happen during reading!
    date_time: ...
    rank: Option<u32>
}

#[derive(PartialEq, Clone, Copy)]
enum DataType {
    Memory,
    DiskSpace,
}


// probably best to construct the plot-objects here, due to the interactive rendering
struct DataReader {
    source_path: PathBuf,

}


impl Data {
    pub fn new() {}

    pub fn available_data() -> Vec<DataSeries> {}
}


struct DataSeries
{
    data_type: DataType,
    name: String, // memory
}

impl DataSeries {

    pub fn name() {
        return format!({self.data_type}_{self.rank}_{self.identifier()})
    }

    pub fn identifier() {
        return file_name if self.data_type == DataType.DiskSpace else "";
    }


    pub fn get_data() -> Vec<DataPoint> {}
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_deserialize_data_point() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}

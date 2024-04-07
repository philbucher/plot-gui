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
    memory: 123.5 // VM_Curr => does not exist for "global" folder
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
    memory: Option<f64>,
    files: HashMap<String, f64>,
    counter: usize // reordered consecutive (must happen during reading!
    date_time: ...
    rank: Option<i32>
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

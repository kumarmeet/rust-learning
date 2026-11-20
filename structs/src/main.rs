//Rust has three kinds of structs: named field, tuple-like, and unit-like.

// name field
#[derive(Debug)] // compiler directive it's a metadata on the line above to tell compiler how to parse the code
struct Llm {
    name: String,
    temperature: f64,
    parameter_length: u64,
    price_per_token: f64,
}

// defining struct methods
#[derive(Debug)]
struct FileMetadata {
    file_name: String,
    file_size: u64,
    file_extension: String,
    file_type: String,
    // file_permissions:
}

impl FileMetadata {
    // associated function
    fn new(file_name: String, file_size: u64, file_extension: String, file_type: String) -> Self {
        return Self {
            file_name,
            file_size,
            file_extension,
            file_type,
        };
    }
}

impl FileMetadata {
    // fn(self, arg){}
    // fn(&self, arg){}
    // fn(mut self, arg){}
    // fn(&mut self, arg){}

    //So 'Self' is another way that we can refer to the type 'FileMetadata' in here.
    //The advantage of this type annotation, even though it's the same thing, is that this code will stay
    // the same if the struct name changes. It's more resilient to change.

    // fn display_file_info(self: Self) {
    // fn display_file_info(self: FileMetadata) {
    // fn display_file_info(self) {
    fn display_file_info(&self) {
        println!("{:#?}", self);
    }

    fn file_name(&self) -> &str {
        &self.file_name
    }

    fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }

    fn convert_file_type_pdf(mut self) {
        self.file_type = "pdf".to_string();
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
struct Computer {
    new_cpu: String,
    new_memory: u32,
    new_hard_drive_capacity: u32,
}

impl Computer {
    fn new(new_cpu: String, new_memory: u32, new_hard_drive_capacity: u32) -> Self {
        Self {
            new_cpu,
            new_memory,
            new_hard_drive_capacity,
        }
    }

    fn update_cpu(&mut self, cpu: String) -> &mut Self {
        self.new_cpu = cpu;
        self
    }

    fn update_memory(&mut self, memory: u32) -> &mut Self {
        self.new_memory = memory;
        self
    }
    fn update_hard_drive_capacity(&mut self, hard_drive_capacity: u32) -> &mut Self {
        self.new_hard_drive_capacity = hard_drive_capacity;
        self
    }
}

fn main() {
    // there is no partial mutable struct, all will mutable or none
    let mut chat_gpt = Llm {
        name: String::from("chat gpt"),
        temperature: 0.7,
        parameter_length: 1_000_000_000,
        price_per_token: 0.006,
    };

    println!(
        "1. The {} has set temperature is {} also parameter length is {} with price per token {}",
        chat_gpt.name, chat_gpt.temperature, chat_gpt.parameter_length, chat_gpt.price_per_token
    );

    chat_gpt.name = String::from("opus 5.5");
    chat_gpt.temperature = 0.8;
    chat_gpt.parameter_length = 1_000_000_000_000_000;
    chat_gpt.price_per_token = 0.009;

    println!(
        "2. The {} has set temperature is {} also parameter length is {} with price per token {}",
        chat_gpt.name, chat_gpt.temperature, chat_gpt.parameter_length, chat_gpt.price_per_token
    );

    let name = chat_gpt.name;

    let usage = llm_usage(
        name,
        chat_gpt.temperature,
        chat_gpt.parameter_length,
        chat_gpt.price_per_token,
    );

    println!(
        "3. The {} has set temperature is {} also parameter length is {} with price per token {}",
        usage.name, usage.temperature, usage.parameter_length, usage.price_per_token
    );

    let update_gpt = Llm {
        name: usage.name.clone(),
        ..chat_gpt
    };

    println!(
        "4. The {} has set temperature is {} also parameter length is {} with price per token {}",
        update_gpt.name,
        update_gpt.temperature,
        update_gpt.parameter_length,
        update_gpt.price_per_token
    );

    let gemini = Llm {
        name: String::from("google gimini 5"),
        temperature: 0.9,
        parameter_length: 1_000_000_000_000_000,
        price_per_token: 0.08,
    };

    let gemini = struct_pass(gemini);
    println!(
        "derive or attribute debug trait for struct -> {:#?}",
        gemini
    );

    let file = FileMetadata::new(
        String::from("file"),
        1000,
        String::from("pdf"),
        String::from("pdf"),
    );

    file.display_file_info();

    let file = FileMetadata::new(
        String::from("file"),
        1000,
        String::from("json"),
        String::from("json"),
    );

    file.convert_file_type_pdf();

    let mut computer = Computer::new(String::from("M4"), 16, 512);

    println!("{:#?}", computer);

    //builder pattern
    computer
        .update_cpu(String::from("M5"))
        .update_memory(32)
        .update_hard_drive_capacity(1024);

    println!("{:#?}", computer);
}

fn llm_usage(name: String, temperature: f64, parameter_length: u64, price_per_token: f64) -> Llm {
    return Llm {
        name,
        temperature,
        parameter_length,
        price_per_token,
    };
}

// fn struct_pass(llm: &mut Llm) -> Llm /* ownership not moved along with read and write */ {
// fn struct_pass(llm: &Llm) -> Llm /* ownership not moved only read not write */ {
// fn struct_pass(mut llm: Llm) -> Llm /*  ownership moved along with read and write */ {
fn struct_pass(llm: Llm) -> Llm /* ownership moved only read not write */ {
    return Llm {
        name: llm.name,
        temperature: llm.temperature,
        parameter_length: llm.parameter_length,
        price_per_token: llm.price_per_token,
    };
}

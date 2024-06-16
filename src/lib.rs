/*
Hugging Face Rust Library to analyse lyrics to songs and puts them into a sqlite db
*/

// Import crates from rust-bert for sequence classification
use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;

// standard library components 
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use sqlite::Connection;

// Function to create an in-memory SQLite database and populate it with candidate labels
fn create_db() -> sqlite::Connection {
    let db = sqlite::open(":memory:").unwrap(); 
    db.execute("CREATE TABLE zeroshotcandidates (id INTEGER PRIMARY KEY, label TEXT)").unwrap(); 
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('rock')").unwrap(); 
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('pop')").unwrap(); 
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('hip hop')").unwrap(); 
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('country')").unwrap(); 
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('latin')").unwrap(); 
    
    // Return the database connection
    db 
}

// get all zero-shot classification candidates from the database as a vector of strings
pub fn get_all_zeroshotcandidates() -> Vec<String> {
    let db = create_db(); // Create the database and get the connection
    let query = "SELECT label FROM zeroshotcandidates"; // SQL query to select all labels
    let mut candidates: Vec<String> = Vec::new(); // Vector to hold the labels
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.iter() {
            let value = value.unwrap(); // Unwrap the SQLite value
            candidates.push(value.to_string()); // Push the label to the vector
        }
        true
    }).unwrap(); // Execute the query and iterate over the results
    candidates // Return the vector of candidates
}

// Function to read lyrics from a file and return them as a vector of strings
pub fn read_lyrics(file: &str) -> Vec<String> {
    let mut lyrics: Vec<String> = Vec::new(); // Vector to hold the lyrics
    let file = File::open(file).expect("Unable to open the file"); // Open the file
    let reader = BufReader::new(file); // Create a buffered reader
    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap each line
        lyrics.push(line); // Push the line to the vector
    }
    lyrics // Return the vector of lyrics
}

/*
    Function to classify lyrics using the zero-shot classification model.
    Accepts a vector of strings (lyrics) and retrieves candidate labels from the in-memory SQLite database.
*/
pub fn classify_lyrics(lyrics: Vec<String>) -> Vec<Vec<Label>> {
    // Extract candidate labels from SQLite db and put them in a vector
    let temp_candidates = get_all_zeroshotcandidates();
    let candidate_labels: Vec<&str> = temp_candidates.iter().map(|s| s.as_str()).collect();

    // Join lyrics into a single string
    let lyrics = lyrics.join(" ");

    // Convert to type std::convert::AsRef<str>
    let lyrics: &str = lyrics.as_ref();

    // Create a zero-shot classification model
    let zero_shot_model = ZeroShotClassificationModel::new(Default::default()).unwrap();

    // Classify the lyrics
    zero_shot_model.predict_multilabel([lyrics], candidate_labels, None, 128)
}

// Add comments wherever necessary to explain the code

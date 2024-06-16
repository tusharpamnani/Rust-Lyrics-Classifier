/*
    A CLI to analyze lyrics to songs and put them into a sqlite db
*/

use clap::Parser;
use sqlite::Result;

// Define the CLI structure using the Clap crate
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Tushar Pamnani",
    about = "A CLI to analyze lyrics to songs and put them into a sqlite db"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>, // Define possible subcommands
}

// Define the subcommands for the CLI
#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Tushar Pamnani")]

    // Subcommand for classifying lyrics
    Classify {
        // File option with a default value of "lyrics.txt"
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },

    // Subcommand for listing candidates
    Candidates {},

    // Subcommand for reading lyrics
    Lyrics {
        // File option with a default value of "lyrics.txt"
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },
}

fn main() {
    let args = Cli::parse(); 
    match args.command {

        // Handle the classify subcommand
        Some(Commands::Classify { file }) => {
            println!("Classifying lyrics from file: {}", file);
            let lyrics = sqlitehf::read_lyrics(&file); // Read lyrics from the file
            let result = sqlitehf::classify_lyrics(lyrics); // Classify the lyrics
                                                            // Print out the classification result in a human-readable format
            for label in result {
                for l in label {
                    println!("{} : {}", l.text, l.score);
                }
            }
        }

        // Handle the candidates subcommand
        Some(Commands::Candidates {}) => {
            for candidate in sqlitehf::get_all_zeroshotcandidates() {
                println!("{}", candidate); // Print each candidate
            }
        }

        // Handle the lyrics subcommand
        Some(Commands::Lyrics { file }) => {
            println!("Reading lyrics from file: {}", file);
            for line in sqlitehf::read_lyrics(&file) {
                println!("{}", line); // Print each line of lyrics
            }
        }
        
        // Handle the case where no subcommand is provided
        None => {
            println!("No command provided");
        }
    }
}

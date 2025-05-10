use nwav::WAV;

fn main() {
    let args = std::env::args();

    if args.len() <= 1 {
        eprintln!("Please provide the name of a file.");
        std::process::exit(1);
    }

    let filename = args.last().unwrap();

    let data = std::fs::read(filename).unwrap();

    let wav = WAV::from_data(data.as_slice());
    let chunks = wav.available_chunks();

    for i in &chunks {
        if i.name == "data" {
            continue;
        }

        let ch = wav.read_chunk(i);
        println!("{} {ch:x?}", i.name);
    }
}
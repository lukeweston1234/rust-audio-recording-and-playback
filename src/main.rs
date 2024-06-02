mod audio;

use audio::AudioClip;
use color_eyre::eyre::Result;

fn main() -> Result<()>{
    color_eyre::install()?; // Error handling and logging

    let clip = AudioClip::record()?;

    let clip_two = AudioClip::record()?;

    let clips = vec![clip, clip_two];

    // Create a thread for each clip to play them concurrently
    let mut handles = vec![];
    for clip in clips {
        let handle = std::thread::spawn(move || {
            clip.play().unwrap();
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

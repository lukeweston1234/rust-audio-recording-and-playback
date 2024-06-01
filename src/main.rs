mod audio_clip;

use audio_clip::AudioClip;
use color_eyre::eyre::Result;

fn main() -> Result<()>{
    color_eyre::install(); // Error handling and logging

    let clip = AudioClip::record()?;

    clip.play();

    Ok(())
}

# Macropad Sequencer

A minimal USB MIDI step sequencer that runs on the [Adafruit Macropad](https://www.adafruit.com/product/5128) written in Rust using Embassy.

This is more of a tech demo than anything else!
I'm using it to mess around with ideas I have around structuring non-trivial Embassy apps and will probably come back and add more features whenever I need a break from life.

## Usage

- Each key is a step in the sequence
- Press and release a key to toggle the step on and off
- Hold a single key down to change the note and velocity
- Hold down multiple keys to only play

## Flashing

While holding down on the rotary encoder, press and release the reset button on the side.
The macropad should show up as a USB mass storage device.
You can then drag the `uf2` file the releases page and reboot it.

For development, install [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs) and do `cargo run --release`.

## Future development

I had to stop hacking on this before it completely consumed my life, but here's a few things I'd like to add:

- Swing
- "Gate" (percentage of step note is on)
- Menu scrolling to support more than 4 options per menu
- A little synth engine that uses the onboard speaker (need to make use of that second core!)

use crate::{Irqs, menus::Note};
use embassy_futures::join::join;
use embassy_rp::{Peri, peripherals::USB, usb::Driver};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_usb::class::{
    cdc_acm::{CdcAcmClass, State},
    midi::MidiClass,
};
use midi_convert::{
    midi_types::{MidiMessage, Note as MidiNote},
    render_slice::MidiRenderSlice,
};
use usbd_midi::{CableNumber, UsbMidiEventPacket};

pub enum MidiEvent {
    Note {
        on: bool,
        note: Note,
        octave: u8,
        velocity: u8,
    },
}

pub static MIDI_CHANNEL: Channel<ThreadModeRawMutex, MidiEvent, 2> = Channel::new();

#[embassy_executor::task]
pub async fn usb_midi(peripheral: Peri<'static, USB>) {
    let driver = Driver::new(peripheral, Irqs);
    let mut usb_config = embassy_usb::Config::new(0xc0de, 0xcafe);
    usb_config.manufacturer = Some("Heuristic Industries");
    usb_config.product = Some("Macropad");
    usb_config.serial_number = Some("123456789");
    usb_config.max_power = 100;
    usb_config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut logger_state = State::new();

    let mut usb_builder = embassy_usb::Builder::new(
        driver,
        usb_config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    let mut midi_class = MidiClass::new(&mut usb_builder, 1, 1, 64);
    let logger_class = CdcAcmClass::new(&mut usb_builder, &mut logger_state, 64);
    let logger_fut = embassy_usb_logger::with_class!(1024, log::LevelFilter::Info, logger_class);

    let mut usb = usb_builder.build();
    let usb_fut = usb.run();

    let midi_fut = async {
        loop {
            match MIDI_CHANNEL.receive().await {
                MidiEvent::Note {
                    on,
                    note,
                    octave,
                    velocity,
                } => {
                    let midi_note = convert_note(note, octave);
                    let message = if on {
                        MidiMessage::NoteOn(1.into(), midi_note, velocity.into())
                    } else {
                        MidiMessage::NoteOff(1.into(), midi_note, velocity.into())
                    };

                    let mut buffer = [0u8; 3];
                    message.render_slice(&mut buffer);
                    let packet =
                        UsbMidiEventPacket::try_from_payload_bytes(CableNumber::Cable0, &buffer)
                            .unwrap();
                    midi_class
                        .write_packet(packet.as_raw_bytes())
                        .await
                        .unwrap();
                }
            }
        }
    };

    join(usb_fut, join(logger_fut, midi_fut)).await;
}

fn convert_note(note: Note, octave: u8) -> MidiNote {
    let root = 21 + octave * 12;
    let offset = match note {
        Note::A => 0,
        Note::BFlat => 1,
        Note::B => 2,
        Note::C => 3,
        Note::CSharp => 4,
        Note::D => 5,
        Note::EFlat => 6,
        Note::E => 7,
        Note::F => 8,
        Note::FSharp => 9,
        Note::G => 10,
        Note::AFlat => 11,
    };

    (root + offset).into()
}

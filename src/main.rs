
extern crate nom_midi as midi;
use nom_midi::note::Note::*;
use midi::note::Note;

#[derive(Debug, PartialEq)]
pub struct SanitizedNote {
    pub global_time : u32,
    pub length: u32,
    pub note: Note,
}

#[derive(Debug)]
struct SanitizedNoteTemp {
    global_time : u32,
    note: Note,
    start: bool,
}


#[derive(Debug, PartialEq)]
pub struct SanitizedNote2 {
    pub global_time : u32,
    pub length: u32,
    pub note: u8,
}

#[derive(Debug)]
struct SanitizedNoteTemp2 {
    global_time : u32,
    note: u8,
    start: bool,
}

#[derive(Debug, Default)]
struct HogeHandler {
    time:u32,
    notes: Vec<SanitizedNoteTemp2>
}


    use ghakuf::messages::*;
    use ghakuf::reader::*;
    use std::path;

impl Handler for HogeHandler {
    fn header(&mut self, _format: u16, _track: u16, _time_base: u16) {
      // Something
    }
    fn meta_event(&mut self, delta_time: u32, _event: &MetaEvent, _data: &Vec<u8>) {
        self.time += delta_time;
      // you
    }
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        self.time += delta_time;
        match event {
            ghakuf::messages::MidiEvent::NoteOn{note, ch: _, velocity: _}  => {
                self.notes.push(SanitizedNoteTemp2{
                    global_time:self.time,
                    note:*note,
                    start: true
                });
            },
            ghakuf::messages::MidiEvent::NoteOff{note, ch: _, velocity: _} => {
                self.notes.push(SanitizedNoteTemp2{
                    global_time:self.time,
                    note:*note,
                    start: false
                });
            },
            _ => {},
        }
      // want
    }
    fn sys_ex_event(&mut self, delta_time: u32, _event: &SysExEvent, _data: &Vec<u8>) {
        self.time += delta_time;
      // to
    }
    fn track_change(&mut self) {
      // do
    }
}

fn extract_notes_2(path: &str) -> Vec<SanitizedNote2> {

    let path = path::Path::new(path);
    let mut handler = HogeHandler::default();
    let mut reader = Reader::new(
        &mut handler,
        &path,
    ).unwrap();
    let _ = reader.read();

    let mut f = handler.notes;

    let mut notes = vec![];
    f.reverse();
    while let Some(next_note) = f.pop() {
        f.reverse();
        //find end
        let pos = f.iter().position(|el| el.note==next_note.note).unwrap();
        let end_time = f[pos].global_time;
        f.remove(pos);

        notes.push(SanitizedNote2{
            note: next_note.note,
            length: end_time - next_note.global_time,
            global_time: next_note.global_time,
        });

        f.reverse();
    }
    notes
        
   
}

fn extract_notes(bytes: &[u8]) -> Vec<SanitizedNote> {
    println!("{:?}", midi::parser::parse_midi(bytes));
    let midi = midi::parser::parse_midi(bytes).unwrap().1;

    //get notes for all tracks
    let mut notes = vec![];
    for track in midi.tracks {
        let mut time = 0;
        let mut f:Vec<_> = track.events.iter().flat_map(|el|{
            time += el.delta_time;
            match el.event {
                nom_midi::EventType::Midi(ev) => {
                    match ev.event {
                        nom_midi::MidiEventType::NoteOn(note,_)  => {
                            Some(SanitizedNoteTemp{
                                global_time:time,
                                note,
                                start: true
                            })
                        },
                        nom_midi::MidiEventType::NoteOff(note,_) => {
                            Some(SanitizedNoteTemp{
                                global_time:time,
                                note,
                                start: false
                            })
                        },
                        _ => None,
                    }
                },
                _ => None,
            }
        }).collect();

        
        f.reverse();
        while let Some(next_note) = f.pop() {
            f.reverse();
            //find end
            let pos = f.iter().position(|el| el.note==next_note.note).unwrap();
            let end_time = f[pos].global_time;
            f.remove(pos);

            notes.push(SanitizedNote{
                note: next_note.note,
                length: end_time - next_note.global_time,
                global_time: next_note.global_time,
            });

            f.reverse();
        }

    }
    notes
}

fn main() {
    let midi = include_bytes!("../Floating Across Water.mid");
    
    let notes = extract_notes_2("./Floating Across Water.mid");
    
    println!("{:#?}", notes);

}

#[test]
fn test_simple() {
    let midi = include_bytes!("../a.mid");
    
    let notes = extract_notes(midi);
    
    println!("{:#?}", notes);

    assert_eq!(notes, vec![                        
        SanitizedNote {      
            global_time: 0,  
            length: 120,     
            note: C3         
        },                   
        SanitizedNote {      
            global_time: 240,
            length: 120,     
            note: A3         
        },                   
        SanitizedNote {      
            global_time: 240,
            length: 120,     
            note: F3         
        },                   
        SanitizedNote {      
            global_time: 360,
            length: 240,     
            note: G3         
        },                   
        SanitizedNote {      
            global_time: 480,
            length: 120,     
            note: E3         
        },                   
        SanitizedNote {      
            global_time: 600,
            length: 120,     
            note: D3         
        }                    
    ]);
}

#[test]
fn test_simple_2() {
    let midi = include_bytes!("../a.mid");
    
    let notes = extract_notes_2("./a.mid");
    
    println!("{:#?}", notes);

    // assert_eq!(notes, vec![                        
    //     SanitizedNote {      
    //         global_time: 0,  
    //         length: 120,     
    //         note: C3         
    //     },                   
    //     SanitizedNote {      
    //         global_time: 240,
    //         length: 120,     
    //         note: A3         
    //     },                   
    //     SanitizedNote {      
    //         global_time: 240,
    //         length: 120,     
    //         note: F3         
    //     },                   
    //     SanitizedNote {      
    //         global_time: 360,
    //         length: 240,     
    //         note: G3         
    //     },                   
    //     SanitizedNote {      
    //         global_time: 480,
    //         length: 120,     
    //         note: E3         
    //     },                   
    //     SanitizedNote {      
    //         global_time: 600,
    //         length: 120,     
    //         note: D3         
    //     }                    
    // ]);
}

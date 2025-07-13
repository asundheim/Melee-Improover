use tm_replay::*;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null_mut;
use slp_parser::*;
use std::ffi::OsStr;
use std::cmp;


#[repr(C)]

pub struct CharacterColourSafe {

    pub character: u8,

    pub colour: u8,
}


#[repr(C)]

pub struct GameInfoSafe {

    pub stage: Stage,

    pub port_used: [bool; 4],

    

    pub starting_character_colours: [CharacterColourSafe; 4],

    //pub starting_character_colours: [Option<CharacterColour>; 4],

    pub names: [[u8; 31]; 4],

    pub connect_codes: [[u8; 10]; 4],

    pub start_time: u64,//Time,

    /// In seconds. Zero if n/a.

    pub timer: u32,

    /// Not the frame length. Add 123 to get that.

    pub duration: i32,

    pub version_major: u8,

    pub version_minor: u8,

    pub version_patch: u8,
}



impl GameInfoSafe {

    pub fn new() -> Self {

        GameInfoSafe {

            stage: Stage::Battlefield, // or a specific variant if no Default impl

            port_used: [false; 4],

            starting_character_colours: [

                    CharacterColourSafe{character: 0, colour: 0}, 

                    CharacterColourSafe{character: 0, colour: 0}, 

                    CharacterColourSafe{character: 0, colour: 0}, 

                    CharacterColourSafe{character: 0, colour: 0},

                    ],

            names: [[0u8; 31]; 4],

            connect_codes: [[0u8; 10]; 4],

            start_time: 0, // implement Default or fill manually

            timer: 0,

            duration: 0,

            version_major: 1,

            version_minor: 0,

            version_patch: 0,

        }

    }





    pub fn min_version(&self, mj: u8, mn: u8, pt: u8) -> bool {

        if self.version_major < mj { return false; }

        if self.version_major > mj { return true; }

        if self.version_minor < mn { return false; }

        if self.version_minor > mn { return true; }

        if self.version_patch < pt { return false; }

        if self.version_patch > pt { return true; }

        true

    }



    /// Returns None if not a two player game

    pub fn low_high_ports(&self) -> Option<(usize, usize)> {

        let mut low_port = None;

        let mut high_port = None;



        for i in 0..4 {

            if self.port_used[i] {

                if low_port.is_none() {

                    low_port = Some(i);

                } else if high_port.is_none() {

                    high_port = Some(i);

                } else {

                    return None;

                }

            }

        }



        low_port.zip(high_port)

    }

}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_replay( name_ptr: *const c_char, p1_char:i32, p2_char:i32)//name: &str)
{
    if name_ptr.is_null() {
        eprintln!("Received null pointer for name!");
        return;
    }

    let test_char: slp_parser::CharacterColour;
    match p1_char{
        0 => test_char = slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::White),
        1 => test_char = slp_parser::CharacterColour::Falco(slp_parser::character_colours::FalcoColour::Blue),
        2 => test_char = slp_parser::CharacterColour::CaptainFalcon(slp_parser::character_colours::CaptainFalconColour::Green),
        3 => test_char = slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::Yellow),
        _ => test_char = slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::Yellow),
    }


    let test_char2: slp_parser::CharacterColour;
    match p2_char{
        0 => test_char2 = slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::White),
        1 => test_char2 = slp_parser::CharacterColour::Falco(slp_parser::character_colours::FalcoColour::Blue),
        2 => test_char2 = slp_parser::CharacterColour::CaptainFalcon(slp_parser::character_colours::CaptainFalconColour::Green),
        3 => test_char2 = slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::Yellow),
        _ => test_char2 = slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::Yellow),
    }
    let c_str = CStr::from_ptr( name_ptr );
    let u8_slice: &[u8] = c_str.to_bytes();

   // println!("test_replay called with name: {}", name);
    //let name= b"testexport";
    let mut filename_buf = [0u8; 31];
    filename_buf[0..u8_slice.len()].copy_from_slice(u8_slice);

    let inputs1: [Input; 1] = [
        Input::NONE.add(255).stick(80, 0)
    ];

    let inputs2: [Input; 1] = [
        Input::NONE.add(255).stick(-80, 0)
    ];

    let gci = construct_tm_replay(
        &RecordingState {
            time: RecordingTime::today_approx(),
            filename: filename_buf,
            menu_settings: Default::default(),
            start_frame: 0,
            stage: slp_parser::Stage::FinalDestination,//::Stage::FinalDestination,
            hmn_state: CharacterState {
                character: test_char,//slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::White),
                position: [0.0, 0.0, 0.0],
                state: slp_parser::ActionState::Standard(slp_parser::StandardActionState::AttackLw4),//slp_parser::ActionState::AttackLw4,
                state_frame: 0.0,
                direction: slp_parser::Direction::Right,
                ..Default::default()
            },
            cpu_state: CharacterState {
                character: test_char2,//slp_parser::Character::Fox.neutral(),
                position: [10.0, 0.0, 0.0],
                //state: slp_parser::ActionState::Standard(slp_parser::StandardActionState::AttackDsmash),
                state_frame: 0.0,
                direction: slp_parser::Direction::Left,
                ..Default::default()
            },
            cpu_follower_state: None,
            hmn_follower_state: None,
        },
        &InputRecordings {
            hmn_slots: [
                Some(&inputs1), // Slot 0
                None,           // Slot 1
                None,           // Slot 2
                None,           // Slot 3
                None,           // Slot 4
                None,           // Slot 5
            ],
            cpu_slots: [
                Some(&inputs2), // Slot 0
                None,           // Slot 1
                None,           // Slot 2
                None,           // Slot 3
                None,           // Slot 4
                None,           // Slot 5
            ],
        },
        0
    ).unwrap();

    let mut path = std::path::PathBuf::from("./");

    //let file : &str = path;
    //let ext: &str = ".gci";
 

    let test_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Invalid UTF-8 input");
            return;
        }
    };

    let save_file_name = format!("{}{}{}", "C:\\Users\\ficti\\Documents\\Dolphin\\FM-Slippi\\User\\GC\\USA\\Card A\\", test_str, ".gci");

    path.push(&save_file_name);
    std::fs::write(&path, &gci).unwrap();
    println!("wrote replay {}", path.display());
}



fn convert_character_colour_to_safe(colour: &CharacterColour) -> CharacterColourSafe {
    match colour {
        CharacterColour::Mario(c)             => CharacterColourSafe { character: 0,  colour: *c as u8 },
        CharacterColour::Fox(c)               => CharacterColourSafe { character: 1,  colour: *c as u8 },
        CharacterColour::CaptainFalcon(c)     => CharacterColourSafe { character: 2,  colour: *c as u8 },
        CharacterColour::DonkeyKong(c)        => CharacterColourSafe { character: 3,  colour: *c as u8 },
        CharacterColour::Kirby(c)             => CharacterColourSafe { character: 4,  colour: *c as u8 },
        CharacterColour::Bowser(c)            => CharacterColourSafe { character: 5,  colour: *c as u8 },
        CharacterColour::Link(c)              => CharacterColourSafe { character: 6,  colour: *c as u8 },
        CharacterColour::Sheik(c)             => CharacterColourSafe { character: 7,  colour: *c as u8 },
        CharacterColour::Ness(c)              => CharacterColourSafe { character: 8,  colour: *c as u8 },
        CharacterColour::Peach(c)             => CharacterColourSafe { character: 9,  colour: *c as u8 },
        CharacterColour::Popo(c)              => CharacterColourSafe { character: 10, colour: *c as u8 },
        CharacterColour::Nana(c)              => CharacterColourSafe { character: 11, colour: *c as u8 },
        CharacterColour::Pikachu(c)           => CharacterColourSafe { character: 12, colour: *c as u8 },
        CharacterColour::Samus(c)             => CharacterColourSafe { character: 13, colour: *c as u8 },
        CharacterColour::Yoshi(c)             => CharacterColourSafe { character: 14, colour: *c as u8 },
        CharacterColour::Jigglypuff(c)        => CharacterColourSafe { character: 15, colour: *c as u8 },
        CharacterColour::Mewtwo(c)            => CharacterColourSafe { character: 16, colour: *c as u8 },
        CharacterColour::Luigi(c)             => CharacterColourSafe { character: 17, colour: *c as u8 },
        CharacterColour::Marth(c)             => CharacterColourSafe { character: 18, colour: *c as u8 },
        CharacterColour::Zelda(c)             => CharacterColourSafe { character: 19, colour: *c as u8 },
        CharacterColour::YoungLink(c)         => CharacterColourSafe { character: 20, colour: *c as u8 },
        CharacterColour::DrMario(c)           => CharacterColourSafe { character: 21, colour: *c as u8 },
        CharacterColour::Falco(c)             => CharacterColourSafe { character: 22, colour: *c as u8 },
        CharacterColour::Pichu(c)             => CharacterColourSafe { character: 23, colour: *c as u8 },
        CharacterColour::MrGameAndWatch(c)    => CharacterColourSafe { character: 24, colour: *c as u8 },
        CharacterColour::Ganondorf(c)         => CharacterColourSafe { character: 25, colour: *c as u8 },
        CharacterColour::Roy(c)               => CharacterColourSafe { character: 26, colour: *c as u8 },
    }
}

#[repr(C)]
struct TestStruct{
    
    stage: Stage,
    test1: u32,
    test2: u32,
}


#[unsafe(no_mangle)]
pub extern "C" fn free_info(ptr: *mut GameInfoSafe) {
    if !ptr.is_null() {
        unsafe {
            // Take back ownership and drop
            Box::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn read_info( name_ptr: *const c_char ) -> *mut GameInfoSafe{

    
    if name_ptr.is_null() {
        eprintln!("Null pointer passed");
        return std::ptr::null_mut();
    }

  let mut sdi = slp_parser::SlpDirectoryInfo {
        slp_files: vec![],
        folders: vec![],
        dir_hash: 0,
    };
    
    //let result = slp_parser::read_info(p);
    

    let c_str = unsafe {CStr::from_ptr(name_ptr) };
    let bytes = c_str.to_bytes();
    let os_str = {
            
        // Windows paths are typically UTF-16; direct conversion from CStr can be lossy
        // If the C string is UTF-8 encoded, use:
        match std::str::from_utf8(bytes) {
            Ok(valid_str) => OsStr::new(valid_str),
            Err(_) => {
                //eprintln!("Invalid UTF-8 in input");
                //return null_mut();
                return null_mut();
            }
        }
    };

    let mut p: &Path = Path::new( os_str);//"F:\\Libs\\clip\\Game_20250616T153909.slp");


    match slp_parser::read_info(p){
        Ok(info) => {

            //let boxed = Box::new(info);

            // if info.stage != Stage::FinalDestination && info.stage != Stage::Battlefield && info.stage != Stage::YoshisStory
            // && info.stage != Stage::DreamLandN64 && info.stage != Stage::PokemonStadium && info.stage != tage::FountainOfDreams
            // {
            //     return null_mut();
            // }
            
            let mut safe = GameInfoSafe {
                stage: info.stage,
                port_used: [false; 4],
                starting_character_colours: [
                    CharacterColourSafe{character: 0, colour: 0}, 
                    CharacterColourSafe{character: 0, colour: 0}, 
                    CharacterColourSafe{character: 0, colour: 0}, 
                    CharacterColourSafe{character: 0, colour: 0},
                    ],
                names: [[0u8; 31]; 4],
                connect_codes: [[0u8; 10]; 4],
                start_time: info.start_time.0,
                timer: info.timer,
                duration: info.duration,
                version_major: info.version_major,
                version_minor: info.version_minor,
                version_patch: info.version_patch,
            };
            

            for i in 0..4 {
                safe.port_used[i] = info.port_used[i];
            }

            let mut safe_colours: [(u8, u8); 4] = [(0, 0); 4];

            for i in 0..4 {
                 safe_colours[i] = match &info.starting_character_colours[i] {
            Some(colour) => {
                let safe = convert_character_colour_to_safe(colour);
                (safe.character, safe.colour)
            },
            None => (0, 0), // Or any sentinel value you want for "no colour"
        
            };
            }

            for i in 0..4{
            safe.starting_character_colours[i].character = safe_colours[i].0;
            safe.starting_character_colours[i].colour = safe_colours[i].1;
            }

             // Copy names
            for i in 0..4 {
                safe.names[i] = info.names[i];
            }

            // Copy connect codes
            for i in 0..4 {
                safe.connect_codes[i] = info.connect_codes[i];
            }

            return Box::into_raw(Box::new(safe));
        }
         Err(e) => {
        eprintln!("Error reading info: {:?}", e);
        return std::ptr::null_mut();
        }
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn check_clips( name_ptr: *const c_char, out_len: *mut usize, hmnPlayer: i32, saveButtonType : i32 ) -> *mut i32{
    if name_ptr.is_null() {
        unsafe { *out_len = 0 };
        return null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(name_ptr) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            unsafe { *out_len = 0 };
            return null_mut();
        }
    };

    let mut real_save_button: u16 = 0;
    if saveButtonType == 0 {
        real_save_button = buttons_mask::D_PAD_DOWN;
    }
    else if saveButtonType == 1 {
        real_save_button = buttons_mask::D_PAD_RIGHT;
    }
    else if saveButtonType == 2 {
        real_save_button = buttons_mask::D_PAD_LEFT;
    }
    else if saveButtonType == 3 {
        real_save_button = buttons_mask::START;
    }
    else if saveButtonType == 4 {
        real_save_button = buttons_mask::X;
    }
    else if saveButtonType == 5 {
        real_save_button = buttons_mask::Y;
    }
    else if saveButtonType == 6 {
        real_save_button = buttons_mask::L_DIGITAL;
    }
    else if saveButtonType == 7 {
        real_save_button = buttons_mask::R_DIGITAL;
    }
    else {
        real_save_button = buttons_mask::D_PAD_DOWN;
    }

   // let result = slp_parser::read_info(path_str);

   let game = match read_game(std::path::Path::new(path_str)) { Ok(s) => s, Err(_) => panic!() };

    let mut clip_frames: Vec<i32> = Vec::new();
    if let Some((lo, hi)) = game.info.low_high_ports() {
        let frames_lo = game.frames[lo].as_ref().unwrap();
        let frames_hi = game.frames[hi].as_ref().unwrap();
        //let parsed_lo = parse_actions(frames_lo);
        //let parsed_hi = parse_actions(frames_hi);

        
        let mut start_frame = 0;
        let mut num_frames = 360;
        
        let mut name = String::from("new_recording");
        let mut flags = 0;

       
        if( hmnPlayer == 0 ) {
            for (i, f) in frames_lo.iter().enumerate() {
            //println!("Frame {}, {}", i, f.analog_trigger_value);
            if f.buttons_mask & real_save_button != 0
            {
                if( i > 0 && frames_lo[i - 1].buttons_mask & real_save_button == 0) {
                    //println!("Frame {}: D-Pad Right pressed", i);
                    clip_frames.push( i as i32 );
                }
                //println!("Frame {}: D-Pad Right pressed", i);
            }
        }
    }
        else if( hmnPlayer == 1 ) {
            for (i, f) in frames_hi.iter().enumerate() {
            //println!("Frame {}, {}", i, f.analog_trigger_value);
            if f.buttons_mask & real_save_button != 0
            {
                if( i > 0 && frames_hi[i - 1].buttons_mask & real_save_button == 0) {
                    //println!("Frame {}: D-Pad Right pressed", i);
                    clip_frames.push( i as i32 );
                }
                //println!("Frame {}: D-Pad Right pressed", i);
            }
        }
    }


        
    }


        // let mut default_clip_length = 360;
        // for (i, clip_frame) in clip_frames.iter().enumerate() {
        //     //println!("Value: {}", cmp::max( 0, number - 700));
        //     start_frame = cmp::max( 0, *clip_frame - default_clip_length);
        //     num_frames = cmp::min( *clip_frame, default_clip_length);


        //     name =  format!("new_recording_{}_frame_{}", i, *clip_frame);

        //     let savestate: Vec<u8> = construct_tm_replay_from_slp(&game,HumanPort::HumanLowPort,
        //     start_frame as usize,num_frames as usize,&name,flags,).unwrap();
        //       //  .map_err(|e| format!("Could not write"));
        
        //     let mut output_file = format!("{}.gci", name);

        //     std::fs::write(&output_file, &savestate);
        //      println!("Savestate file '{}' created", &output_file);
        // }


    // Dummy result: Pretend we found some clip info
    //let result = vec![1, 42, 99];

    unsafe {
        *out_len = clip_frames.len();
    }

    let boxed = clip_frames.into_boxed_slice();
    Box::into_raw(boxed) as *mut i32
}

#[unsafe(no_mangle)]
pub extern "C" fn free_clip_array(ptr: *mut i32, len: usize) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn create_savestates( 
    export_path_ptr: *const c_char, 
    file_name_ptr: *const c_char, 
    game_name_ptr: *const c_char, 
    hmn_player: i32, 
    clip_length: i32, 
    naming_style : i32, 
    saveButtonType : i32,  
    startFrame : i32) {
    if export_path_ptr.is_null() {
        return;
    }

    if file_name_ptr.is_null() {
        return;
    }

    if game_name_ptr.is_null() {
        return;
    }

    let file_c_str = unsafe { CStr::from_ptr(file_name_ptr) };
    let file_path_str = match file_c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return;
        }
    };

    let export_path_c_str = unsafe { CStr::from_ptr(export_path_ptr) };
    let export_path_str = match export_path_c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return;
        }
    };

    let game_name_c_str = unsafe { CStr::from_ptr(game_name_ptr) };
    let game_name_str = match game_name_c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return;
        }
    };

    let mut real_save_button: u16 = 0;
    if saveButtonType == 0 {
        real_save_button = buttons_mask::D_PAD_DOWN;
    }
    else if saveButtonType == 1 {
        real_save_button = buttons_mask::D_PAD_RIGHT;
    }
    else if saveButtonType == 2 {
        real_save_button = buttons_mask::D_PAD_LEFT;
    }
    else if saveButtonType == 3 {
        real_save_button = buttons_mask::START;
    }
    else if saveButtonType == 4 {
        real_save_button = buttons_mask::X;
    }
    else if saveButtonType == 5 {
        real_save_button = buttons_mask::Y;
    }
    else if saveButtonType == 6 {
        real_save_button = buttons_mask::L_DIGITAL;
    }
    else if saveButtonType == 7 {
        real_save_button = buttons_mask::R_DIGITAL;
    }
    else {
        real_save_button = buttons_mask::D_PAD_DOWN;
    }
    
    let game = match read_game(std::path::Path::new(file_path_str)) { Ok(s) => s, Err(_) => panic!() };

    let mut stored_low = 0;
    let mut stored_high = 0;
    let mut clip_frames: Vec<i32> = Vec::new();
    if let Some((lo, hi)) = game.info.low_high_ports() {
        let frames_lo = game.frames[lo].as_ref().unwrap();
        let frames_hi = game.frames[hi].as_ref().unwrap();

        stored_low = lo;
        stored_high = hi;

        let mut start_frame = 0;
        let mut num_frames = 360;
        
        let mut name = String::from("new_recording");
        let mut flags = 0;

        if startFrame == -1 {
            if hmn_player == 0 {
                for (i, f) in frames_lo.iter().enumerate() {
                    //println!("Frame {}, {}", i, f.analog_trigger_value);
                    if f.buttons_mask & real_save_button != 0
                    {
                        if i > 0 && frames_lo[i - 1].buttons_mask & real_save_button == 0 {
                            //println!("Frame {}: D-Pad Right pressed", i);
                            clip_frames.push( i as i32 );
                        }
                        //println!("Frame {}: D-Pad Right pressed", i);
                    }
                }
            }
            else if hmn_player == 1 {
                for (i, f) in frames_hi.iter().enumerate() {
                    //println!("Frame {}, {}", i, f.analog_trigger_value);
                    if f.buttons_mask & real_save_button != 0
                    {
                        if i > 0 && frames_hi[i - 1].buttons_mask & real_save_button == 0 {
                            //println!("Frame {}: D-Pad Right pressed", i);
                            clip_frames.push( i as i32 );
                        }
                        //println!("Frame {}: D-Pad Right pressed", i);
                    }
                }
            }
        }

        let SHORT_NAMES: [&str; 28] = [
            "Mario",
            "Fox",
            "CF",
            "DK",
            "Kirb",
            "Bow",
            "Link",
            "Sheik",
            "Ness",
            "Peach",
            "Popo",
            "Nana",
            "Pika",
            "Samus",
            "Yoshi",
            "Puff",
            "Mew2",
            "Luigi",
            "Marth",
            "Zelda",
            "YLink",
            "Doc",
            "Falco",
            "Pichu",
            "GaW",
            "Ganon",
            "Roy",
            "XX",
        ];

        let mut shortStageName = String::new();
        if game.info.stage == Stage::FinalDestination {
            shortStageName = String::from("FD");
        }
        else if game.info.stage == Stage::Battlefield {
            shortStageName = String::from("BF");
        }
        else if game.info.stage == Stage::YoshisStory {
            shortStageName = String::from("YS");
        }
        else if game.info.stage == Stage::DreamLandN64 {
            shortStageName = String::from("DL");
        }
        else if game.info.stage == Stage::PokemonStadium {
            shortStageName = String::from("PS");
        }
        else if game.info.stage == Stage::FountainOfDreams {
            shortStageName = String::from("FoD");
        }
        else {
            shortStageName = String::from("XX");
        
        }

        let mut default_clip_length = clip_length;//360;

        if startFrame == -1 {
            for (i, clip_frame) in clip_frames.iter().enumerate() {
                //println!("Value: {}", cmp::max( 0, number - 700));
                start_frame = cmp::max( 0, *clip_frame - default_clip_length);
                num_frames = cmp::min( *clip_frame, default_clip_length);

                if naming_style == 0 {
                    let mut ind = 0;
                    let mut opp = 0;
                    if hmn_player == 0
                    {
                        ind = stored_low;
                        opp = stored_high;
                    }
                    else if hmn_player ==1
                    {
                        ind = stored_high;
                        opp = stored_low;
                    }

                    name = format!("{}-{}-{}-{}-f{}", SHORT_NAMES[game.info.starting_character_colours[ind].unwrap().character() as usize].to_string(),
                            SHORT_NAMES[game.info.starting_character_colours[opp].unwrap().character() as usize].to_string(),
                            shortStageName, i + 1, start_frame);//month, day, year - 2000, hour, minute, i + 1 );//, day, year, hour, minute
                }
                else if naming_style == 1 {
                    name =  format!("{}-{}", game_name_str, i + 1);
                }
                else if naming_style == 2 {
                    name =  format!("{}-f{}", game_name_str, start_frame);
                }
                else {
                    name = format!("error");
                }
        
                let mut hport = HumanPort::HumanLowPort;
                if hmn_player == 1 {
                    hport = HumanPort::HumanHighPort;
                }

                let savestate: Vec<u8> = construct_tm_replay_from_slp(&game,hport,
                start_frame as usize,num_frames as usize,&name,flags,).unwrap();
        
                let output_file = std::path::Path::new(export_path_str).join(format!("{}.gci", name));

                std::fs::write(&output_file, &savestate);
                println!("Savestate file '{}' created", output_file.display());
            }
        }
        else {
            if naming_style == 0 {
                let mut ind = 0;
                let mut opp = 0;
                if hmn_player == 0
                {
                    ind = stored_low;
                    opp = stored_high;
                }
                else if hmn_player ==1
                {
                    ind = stored_high;
                    opp = stored_low;
                }

                name = format!("{}-{}-{}-{}-f{}", SHORT_NAMES[game.info.starting_character_colours[ind].unwrap().character() as usize].to_string(),
                        SHORT_NAMES[game.info.starting_character_colours[opp].unwrap().character() as usize].to_string(),
                        shortStageName, 1, start_frame);//month, day, year - 2000, hour, minute, i + 1 );//, day, year, hour, minute
            }
            else if naming_style == 1 {
                name =  format!("{}-{}", game_name_str, 1);
            }
            else if naming_style == 2 {
                name =  format!("{}-f{}", game_name_str, start_frame);
            }
            else {
                name = format!("error");
            }
        
            let mut hport = HumanPort::HumanLowPort;
            if hmn_player == 1 {
                hport = HumanPort::HumanHighPort;
            }

            start_frame = cmp::max( 0, startFrame - default_clip_length);
            num_frames = cmp::min( startFrame, default_clip_length);

            let savestate: Vec<u8> = construct_tm_replay_from_slp(&game,hport,
            start_frame as usize, num_frames as usize,&name,flags,).unwrap();
        
            let output_file = std::path::Path::new(export_path_str).join(format!("{}.gci", name));

            std::fs::write(&output_file, &savestate);
            println!("Savestate file '{}' created", output_file.display());
        }
    }
}

fn is_not_entry_state(state: ActionState) -> bool {
    match state {
        ActionState::Standard(inner) => !matches!(inner,
            StandardActionState::Entry |
            StandardActionState::EntryStart |
            StandardActionState::EntryEnd
        ),
        _ => true, // Not a Standard action, so it's not an Entry-related state
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn create_filtered_savestates( export_path_ptr: *const c_char, file_name_ptr: *const c_char, game_name_ptr: *const c_char, hmn_player: i32, clip_length: i32,  ) {

    if export_path_ptr.is_null() {
        return;
    }

    if file_name_ptr.is_null() {
        return;
    }

    if game_name_ptr.is_null() {
        return;
    }

//     let file_c_str = unsafe { CStr::from_ptr(file_name_ptr) };
//     let file_path_str = match file_c_str.to_str() {
//         Ok(s) => s,
//         Err(_) => {
//             return;
//         }
//     };

//     let export_path_c_str = unsafe { CStr::from_ptr(export_path_ptr) };
//     let export_path_str = match export_path_c_str.to_str() {
//         Ok(s) => s,
//         Err(_) => {
//             return;
//         }
//     };

//     let game_name_c_str = unsafe { CStr::from_ptr(game_name_ptr) };
//     let game_name_str = match game_name_c_str.to_str() {
//         Ok(s) => s,
//         Err(_) => {
//             return;
//         }
//     };

//    // let result = slp_parser::read_info(path_str);

//    let Ok((game, _)) = read_game(std::path::Path::new(file_path_str)) else { panic!(); };

//     let mut clip_frames: Vec<i32> = Vec::new();
//     if let Some((lo, hi)) = game.info.low_high_ports() {
//         let frames_lo = game.frames[lo].as_ref().unwrap();
//         let frames_hi = game.frames[hi].as_ref().unwrap();
//         //let parsed_lo = parse_actions(frames_lo);
//         //let parsed_hi = parse_actions(frames_hi);

        
//         let mut start_frame = 0;
//         let mut num_frames = 360;
        
//         let mut name = String::from("new_recording");
//         let mut flags = 0;

       
// //pub enum ActionState {
// //    Standard(StandardActionState),
// //    Special(SpecialActionState),

//         let mut myStart: i32 = 0;

//         if hmn_player == 0 {

//             for (i, f) in frames_lo.iter().enumerate() {
//                if is_not_entry_state(f.state){
//                     myStart = i as i32;
//                     break;
//                }
//         }

//             for (i, f) in frames_lo.iter().enumerate() {
//             //println!("Frame {}, {}", i, f.analog_trigger_value);

//             match f.state {
//                 ActionState::Standard(StandardActionState::AttackLw4) => {
//                     //println!("Matched AttackLw4");
//                     if f.anim_frame == 1.0 {
//                         clip_frames.push( i as i32 );
//                     }
//                 }
//                 _ => {}
//             }
//             // if let ActionState::Standard(StandardActionState::AttackLw4) = f.state {
//             //     //println!("Yes: player is waiting.");
//             //     if f.anim_frame == 1.0 {
//             //         //println!("Frame {}: D-Pad Right pressed", i);
//             //         clip_frames.push( i as i32 );
//             //     }
//             // }

//         }
//     }
//         else if hmn_player == 1 {

//             for (i, f) in frames_hi.iter().enumerate() {
//                if is_not_entry_state(f.state){
//                     myStart = i as i32;
//                     break;
//                }
//             }

//             for (i, f) in frames_hi.iter().enumerate() {
//              match f.state {
//                 ActionState::Standard(StandardActionState::AttackLw4) => {
//                     if f.anim_frame == 1.0 {
//                         clip_frames.push( i as i32 );
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }

//     let extra = 20;
//      let mut default_clip_length = clip_length;//360;
//         for (i, clip_frame) in clip_frames.iter().enumerate() {
//             //println!("Value: {}", cmp::max( 0, number - 700));
//             //cmp::max( 0, *clip_frame - default_clip_length);
//             // if start_frame == 0{
//             //      num_frames = *clip_frame + extra;
//             // }
//             // else {
//             //     num_frames = cmp::min( *clip_frame + extra, default_clip_length + extra);    
//             // }

//            // start_frame = (*clip_frame-600) as i32;
//             //    num_frames = 605;//(*clip_frame) + 5;
//             if *clip_frame < clip_length {
//                 start_frame = 0;
//                 num_frames = (*clip_frame - myStart) + extra;
//             }
//             else {
//                  start_frame = (*clip_frame-clip_length) as i32;
//                 num_frames = clip_length + extra;//(*clip_frame) + 5;
//             }
           
            


//             name =  format!("test_{}_mystart_{}_frame_{}", i + 1, myStart, *clip_frame);
            

//             let mut hport = HumanPort::HumanLowPort;
//             if hmn_player == 1 {
//                 hport = HumanPort::HumanHighPort;
//             }
//             let savestate: Vec<u8> = construct_tm_replay_from_slp(&game,hport,
//             start_frame as usize,num_frames as usize,&name,flags,).unwrap();
//               //  .map_err(|e| format!("Could not write"));
        
//             let mut output_file = format!("{}\\{}.gci", export_path_str, name);

//             std::fs::write(&output_file, &savestate);
//              println!("Savestate file '{}' created", &output_file);
//         }
//     }
}
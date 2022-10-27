// src/mode.rs

use std::collections::HashMap;
//use gilrs::EventType;
use macroquad_canvas::Canvas2D;

pub mod about_mode;
pub mod bdg_mode;
pub mod credits_mode;
pub mod cwg_title_mode;
pub mod menu_mode;
pub mod new_game_mode;
pub mod play_mode;
pub mod quit_mode;
pub mod settings_mode;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ModeTag {
    BdgMode,
    CwgTitleMode,
    MenuMode,
    NewGameMode,
    PlayMode,
    CreditsMode,
    AboutMode,
    SettingsMode,
    QuitMode,
}

pub trait GameMode {
    fn get_name(&self) -> String;

    fn init(&mut self);
    
    fn update(&mut self,
	      dt_seconds: f32,
	      //events: Vec<EventType>,
	      canvas: &Canvas2D) -> Option<ModeTag>;

    fn draw(&self, canvas: &Canvas2D);
}

pub struct ModeMgr {
    mode_registry : HashMap::<ModeTag, Box::<dyn GameMode>>,
    current_mode_tag : ModeTag,
}

impl ModeMgr {
    pub fn new(init_tag : ModeTag) -> ModeMgr {
	ModeMgr {
	    mode_registry : HashMap::<ModeTag, Box::<dyn GameMode>>::new(),
	    current_mode_tag : init_tag
	}
    }
    
    pub fn register_mode(&mut self, tag : ModeTag, mode : Box::<dyn GameMode>) {
	self.mode_registry.insert(tag, mode);
    }

    pub fn get_current_mode_tag(&self) -> ModeTag {
	self.current_mode_tag
    }

    pub fn get_current_mode(&mut self) -> &mut dyn GameMode {
	&mut **self.mode_registry.get_mut(&self.current_mode_tag).unwrap()
    }
    
    pub fn set_current_mode(&mut self, tag : ModeTag) {
	self.current_mode_tag = tag;
    }
}


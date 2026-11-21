use super::{Ui,Input,back,rebuild,activate,enter};
use forge::{input::Key,imgui};
// MHGURecomp 0.15.4+: ImGui capture flags alone do not release the host pointer.
unsafe extern "C"{fn forge_input_setMouseFree(free:bool);}
static mut MOUSE_FREE:bool=false;
// Korean fork: Hangul typing (hangul.rs). Han/Eng: the 한/영 key (LANG1, scancode 144) or Right Alt (230); Forge's key
// numbers are the keyboard's scancodes, so they are asked for by number.
unsafe extern "C"{#[link_name="forge_input_isKeyPressed"]fn key_pressed_raw(key:u32)->bool;}
static mut KOREAN:bool=true;
static mut COMPOSER:super::hangul::Composer=super::hangul::Composer::new();
pub fn korean()->bool{unsafe{KOREAN}}
fn composer()->&'static mut super::hangul::Composer{unsafe{&mut *core::ptr::addr_of_mut!(COMPOSER)}}
pub fn mouse_free(free:bool){unsafe{if MOUSE_FREE!=free{forge_input_setMouseFree(free);MOUSE_FREE=free;}}}
#[derive(Clone,Copy)]pub enum Action{BuilderEffect(usize),ArmorOverview,BuilderSkills,BuilderJewels,BuilderJewel(usize),BuilderStart,BuilderSelect(usize),BuilderResult(usize),BuilderWish,ItemSource(usize),ArmorFilter(bool),ArmorRank(i8),QuestPlan,QuestSelect(usize),QuestMaterial(usize),QuestMember(usize),ArmorSetWish,SearchKey(usize),Armor(usize),ArmorWish,QuickExpand,WishMaterial(usize,usize),WishRemove(usize),None,Wishlist,Back,Search,Tab(usize),Select(usize),Entry(usize),Zone(usize),Rank(usize),WeaponNode(usize),Recipe(usize),Material(usize),Panel(bool),ItemGroup(usize),Page(bool)}
pub fn apply(u:&mut Ui,a:Action){if u.builder&&u.detail==usize::MAX&&matches!(a,Action::Armor(_)|Action::BuilderJewels|Action::BuilderSkills|Action::BuilderWish){super::set_builder::browse();}match a{Action::BuilderEffect(i)=>super::set_builder::show_effect(i),Action::ArmorOverview=>{u.armor_overview=true;u.link=0;},Action::BuilderSkills=>super::set_builder::show_skills(),Action::BuilderJewels=>super::set_builder::show_jewels(),Action::BuilderJewel(i)=>super::set_builder::open_jewel(u,i),Action::BuilderStart=>{super::set_builder::run_toggle();u.dirty=true;},Action::BuilderSelect(i)=>{super::set_builder::toggle(i);},Action::BuilderResult(i)=>{super::set_builder::select_result(i);},Action::BuilderWish=>super::set_builder::add_missing(),Action::ItemSource(i)=>super::item::open_source(u,i),Action::ArmorFilter(gender)=>{let f=if gender{&mut u.armor_gender}else{&mut u.armor_hunter};*f=(*f+2)%3-1;u.selected=0;u.dirty=true;},Action::ArmorRank(i)=>{u.armor_rank=i;u.selected=0;u.dirty=true;},Action::QuestPlan=>{u.planner=true;super::quest::start();},Action::QuestSelect(i)=>super::quest::select(i),Action::QuestMaterial(i)=>super::quest::open_need(u,i),Action::QuestMember(i)=>super::quest::open_member(u,i),Action::ArmorSetWish=>super::armor::add_set(u),Action::SearchKey(i)=>{let done=super::search::edit(&mut u.query,&mut u.len,super::search::KEYS[i]);u.typing=!done;changed(u);},Action::Armor(i)=>enter(u,super::catalog::ENTRIES.len()+i),Action::ArmorWish=>{super::armor::toggle(u);},Action::WishMaterial(w,m)=>{u.selected=w;u.wish_material=m;super::wishlist::open_material(u);},Action::QuickExpand=>{u.quick_map=false;},
 Action::WeaponNode(id)=>{super::weapon::select(u,id);u.tree_focus=true;u.panel_entered=true;},Action::Recipe(i)=>{u.recipe=i;u.link=usize::MAX;u.tree_focus=false;u.panel_entered=true;},Action::Material(i)=>{super::weapon::open_material(u,i);},Action::Panel(tree)=>{u.tree_focus=tree;u.panel_entered=true;if !tree{u.link=0;}},
 Action::ItemGroup(i)=>super::item::group(u,i),Action::WishRemove(i)=>{u.selected=i;super::wishlist::remove_selected(u);},
 Action::Wishlist=>{if super::weapon::active(u){super::wishlist::toggle(u.detail_node,u.recipe);u.dirty=true;}},
 Action::None=>{},Action::Back=>back(u),Action::Search=>{u.editor.end(u.len,false);u.pc_search=true;u.typing=false;},
 Action::Tab(tab)=>{u.builder=false;super::quest::close_group();u.planner=false;u.tab=tab as u8;u.selected=0;u.wish_material=0;u.detail=usize::MAX;u.depth=0;u.len=0;u.pc_search=false;u.typing=false;u.armor_overview=false;u.dirty=true;if tab==5{super::set_builder::open(u);}},
 Action::Select(i)=>{u.pc_search=false;u.selected=i;activate(u);},Action::Entry(id)=>{u.pc_search=false;enter(u,id);},
 Action::Zone(i)=>{u.zone=i;u.selected=0;u.dirty=true;},Action::Rank(i)=>{u.rank=i as u8;if u.location>=0{u.zone=0;u.selected=0;}u.dirty=true;},
 Action::Page(down)=>{if u.builder&&u.detail==usize::MAX{super::set_builder::page(u,down);}else if u.tab==3&&u.detail==usize::MAX{if u.planner{super::quest::move_rows(down,8);}else{super::wishlist::move_material(u,down,8);}}else if super::item::active(u){super::item::scroll(u,down,8);}else if super::weapon::active(u){super::weapon::change_recipe(u,down);}else if u.detail==usize::MAX&&u.tab==2&&u.location>=0&&u.zone_count>0{u.zone=if down{(u.zone+1)%u.zone_count}else{(u.zone+u.zone_count-1)%u.zone_count};u.selected=0;u.dirty=true;}else if super::armor::active(u){if u.armor_overview{super::armor::move_skill(u,down,6);}else{super::armor::move_material(u,down,8);}}else if u.detail!=usize::MAX{let max=super::catalog::ENTRIES[u.detail].body.lines().count().saturating_sub(21);u.scroll=if down{(u.scroll+10).min(max)}else{u.scroll.saturating_sub(10)};let links=super::catalog::ENTRIES[u.detail].links.len();u.link=if down{(u.link+10).min(links.saturating_sub(1))}else{u.link.saturating_sub(10)};}else{u.selected=if down{(u.selected+10).min(u.count.saturating_sub(1))}else{u.selected.saturating_sub(10)};}}
 }rebuild(u);}
pub fn changed(u:&mut Ui){u.dirty=true;u.selected=0;u.wish_material=0;u.detail=usize::MAX;u.depth=0;}
pub fn keys(u:&mut Ui,p:&mut u32){
 // Held keyboard keys outrank their mapped game-button repeats for prompt style.
 if [Key::Up,Key::Down,Key::Left,Key::Right,Key::Enter,Key::Tab,Key::PageUp,Key::PageDown,Key::LeftBracket,Key::RightBracket,Key::F8,Key::F9,Key::F10,Key::A,Key::B,Key::C,Key::D,Key::E,Key::F,Key::G,Key::H,Key::I,Key::J,Key::K,Key::L,Key::M,Key::N,Key::O,Key::P,Key::Q,Key::R,Key::S,Key::T,Key::U,Key::V,Key::W,Key::X,Key::Y,Key::Z,Key::N0,Key::N1,Key::N2,Key::N3,Key::N4,Key::N5,Key::N6,Key::N7,Key::N8,Key::N9,Key::Space,Key::Minus,Key::Apostrophe,Key::Escape,Key::Backspace,Key::Delete,Key::Home,Key::End,Key::F6].into_iter().any(Input::is_key_down){u.desktop=true;}
 let wheel=imgui::get_io().MouseWheel;if wheel!=0.0{u.desktop=true;}
 if wheel!=0.0&&!u.pc_search{let down=wheel<0.0;if u.builder&&u.detail==usize::MAX&&super::set_builder::state().show_results{super::set_builder::page(u,down);}else if u.tab==3&&u.detail==usize::MAX{if u.planner{super::quest::move_rows(down,3);}else{super::wishlist::move_material(u,down,3);}}else if super::item::active(u){super::item::scroll(u,down,3);}else if super::armor::active(u){if u.armor_overview{super::armor::move_skill(u,down,3);}else{super::armor::move_material(u,down,3);}}else if u.detail!=usize::MAX{let max=super::catalog::ENTRIES[u.detail].body.lines().count().saturating_sub(21);u.scroll=if down{(u.scroll+3).min(max)}else{u.scroll.saturating_sub(3)};}else{u.selected=if down{(u.selected+3).min(u.count.saturating_sub(1))}else{u.selected.saturating_sub(3)};}}
 if Input::is_key_pressed(Key::Escape){u.desktop=true;back(u);*p=0;return;}
 if Input::is_ctrl_down()&&(Input::is_key_pressed(Key::F)||Input::is_key_pressed(Key::V)){if !u.pc_search{u.editor.end(u.len,false);}u.pc_search=true;u.typing=false;u.desktop=true;*p=0;}
 if unsafe{key_pressed_raw(144)||key_pressed_raw(230)}{unsafe{KOREAN=!KOREAN;}composer().finish();u.desktop=true;}
 let mut typed=[0u8;40];let mut typed_len=0;
 for (key,ch) in [(Key::A,b'a'),(Key::B,b'b'),(Key::C,b'c'),(Key::D,b'd'),(Key::E,b'e'),(Key::F,b'f'),(Key::G,b'g'),(Key::H,b'h'),(Key::I,b'i'),(Key::J,b'j'),(Key::K,b'k'),(Key::L,b'l'),(Key::M,b'm'),(Key::N,b'n'),(Key::O,b'o'),(Key::P,b'p'),(Key::Q,b'q'),(Key::R,b'r'),(Key::S,b's'),(Key::T,b't'),(Key::U,b'u'),(Key::V,b'v'),(Key::W,b'w'),(Key::X,b'x'),(Key::Y,b'y'),(Key::Z,b'z'),(Key::N0,b'0'),(Key::N1,b'1'),(Key::N2,b'2'),(Key::N3,b'3'),(Key::N4,b'4'),(Key::N5,b'5'),(Key::N6,b'6'),(Key::N7,b'7'),(Key::N8,b'8'),(Key::N9,b'9'),(Key::Space,b' '),(Key::Minus,b'-'),(Key::Apostrophe,b'\'')]{if Input::is_key_pressed(key){typed[typed_len]=ch;typed_len+=1;}}
 // Printable physical keys take precedence over the game's WASD/controller mapping.
 if typed_len>0&&!Input::is_ctrl_down()&&!Input::is_alt_down(){if !u.pc_search{u.editor.end(u.len,false);}u.pc_search=true;u.typing=false;u.desktop=true;*p=0;}
 if u.pc_search{
  // Moving vertically browses results immediately; horizontal arrows keep editing the caret.
  if !Input::is_ctrl_down(){let up=Input::is_key_pressed(Key::Up)||*p&(1<<13)!=0;let down=Input::is_key_pressed(Key::Down)||*p&(1<<15)!=0;if up||down{u.pc_search=false;*p=if down{1<<15}else{1<<13};return;}}
  if Input::is_ctrl_down(){u.desktop=true;*p=0;
   if Input::is_key_pressed(Key::A){u.editor.select_all(u.len);}
   if Input::is_key_pressed(Key::V){u.editor.insert(&mut u.query,&mut u.len,imgui::get_clipboard_text().lines().next().unwrap_or(""));changed(u);}
   if Input::is_key_pressed(Key::C)||Input::is_key_pressed(Key::X){let(l,r)=u.editor.selection();if l<r{let mut copy=[0u8;49];copy[..r-l].copy_from_slice(&u.query[l..r]);if let Ok(s)=core::ffi::CStr::from_bytes_with_nul(&copy[..r-l+1]){imgui::set_clipboard_text(s);}if Input::is_key_pressed(Key::X){u.editor.delete(&mut u.query,&mut u.len,false);changed(u);}}}
   return;
  }
  let finish=*p&3!=0;*p=0;if finish||Input::is_key_pressed(Key::Enter){composer().finish();u.pc_search=false;return;}
  let shift=Input::is_shift_down();
  if [Key::Left,Key::Right,Key::Home,Key::End,Key::Delete].into_iter().any(Input::is_key_pressed){composer().finish();}
  for(key,right)in[(Key::Left,false),(Key::Right,true)]{if Input::is_key_pressed(key){u.editor.step(&u.query[..u.len],right,shift);}}
  if Input::is_key_pressed(Key::Home){u.editor.home(shift);}if Input::is_key_pressed(Key::End){u.editor.end(u.len,shift);}
  for &ch in &typed[..typed_len]{
   let jamo=if korean(){super::hangul::jamo(ch,shift)}else{None};
   if let Some(j)=jamo{let open=composer().open;let out=composer().key(j);u.editor.select_back(open);u.editor.insert(&mut u.query,&mut u.len,out.str());}
   else{composer().finish();let c=if shift{ch.to_ascii_uppercase()}else{ch};let b=[c];u.editor.insert(&mut u.query,&mut u.len,core::str::from_utf8(&b).unwrap_or(""));}
  }
  if typed_len>0{changed(u);}
  if Input::is_key_pressed(Key::Backspace){let open=composer().open;match composer().back(){Some(out)=>{u.editor.select_back(open);u.editor.insert(&mut u.query,&mut u.len,out.str());}None=>u.editor.delete(&mut u.query,&mut u.len,true)}changed(u);}
  if Input::is_key_pressed(Key::Delete){u.editor.delete(&mut u.query,&mut u.len,false);changed(u);}
 }else{
  for(key,bit)in[(Key::Up,13),(Key::Down,15),(Key::Left,12),(Key::Right,14),(Key::Enter,0),(Key::Tab,7),(Key::LeftBracket,8),(Key::RightBracket,9),(Key::F8,5),(Key::F9,2),(Key::F10,4)]{if Input::is_key_pressed(key){u.desktop=true;*p|=1<<bit;}}
  if Input::is_key_pressed(Key::PageDown){u.desktop=true;apply(u,Action::Page(true));}if Input::is_key_pressed(Key::PageUp){u.desktop=true;apply(u,Action::Page(false));}
 }
}
pub fn pointer(u:&mut Ui)->Option<(f32,f32)>{
 let io=imgui::get_io();if io.MouseClicked[0]{u.desktop=true;return Some((io.MousePos.x,io.MousePos.y));}
 let touch=Input::get_touch();let clicked=touch.is_some()&&!u.touch_held;u.touch_held=touch.is_some();
 if !clicked{return None;}u.desktop=true;let(x,y)=touch?;
 // Forge exposes the centered 1280x720 game HUD canvas, not window pixels.
 let w=io.DisplaySize.x;let h=io.DisplaySize.y;let scale=(w/1280.0).min(h/720.0);
 Some(((w-1280.0*scale)/2.0+x*scale,(h-720.0*scale)/2.0+y*scale))
}

/// Keyboard equivalents use the same actions as controller hints.
pub fn prompt(label:&str)->super::Text{use core::fmt::Write;let mut out=super::Text::new();let mut rest=label;
 let rules=[("X to edit","Ctrl+F to edit"),("Press R3","Press F8"),("B  Details","Enter"),("B:","Enter:"),("A:","Esc:"),("B  ","Enter  "),("A  ","Esc  "),("X  ","Ctrl+F  "),("Y  ","F9  "),("Y:","F9:"),("R3  ","F8  "),("R3:","F8:"),("L3  ","F10  "),("L3:","F10:"),("LB / RB","Tab"),("LT / RT","[ / ]"),("D-pad / left stick","Arrow keys"),("D-pad","Arrows"),("Right stick: scroll details","Mouse wheel: scroll details"),("방향키 / 왼쪽 스틱","화살표 키"),("방향키","화살표"),("오른쪽 스틱: 상세 스크롤","마우스 휠: 상세 스크롤"),("Y로","F9로")];
 while !rest.is_empty(){if let Some((from,to))=rules.iter().find(|(from,_)|rest.starts_with(from)&&(rest.len()==label.len()||label.as_bytes()[label.len()-rest.len()-1].is_ascii_whitespace())){let _=write!(out,"{}",to);rest=&rest[from.len()..];}else{let ch=rest.chars().next().unwrap();let _=write!(out,"{}",ch);rest=&rest[ch.len_utf8()..];}}
 out
}

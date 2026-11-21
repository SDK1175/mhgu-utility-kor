use super::{Ui,armor_data,catalog,wishlist,enter};
pub const SLOTS:[&str;5]=["Head","Chest","Arms","Waist","Legs"];
pub fn rank(a:&armor_data::Armor)->i8{if a.rarity<=3{0}else if a.rarity<=7{1}else{2}}
pub fn set_name(a:&armor_data::Armor)->&'static str{armor_data::SET_NAMES.binary_search_by_key(&a.family,|s|s.0).ok().map(|i|armor_data::SET_NAMES[i].1).unwrap_or(a.name)}
pub fn active(u:&Ui)->bool{u.detail>=catalog::ENTRIES.len()&&u.detail-catalog::ENTRIES.len()<armor_data::ARMOR.len()}
pub fn index(u:&Ui)->usize{u.detail-catalog::ENTRIES.len()}
pub fn materials(id:usize)->wishlist::Materials{let a=&armor_data::ARMOR[id];wishlist::Materials::Armor(a.material as usize,a.material_count as usize)}
pub fn toggle(u:&mut Ui){wishlist::toggle(wishlist::ARMOR_BASE+index(u),0);u.dirty=true;}
pub fn add_set(u:&mut Ui){let family=armor_data::ARMOR[index(u)].family;for(i,a)in armor_data::ARMOR.iter().enumerate(){if a.family==family&&!wishlist::contains(wishlist::ARMOR_BASE+i,0){wishlist::toggle(wishlist::ARMOR_BASE+i,0);if wishlist::failed(){break;}}}u.dirty=true;}
pub fn move_material(u:&mut Ui,down:bool,step:usize){let max=materials(index(u)).len().saturating_sub(1);u.link=if down{(u.link+step).min(max)}else{u.link.saturating_sub(step)};}
pub fn input(u:&mut Ui,p:u32){
 if p&(1<<5)!=0{if u.armor_overview{add_set(u);}else{toggle(u);}}if p&(1<<2)!=0{add_set(u);}
 if u.armor_overview{if p&(1<<15)!=0{move_skill(u,true,1);}if p&(1<<13)!=0{move_skill(u,false,1);}}else{if p&(1<<15)!=0{move_material(u,true,1);}if p&(1<<13)!=0{move_material(u,false,1);}if p&1!=0{if let Some(m)=materials(index(u)).get(u.link){if m.entry!=65535{enter(u,m.entry as usize);}}}}
 if p&((1<<12)|(1<<14))!=0{let current=index(u);let a=&armor_data::ARMOR[current];let mut family=[usize::MAX;5];for(i,s)in armor_data::ARMOR.iter().enumerate(){if s.family==a.family{family[s.slot as usize]=i;}}
 let at=if u.armor_overview{0}else{a.slot as usize+1};let down=p&(1<<14)!=0;for step in 1..=6{let next=(at+if down{step}else{6-step})%6;if next==0{u.armor_overview=true;u.link=0;break;}if family[next-1]!=usize::MAX{u.detail=catalog::ENTRIES.len()+family[next-1];u.armor_overview=false;u.link=0;break;}}}
}
pub fn set_label(a:&armor_data::Armor)->super::Text{use core::fmt::Write;let mut text=super::Text::new();let name=set_name(a);let ko=super::ko::tr(name);let _=write!(text,"{}",ko);for suffix in [" XR"," X"," S"," R"," Z"]{if a.name.ends_with(suffix)&&!name.ends_with(suffix)&&!ko.ends_with(suffix.trim_start()){let _=write!(text,"{}",suffix);break;}}text}
pub struct Summary{pub defense:u16,pub max_defense:u16,pub resist:[i16;5],pub slots:u8,pub pieces:u8,pub skills:crate::set_skills::Totals,pub skill_rows:[u8;206],pub skill_count:usize}
pub fn summary(id:usize)->Summary{let mut e=crate::set_skills::Evaluator::new();let mut torso=0;let family=armor_data::ARMOR[id].family;let mut out=Summary{defense:0,max_defense:0,resist:[0;5],slots:0,pieces:0,skills:crate::set_skills::Totals{points:[0;206],present:[false;206]},skill_rows:[0;206],skill_count:0};
 for a in armor_data::ARMOR.iter().filter(|a|a.family==family){out.defense+=a.defense;out.max_defense+=a.max_defense;out.slots+=a.slots;out.pieces+=1;for i in 0..5{out.resist[i]+=i16::from(a.resist[i]);}for &(tree,p)in &armor_data::SKILLS[a.skill as usize..a.skill as usize+a.skill_count as usize]{let tree=crate::set_data::ARMOR_TREES[tree as usize];e.add(a.slot as usize,tree,i16::from(p));if a.slot!=1&&tree==crate::set_data::TORSO_UP{torso+=1;}}}
 e.set_torso_count(torso);out.skills=e.finish();for tree in 1..206{if out.skills.present[tree]&&out.skills.points[tree]!=0{out.skill_rows[out.skill_count]=tree as u8;out.skill_count+=1;}}out
}

pub fn hunter_filter(u:&Ui)->&'static str{["Type: All","Type: Blademaster","Type: Gunner"][(u.armor_hunter+1)as usize]}
pub fn gender_filter(u:&Ui)->&'static str{["Gender: All","Gender: Male","Gender: Female"][(u.armor_gender+1)as usize]}

pub fn move_skill(u:&mut Ui,down:bool,step:usize){let count=summary(index(u)).skill_count;u.link=if down{(u.link+step).min(count.saturating_sub(1))}else{u.link.saturating_sub(step)};}

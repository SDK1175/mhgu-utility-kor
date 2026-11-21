//! Read-only MHGU 1.4 player/equipment capture. No native skill rebuilds (those
//! write caches), no save writes. Unknown progression fields remain None.
//! Layout evidence: f_34133c (2000 records, stride 44, +0x1c), f_340f60 /
//! f_725878 (capacity), f_2ddd48 (raw talisman skills), f_2db5c4 (slots).
pub const MAX_EQUIPMENT:usize=2000;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Status{Unavailable,Ready,UnsupportedBuild,InvalidData,Changed,CapacityExceeded}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct Skill{pub tree:u8,pub points:i8}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct Charm{pub box_index:u16,pub slots:u8,pub skills:[Skill;2]}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct OwnedArmor{pub slot:u8,pub native:u16}
pub struct Snapshot{pub status:Status,pub gender:Option<u8>,pub hunter:Option<u8>,pub weapon_slots:Option<u8>,pub hr:Option<u16>,pub village_stars:Option<u8>,pub capacity:usize,pub owned:[OwnedArmor;MAX_EQUIPMENT],pub owned_count:usize,pub charms:[Charm;MAX_EQUIPMENT],pub charm_count:usize}
impl Snapshot{
 pub const fn new()->Self{Self{status:Status::Unavailable,gender:None,hunter:None,weapon_slots:None,hr:None,village_stars:None,capacity:0,owned:[OwnedArmor{slot:0,native:0};MAX_EQUIPMENT],owned_count:0,charms:[Charm{box_index:0,slots:0,skills:[Skill{tree:0,points:0};2]};MAX_EQUIPMENT],charm_count:0}}
 pub fn owns(&self,slot:u8,native:u16)->bool{self.status==Status::Ready&&self.owned[..self.owned_count].iter().any(|a|a.slot==slot&&a.native==native)}
 fn invalidate(&mut self,status:Status){self.status=status;self.gender=None;self.hunter=None;self.weapon_slots=None;self.hr=None;self.village_stars=None;self.capacity=0;self.owned_count=0;self.charm_count=0;}
}
/// Reads may fail; a missing byte must never be interpreted as zero stock.
pub trait Memory{
 fn word(&self,address:u32)->Option<u32>;
 fn half(&self,address:u32)->Option<u16>;
 fn byte(&self,address:u32)->Option<u8>;
 /// Pure table lookup, never f_2ddd48 or another cache-building routine.
 fn equipment_slots(&self,_record:u32)->Option<u8>{None}
}
#[cfg(not(test))]unsafe extern "C"{fn mhxx_gu_data(offset:u32)->u32;}
/// Korean fork (MHXX Recomp): GU's data at base+offset sits elsewhere in MHXX, and the port says where (an older port,
/// or GU itself: 0, the address stays GU's).
pub fn gd(b:u32,off:u32)->u32{#[cfg(not(test))]{let a=unsafe{mhxx_gu_data(off)};if a!=0{return a;}}b.wrapping_add(off)}
/// An address in the game's image: GU's (base..base+len), or on the port MHXX's own (where vtables and the functions
/// they hold really are).
pub fn in_image(b:u32,v:u32,len:u32)->bool{(v>=b&&v<b.wrapping_add(len))||(0x0020_0000..0x0240_0000).contains(&v)}
fn valid(p:u32)->bool{(0x08000000..0xa7fb0000).contains(&p)&&p&3==0}
pub fn capacity(flags:u32,expansions:u32)->usize{1000+200*((flags>>21&1)+(flags>>22&1)+(expansions&7).count_ones())as usize}
// Native weapon level records are inline objects (e.g. f_3327b0 uses
// stride20). Interpret their proven byte accessor instead of treating the
// native helper's zero-on-missing-resource return as known zero slots.
pub fn decode_weapon_row(m:&impl Memory,b:u32,row:u32)->Option<u8>{
 if !valid(row){return None;}let v=m.word(row)?;
 if v&3!=0||!in_image(b,v,0x1850000){return None;}
 let f=m.word(v+0x38)?;if f&3!=0||!in_image(b,f,0x13f2000){return None;}
 let load=m.word(f)?;if load&0xfffff000!=0xe5d00000||m.word(f+4)!=Some(0xe12fff1e){return None;}
 let offset=load&0xfff;if offset>=64{return None;}
 m.byte(row.checked_add(offset)?).filter(|n|*n<=3)
}
// Korean fork, diagnosis: the last step capture reached and its values (logged by refresh when it fails).
pub static mut DIAG:[u32;5]=[0;5];
fn d(step:u32,a:u32,b:u32,c:u32,e:u32){unsafe{DIAG=[step,a,b,c,e];}}
fn capture(m:&impl Memory,b:u32,s:&mut Snapshot)->Result<(),Status>{d(1,b,0,0,0);
 let unavailable=Status::Unavailable;let invalid=Status::InvalidData;
 let save=m.word(gd(b,0x1897f78)).ok_or(unavailable)?;
 let bx=m.word(gd(b,0x1856fe8)).ok_or(unavailable)?;let gc=m.word(gd(b,0x188ca48)).ok_or(unavailable)?;let manager=m.word(gd(b,0x1885b58)).ok_or(unavailable)?;d(2,save,bx,gc,manager);
 if !valid(save)||!valid(bx)||!valid(gc)||!valid(manager){return Err(unavailable);}
 d(3,m.word(save).unwrap_or(1),gd(b,0x176af48),m.word(bx).unwrap_or(1),gd(b,0x173e3d0));
 if m.word(save)!=Some(gd(b,0x176af48))||m.word(bx)!=Some(gd(b,0x173e3d0)){return Err(Status::UnsupportedBuild);}
 // Town-only capture avoids departure/replacement and Prowler equipment data.
 d(4,m.byte(gc+0x4c).map_or(999,u32::from),0,0,0);if m.byte(gc+0x4c).ok_or(unavailable)?>7{return Err(unavailable);}
 let local=m.word(gc+0x30).ok_or(unavailable)?;d(5,local,0,0,0);if local>=4{return Err(invalid);}
 let actor=m.word(manager+0x1c+local*4).ok_or(unavailable)?;d(6,actor,0,0,0);if !valid(actor){return Err(unavailable);}
 let pd=m.word(actor+0x13c4).ok_or(unavailable)?;d(7,pd,0,0,0);if !valid(pd){return Err(unavailable);}
 let gender=m.byte(pd+0x4d8).ok_or(unavailable)?;d(8,u32::from(gender),m.word(pd+0x14c).unwrap_or(999),m.half(pd+0x554).map_or(999,u32::from),0);if gender>1{return Err(invalid);}
 let wt=m.word(pd+0x14c).ok_or(unavailable)?;
 let hunter=match wt{4|6|10=>1,0..=3|7..=9|11..=14=>0,_=>return Err(unavailable)};
 let hr=m.half(pd+0x554).ok_or(unavailable)?;if hr==0||hr>999{return Err(invalid);}
 // Native weapon types are record tags7..21 (tag12 is the unused weapon5).
 let weapon=pd+0x18;let weapon_bits=m.half(weapon+4).ok_or(unavailable)?;let weapon_id=m.half(weapon+6).ok_or(unavailable)?;
 d(9,u32::from(weapon_bits),u32::from(weapon_id),wt,0);if u32::from(weapon_bits&31)!=wt+7||weapon_id==0{return Err(invalid);}
 d(10,0,0,0,0);let slots=m.equipment_slots(weapon).ok_or(unavailable)?;if slots>3{return Err(invalid);}
 let flags=m.word(save+0x958).ok_or(unavailable)?;let expansions=m.word(save+0x3614).ok_or(unavailable)?;
 d(11,flags,expansions,0,0);s.capacity=capacity(flags,expansions);if s.capacity>MAX_EQUIPMENT{return Err(Status::CapacityExceeded);}
 // Scan the entire proven physical array, not a hard-coded 1000-charms prefix.
 // A record beyond currently unlocked capacity is inconsistent, never ignored.
 for i in 0..MAX_EQUIPMENT{
  let p=bx+0x1c+i as u32*44;let bits=m.half(p+4).ok_or(unavailable)?;let ty=(bits&31)as u8;d(12,i as u32,u32::from(bits),m.half(p+6).map_or(0,u32::from),s.capacity as u32);
  if ty==0{continue;}
  if i>=s.capacity{return Err(invalid);}
  if ty>24{return Err(invalid);}
  let id=m.half(p+6).ok_or(unavailable)?;if id==0{return Err(invalid);}
  match ty{
   1..=5=>{if s.owned_count==s.owned.len(){return Err(Status::CapacityExceeded);}s.owned[s.owned_count]=OwnedArmor{slot:ty-1,native:id};s.owned_count+=1;},
   6=>{
    if s.charm_count==s.charms.len(){return Err(Status::CapacityExceeded);}
    let slots=m.byte(p+0x14).ok_or(unavailable)?;if slots>3{return Err(invalid);}
    let mut skills=[Skill::default();2];for k in 0..2{let tree=m.byte(p+0x10+k).ok_or(unavailable)?;let points=m.byte(p+0x12+k).ok_or(unavailable)?as i8;if tree>205||(tree==0&&points!=0){return Err(invalid);}skills[k as usize]=Skill{tree,points};}
    if m.byte(p+0x14)!=Some(slots)||(0..2).any(|k|m.byte(p+0x10+k)!=Some(skills[k as usize].tree)||m.byte(p+0x12+k)!=Some(skills[k as usize].points as u8)){return Err(Status::Changed);}
    s.charms[s.charm_count]=Charm{box_index:i as u16,slots,skills};s.charm_count+=1;
   },_=>{}
  }
  if m.half(p+4)!=Some(bits)||m.half(p+6)!=Some(id){return Err(Status::Changed);}
 }
 // Do not publish a snapshot across character/scene replacement.
 d(13,0,0,0,0);
 if m.word(gd(b,0x1897f78))!=Some(save)||m.word(gd(b,0x1856fe8))!=Some(bx)||m.word(gd(b,0x188ca48))!=Some(gc)||m.word(gd(b,0x1885b58))!=Some(manager)||m.word(gc+0x30)!=Some(local)||m.word(manager+0x1c+local*4)!=Some(actor)||m.word(actor+0x13c4)!=Some(pd)||m.word(save+0x958)!=Some(flags)||m.word(save+0x3614)!=Some(expansions)||m.byte(gc+0x4c).is_none_or(|v|v>7)||m.byte(pd+0x4d8)!=Some(gender)||m.word(pd+0x14c)!=Some(wt)||m.half(pd+0x554)!=Some(hr)||m.half(weapon+4)!=Some(weapon_bits)||m.half(weapon+6)!=Some(weapon_id)||m.equipment_slots(weapon)!=Some(slots){return Err(Status::Changed);}
 s.gender=Some(gender);s.hunter=Some(hunter);s.weapon_slots=Some(slots);s.hr=Some(hr);s.status=Status::Ready;Ok(())
}
pub fn decode(m:&impl Memory,base:u32,out:&mut Snapshot)->Result<(),Status>{out.invalidate(Status::Unavailable);let r=capture(m,base,out);if let Err(status)=r{out.invalidate(status);}r}
#[cfg(not(test))]mod live{
 use super::*;use core::{cell::UnsafeCell,ptr,mem};
 struct Local(UnsafeCell<Snapshot>);unsafe impl Sync for Local{}
 static STATE:Local=Local(UnsafeCell::new(Snapshot::new()));
 struct Direct{base:u32}
 impl Memory for Direct{
  fn word(&self,a:u32)->Option<u32>{Some(unsafe{ptr::read_volatile(a as *const u32)})}
  fn half(&self,a:u32)->Option<u16>{Some(unsafe{ptr::read_volatile(a as *const u16)})}
  fn byte(&self,a:u32)->Option<u8>{Some(unsafe{ptr::read_volatile(a as *const u8)})}
  fn equipment_slots(&self,record:u32)->Option<u8>{
   // f_2db5c4 resolves resident equipment tables and reads slot metadata only.
   d(20,self.word(self.base+0xdb5c4).unwrap_or(1),self.word(self.base+0x13f03c).unwrap_or(1),self.word(self.base+0x13c208).unwrap_or(1),0);
   if self.word(self.base+0xdb5c4)!=Some(0xe92d4830)||self.word(self.base+0x13f03c)!=Some(0xe1d120b4)||self.word(self.base+0x13c208)!=Some(0xe92d4070){return None;}
   let ty=u32::from(self.half(record+4)?&31);let id=u32::from(self.half(record+6)?);
   if !(7..=21).contains(&ty)||ty==12||id==0{return None;}
   // f_33c208 weapon switch reads these resident tables. Check the table and
   // its count before calling its resolver; a missing resource also yields0
   // from f_2db5c4 and must not masquerade as a valid zero-slot weapon.
   let root=self.word(gd(self.base,0x1856b18))?;d(21,root,ty,id,0);if !valid(root){return None;}
   let offset=0x50+4*(ty-7-u32::from(ty>12));let table=self.word(root+offset)?;
   d(22,table,self.word(table+0x64).unwrap_or(1),id,0);if !valid(table)||id>=self.word(table+0x64)?{return None;}
   let resolve:unsafe extern "C" fn(u32,u32)->u32=unsafe{mem::transmute(self.base+0x13f03c)};
   let resource=unsafe{resolve(root,record)};d(23,resource,0,0,0);if !valid(resource){return None;}
   let vtable=self.word(resource)?;d(24,vtable,self.word(vtable+0xd8).unwrap_or(1),0,0);if vtable&3!=0||!in_image(self.base,vtable,0x1850000){return None;}
   let level_getter=self.word(vtable+0xd8)?;if level_getter&3!=0||!in_image(self.base,level_getter,0x13f2000){return None;}
   // The weapon branch of f_2db5c4 first resolves the selected upgrade level;
   // a null level row also makes that native helper return0.
   let level=(self.half(record+4)?>>5)&31;
   let lookup:unsafe extern "C" fn(u32,u32)->u32=unsafe{mem::transmute(level_getter)};
   let row=unsafe{lookup(resource,u32::from(level))};d(25,row,u32::from(level),self.word(row).unwrap_or(1),0);decode_weapon_row(self,self.base,row)
  }
 }
 pub fn refresh()->bool{let b=forge::mem::text_addr();let r=unsafe{decode(&Direct{base:b},b,&mut*STATE.0.get())};
  // Korean fork, diagnosis: where capture stopped (once per distinct place and values)
  static mut LAST:[u32;6]=[0;6];
  if let Err(status)=r{let v=unsafe{DIAG};let key=[status as u32,v[0],v[1],v[2],v[3],v[4]];
   if unsafe{LAST}!=key{unsafe{LAST=key;}forge::log::prelude::warn!("set_player: {:?} at step {} [{:#x} {:#x} {:#x} {:#x}] (text {:#x})",status,v[0],v[1],v[2],v[3],v[4],b);}}
  else{forge::log::prelude::info!("set_player: ready, {} armor, {} charms, HR {:?}",snapshot().owned_count,snapshot().charm_count,snapshot().hr);}
  r.is_ok()}
 pub fn snapshot()->&'static Snapshot{unsafe{&*STATE.0.get()}}
}
#[cfg(not(test))]pub use live::*;

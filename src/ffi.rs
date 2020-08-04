#[link(name = "c")]
extern "C" {
  fn geteuid() -> u32;
}

pub fn get_user_id() -> u32 {
  unsafe { geteuid() }
}

pub fn helper_caller() {
  println!("Calling from helper caller")
}

pub fn deep_caller() {
  deep();
}

fn deep() {
  println!("Calling from dep")
}
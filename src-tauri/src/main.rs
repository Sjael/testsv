#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use palette::*;
use palette::Hue;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .invoke_handler(tauri::generate_handler![generate_gradient])
    .run(context)
    .expect("error while running tauri application");
}


#[tauri::command]
fn generate_gradient(r: u8, g: u8, b: u8) -> (Vec<Vec<u8>>, Vec<Vec<u8>>){
  let myrgb = Srgb::new(
    r as f32 / 255.,
    g as f32 / 255.,
    b as f32 / 255.
  );

  let my_lch = Lch::from_color(myrgb.into_linear());
  let gradient = Gradient::new(vec![
    Lch::new(0.0, my_lch.chroma, my_lch.hue),
    my_lch,
    Lch::new(128.0, my_lch.chroma, my_lch.hue),
  ]);
  let mut orig_colors : Vec<Vec<u8>> = Vec::new();
  let colors = gradient
    .take(10)
    .enumerate()
    .map(|(idx, color)| {
      //println!("before {}", color.into_components());
      let (r2,g2,b2) = Srgb::from_color(color).into_components();
      let lol = vec![
        (r2 * 255.) as u8, 
        (g2 * 255.) as u8, 
        (b2 * 255.) as u8
      ];
      orig_colors.push(lol.clone());
      let (r,g,b) = Srgb::from_color(color.shift_hue(15. * idx as f32)).into_components();
      println!("{} {} {}", r, g ,b);
      //let (r,g,b) = Srgb::from_color(color).into_components();
      vec![
        (r * 255.) as u8, 
        (g * 255.) as u8, 
        (b * 255.) as u8
      ]
    })
    .collect::<Vec<_>>();
  (colors, orig_colors)
}


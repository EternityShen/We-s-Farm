use bevy::prelude::*;

use we_s_fram::plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(plugins::world::WorldPlugin)
        .add_plugins(plugins::camera::MainCameraPlugin)
        .add_plugins(plugins::sunhight::SunHightPlugin)
        .add_plugins(plugins::player::PlayerPlugin)
        .add_plugins(plugins::farming::soil::SoilPlugin)
        .add_plugins(plugins::check_box::CheckBoxPlugin)
        .run();
}

// use std::{
//     collections::HashSet,
//     ops::{Add, Sub},
// };
//
// fn main() {
//     let mask_set = [
//         (IVec2::new(0, 1), 1, "up"),    // 上 (North)
//         (IVec2::new(1, 0), 2, "right"), // 右 (East)
//         (IVec2::new(0, -1), 4, "down"), // 下 (South)
//         (IVec2::new(-1, 0), 8, "left"), // 左 (West)
//     ];
//
//     let ivec2_arry = [
//         (IVec2::new(1, 1), "中"),
//         (IVec2::new(0, 1), "右"),
//         (IVec2::new(2, 1), "左"),
//         (IVec2::new(1, 0), "下"),
//         (IVec2::new(1, 2), "上"),
//         (IVec2::new(0, 2), "左上"),
//         (IVec2::new(0, 0), "左下"),
//         (IVec2::new(2, 2), "右上"),
//         (IVec2::new(2, 0), "右下"),
//     ];
//
//     let mut ivec2_set = HashSet::new();
//
//     for ivec2 in ivec2_arry {
//         ivec2_set.insert(ivec2.0);
//     }
//
//     for (ivec2, c) in ivec2_arry {
//         println!("{:?}:{}", ivec2, c);
//         let mut mask = 0;
//         for (offset, bit, ca) in mask_set {
//             let ivcffset = ivec2 + offset;
//             if ivec2_set.contains(&ivcffset) {
//                 // println!("ivcffset:{:?}", ivcffset);
//                 // println!();
//                 // println!("offset:{:?}, bit:{}, ca:{}", offset, bit, ca);
//                 mask |= bit;
//                 // println!("======");
//             }
//         }
//         println!("Mask: {}", mask);
//         println!();
//         println!("--------------------------------------------");
//         println!();
//     }
// }
//
// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// struct IVec2(i32, i32);
//
// impl IVec2 {
//     fn new(x: i32, y: i32) -> Self {
//         Self(x, y)
//     }
// }
//
// impl Add for IVec2 {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         let mut ivec = self;
//         ivec.0 += rhs.0;
//         ivec.1 += rhs.1;
//
//         ivec
//     }
// }
//
// impl Sub for IVec2 {
//     type Output = Self;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         let mut ivec = self;
//         ivec.0 -= rhs.0;
//         ivec.1 -= rhs.1;
//
//         ivec
//     }
// }

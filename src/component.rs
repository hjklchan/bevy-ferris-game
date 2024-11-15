use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct Enemy;

/// # FromPlayer
///
/// 用于标记在玩家的镭射实体上, 当敌人触碰到玩家的镭射实体,
/// 那么可以通过 [`Laser`] 组件和 [`FromPlayer`] 组件查询目标实体
/// 
#[derive(Component)]
pub struct FromPlayer;

/// # FromPlayer
///
/// 用于标记在敌人的镭射或者敌人自身实体上, 当玩家触碰到敌人前面所述的实体后,
/// 那么可以通过 [`Laser`] 组件和 [`FromEntity`] 组件查询目标实体
/// 
#[derive(Component)]
pub struct FromEnemy;

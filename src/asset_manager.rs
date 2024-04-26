use ggez::{audio, graphics, Context, GameResult};

pub struct Assets {
    pub bgm: audio::Source,
    pub player_laser_1: audio::Source,
    pub laser_1: audio::Source,
    pub special_atk: audio::Source,
    pub spread_shot_3: audio::Source,
    pub spread_shot_5: audio::Source,
    pub damage: audio::Source,
    pub background: graphics::Image,
    pub player_ship: graphics::Image,
    pub boss_ship: graphics::Image,
}

impl Assets {
    pub(crate) fn new(ctx: &mut Context) -> GameResult<Assets> {
        let bgm = audio::Source::new(ctx, "/Lost in Another World.mp3").expect(
            format!(
                "Failed to load bgm from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/Lost in Another World.mp3"
            )
            .as_str(),
        );
        let laser_1 = audio::Source::new(ctx, "/laser_1.flac").expect(
            format!(
                "Failed to load laser1 from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/laser1.flac"
            )
            .as_str(),
        );
        let player_laser_1 = audio::Source::new(ctx, "/player_laser_1.flac").expect(
            format!(
                "Failed to load player_laser1 from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/player_laser1.flac"
            )
            .as_str(),
        );
        let special_atk = audio::Source::new(ctx, "/special_atk.flac").expect(
            format!(
                "Failed to load special_atk from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/special_atk.flac"
            )
            .as_str(),
        );
        let spread_shot_3 = audio::Source::new(ctx, "/spread_shot_3.flac").expect(
            format!(
                "Failed to load spread_shot_3 from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/spread_shot_3.flac"
            )
            .as_str(),
        );
        let spread_shot_5 = audio::Source::new(ctx, "/spread_shot_5.flac").expect(
            format!(
                "Failed to load spread_shot_5 from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/spread_shot_5.flac"
            )
            .as_str(),
        );
        let damage = audio::Source::new(ctx, "/damage.flac").expect(
            format!(
                "Failed to load damage from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/damage.flac"
            )
            .as_str(),
        );
        let background = graphics::Image::from_path(ctx, "/background_1.tiff").expect(
            format!(
                "Failed to load background from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/background_1.tiff"
            )
            .as_str(),
        );
        let player_ship = graphics::Image::from_path(ctx, "/player_ship.tiff").expect(
            format!(
                "Failed to load player_ship from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/player_ship.tiff"
            )
            .as_str(),
        );
        let boss_ship = graphics::Image::from_path(ctx, "/boss_ship.tiff").expect(
            format!(
                "Failed to load boss_ship from path {:?} {:?}",
                ctx.fs.resources_dir(),
                "/boss_ship.tiff"
            )
            .as_str(),
        );
        Ok(Assets {
            bgm,
            player_laser_1,
            laser_1,
            special_atk,
            spread_shot_3,
            spread_shot_5,
            damage,
            background,
            player_ship,
            boss_ship,
        })
    }
}

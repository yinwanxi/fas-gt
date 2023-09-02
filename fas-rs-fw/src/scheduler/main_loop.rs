/* Copyright 2023 shadow3aaa@gitbub.com
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License. */
use std::time::Duration;

use log::{info, trace};
use surfaceflinger_hook_api::Connection;

use super::Scheduler;
use crate::{config::Config, error::Result, PerformanceController};

const BIG_JANK_REC_COUNT: u8 = 3;

impl<P: PerformanceController> Scheduler<P> {
    pub(super) fn main_loop(
        config: &mut Config,
        controller: &P,
        connection: &mut Connection,
        jank_level_max: Option<u32>,
    ) -> Result<()> {
        Self::init_load_default(connection, controller, config)?;

        let mut status = None;
        let mut big_jank_counter = 0;

        loop {
            let update_config = config.cur_game_fps();

            if status != update_config {
                status = update_config;
                if let Some((game, target_fps)) = &status {
                    info!("Loaded on game: {game}");
                    Self::init_load_game(*target_fps, connection, controller, config)?;
                } else {
                    Self::init_load_default(connection, controller, config)?;
                }

                continue;
            }

            let Ok(level) = connection.recv() else {
                continue;
            };
            let level = jank_level_max.map_or(*level, |max| level.min(max));

            trace!("Recv jank: {level:?}");

            if level >= 3 {
                big_jank_counter = BIG_JANK_REC_COUNT; // big jank
            } else if big_jank_counter > 0 {
                big_jank_counter -= 1;
                if level == 0 {
                    continue; // 等待BIG_JANK_REC_COUNT帧后才能降频
                }
            }

            controller.perf(level, config);
        }
    }

    fn init_load_game(
        target_fps: u32,
        connection: &Connection,
        controller: &P,
        config: &Config,
    ) -> Result<()> {
        connection.set_input(Some((target_fps, Duration::new(0, 0))))?;
        controller.init_game(config)
    }

    fn init_load_default(connection: &Connection, controller: &P, config: &Config) -> Result<()> {
        connection.set_input(None)?;
        controller.init_default(config)
    }
}

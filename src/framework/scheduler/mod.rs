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
mod binder;
mod looper;
mod topapp;

use std::time::Duration;

use super::{
    config::Config,
    error::{Error, Result},
    node::Node,
    Extension,
};
use crate::CpuCommon;

use self::binder::FasServer;
use looper::Looper;

#[derive(Debug, Clone, Copy)]
pub struct FasData {
    pub pid: i32,
    pub frametime: Duration,
}

pub struct Scheduler {
    controller: Option<CpuCommon>,
    config: Option<Config>,
}

impl Scheduler {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            controller: None,
            config: None,
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn config(mut self, c: Config) -> Self {
        self.config = Some(c);
        self
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn controller(mut self, c: CpuCommon) -> Self {
        self.controller = Some(c);
        self
    }

    pub fn start_run(self) -> Result<()> {
        let mut node = Node::init()?;
        let extension = Extension::init()?;
        let config = self.config.ok_or(Error::SchedulerMissing("Config"))?;

        let controller = self
            .controller
            .ok_or(Error::SchedulerMissing("Controller"))?;

        let rx = FasServer::run_server(&mut node, config.clone())?;

        Looper::new(rx, config, node, extension, controller).enter_loop()
    }
}

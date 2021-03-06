use derive_new::new;
use gym_rs::core::ActionReward;
use gym_rs::{core::Env, envs::classical_control::mountain_car::MountainCarEnv};
use lgp::extensions::reinforcement_learning::StateRewardPair;
use lgp::{
    core::{algorithm::GeneticAlgorithm, inputs::ValidInput, program::Program, registers::R32},
    extensions::reinforcement_learning::{
        ReinforcementLearningInput, ReinforcementLearningParameters, Reward,
    },
};
use serde::Serialize;

pub struct MountainCarLgp;

impl GeneticAlgorithm for MountainCarLgp {
    type O = Program<ReinforcementLearningParameters<MountainCarInput>>;
}

#[derive(Debug, Serialize, new, Clone)]
pub struct MountainCarInput {
    environment: MountainCarEnv,
}

impl ValidInput for MountainCarInput {
    const N_INPUT_REGISTERS: usize = 2;
    const N_ACTION_REGISTERS: usize = 3;

    fn flat(&self) -> Vec<R32> {
        let state = self.get_state();
        state
    }
}

impl ReinforcementLearningInput for MountainCarInput {
    fn init(&mut self) {
        self.environment.reset(Some(0), false, None);
    }

    fn act(&mut self, action: usize) -> StateRewardPair {
        let ActionReward { reward, done, .. } = self.environment.step(action);
        let reward_f32 = reward.into_inner() as f32;

        StateRewardPair {
            state: self.get_state(),
            reward: match done {
                true => Reward::Terminal(reward_f32),
                false => Reward::Continue(reward_f32),
            },
        }
    }

    fn get_state(&self) -> Vec<R32> {
        let state = &self.environment.state;
        [state.position, state.velocity]
            .map(|v| v.into_inner() as f32)
            .to_vec()
    }

    fn finish(&mut self) {
        self.environment.close();
    }

    fn reset(&mut self) {
        self.environment.reset(None, false, None);
    }
}

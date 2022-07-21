use derive_new::new;
use gym_rs::{core::Env, envs::classical_control::cartpole::CartPoleEnv};
use lgp::{
    core::{algorithm::GeneticAlgorithm, inputs::ValidInput, program::Program},
    extensions::reinforcement_learning::{
        ReinforcementLearningInput, ReinforcementLearningParameters, Reward, StateRewardPair,
    },
};
use ordered_float::OrderedFloat;
use serde::Serialize;

pub struct CartPoleLgp;

#[derive(Clone, Debug, Serialize, new)]
pub struct CartPoleInput {
    environment: CartPoleEnv,
}

impl ValidInput for CartPoleInput {
    const N_INPUT_REGISTERS: usize = 4;
    const N_ACTION_REGISTERS: usize = 2;

    fn flat(&self) -> Vec<lgp::core::registers::RegisterValue> {
        self.get_state()
    }
}

impl ReinforcementLearningInput for CartPoleInput {
    fn init(&mut self) {
        self.environment.reset(None, false, None);
    }

    fn act(&mut self, action: usize) -> StateRewardPair {
        let action_reward = self.environment.step(action);
        let reward = OrderedFloat(action_reward.reward.into_inner() as f32);

        StateRewardPair {
            state: self.get_state(),
            reward: match action_reward.done {
                true => Reward::Terminal(reward),
                false => Reward::Continue(reward),
            },
        }
    }

    fn reset(&mut self) {
        self.environment.reset(None, false, None);
    }

    fn get_state(&self) -> Vec<lgp::core::registers::RegisterValue> {
        let state = self.environment.state;
        let state_vec: Vec<_> = state.into();

        state_vec
            .iter()
            .map(move |s| OrderedFloat(*s as f32))
            .collect()
    }

    fn finish(&mut self) {
        self.environment.close()
    }
}

impl GeneticAlgorithm for CartPoleLgp {
    type O = Program<ReinforcementLearningParameters<CartPoleInput>>;
}

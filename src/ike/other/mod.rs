mod get_up_attack;
mod ledge_attack;
mod final_smash;

pub fn install(agent: &mut smashline::Agent) {
    get_up_attack::install(agent);
    ledge_attack::install(agent);
    final_smash::install(agent);
}
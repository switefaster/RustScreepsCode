use stdweb::js;

fn main() {
    js! {
        var game_loop = @{game_loop};
        module.exports.loop = function() {
            try {
                game_loop();
            } catch (error) {
                console.log("Error:" + err);
                if(error.stack) {
                    console.log("Stack Trace:" + err.stack);
                }
                console.log("Restarting virtual machine...");
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn game_loop() {
    for creep in screeps::game::creeps::values() {
        creep.say("All hail WASM!", false);
    }
}
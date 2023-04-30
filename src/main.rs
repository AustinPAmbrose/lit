use clap::Parser;

/// Control your home lighting from the terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Set the brightness from 0% to 100%
    brightness: Option<u8>,

    /// Only affect the bedroom lights
    #[arg(short, long, default_value_t = false)]
    bedroom: bool,

    // /// Only affect the kitchen lights
    // #[arg(short, long, default_value_t = false)]
    // kitchen: bool,
    //
    // /// Only affect the poker lights
    // #[arg(short, long, default_value_t = false)]
    // poker: bool,
    //
    // /// Only affect the living room lights
    // #[arg(short, long, default_value_t = false)]
    // livingroom: bool,

    /// View the current state of all lights
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    
    // Handle all CLI parsing
    let args = Args::parse();

    // Create an object for the Hue Bridge
    let key  = "Xg-MbGdlRZlXEUqeaEQSNVai2n91W4qPSmrBWrLR";
    let bridge = match hueclient::Bridge::discover() {
        Some(bridge) => bridge.with_user(key),
        None         => {
            println!{"Failed to find a Hue Bridge"}
            println!{"Make sure you're connected to the network and the bridge is powered on..."}; 
            return;
        }
    };
    

    if args.debug {
        for light in &bridge.get_all_lights().unwrap() {
            let light = &light.light;
            println!("name: {}, state: {:?}", light.name, light.state);
        }
    }

    // If the user wants to change the brightness, do it
    if let Some(mut brightness) = args.brightness {

        // Tell the light to turn off if brightness == 0
        // Otherwise, tell it to turn on to brightness
        let mut cmd = hueclient::CommandLight::default();
        if brightness == 0 {
            cmd.on = Some(false);
        } else {
            brightness = (brightness as f32 * 2.54) as u8;
            cmd.on = Some(true);
            cmd.bri = Some(brightness);
        }

        // Update the lights
        for light in &bridge.get_all_lights().unwrap() {
            if light.light.name.contains("Bedroom") {
                if args.bedroom == true {
                    bridge.set_light_state(light.id, &cmd).unwrap();
                }
            } else {
                if args.bedroom == false {
                    bridge.set_light_state(light.id, &cmd).unwrap();
                }
            }
        }
    }
}
use clap::Parser;
use std::path::PathBuf;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Set the brightness from 0% to 100%
    brightness: Option<u8>,

    /// Register a new computer for the first time
    #[arg(short, long, default_value_t = false)]
    register: bool,

    /// Only affect the bedroom lights
    #[arg(short, long, default_value_t = false)]
    bedroom: bool,

    /// Only affect the kitchen lights
    #[arg(short, long, default_value_t = false)]
    kitchen: bool,

    /// Only affect the poker lights
    #[arg(short, long, default_value_t = false)]
    poker: bool,

    /// Only affect the living room lights
    #[arg(short, long, default_value_t = false)]
    livingroom: bool,
}

fn main() {
    
    let args = Args::parse();
    // Reformat brightness from 0 to 100
    let user = get_computer_name();
    let key  = "Xg-MbGdlRZlXEUqeaEQSNVai2n91W4qPSmrBWrLR";
    let bridge = match hueclient::Bridge::discover() {
        Some(bridge) => bridge,
        None         => {
            println!{"Failed to find a Hue Bridge"}
            println!{"Make sure you're connected to the network and the bridge is powered on..."}; 
            return;
        }
    };
    
    // Register a new user and exit if the user passes the --register flag
    if args.register == true {
        match bridge.register_user(&user) {
            Ok(bridge)  => {
                println!("Successfully registered user: {}, with Hue Bridge :)", &user);
                println!("New key is: {}", &bridge.username);
                save_config(&bridge.username);
            },
            Err(_) => {
                println!("Failed to register with the Hue Bridge...");
                println!("Make sure you press the button on the Bridge before");
                println!("running this command...");
                return;
            }
        };
        
        return;
    }

    // Try to load the username from the config file
    // let username = match load_config() {
    //     Ok(username) => username,
    //     Err(msg)     => {
    //         println!("{msg}");
    //         return;
    //     }
    // };

    
    // Connect to the bridge with the username
    let bridge = bridge.with_user(key);
    

    // for light in &bridge.get_all_lights().unwrap() {
    //     println!("{:?}", light);
    // }

    // If the user wants to change the brightness, do it
    if let Some(mut brightness) = args.brightness {
          brightness = (brightness as f32 * 2.54) as u8;
        let cmd = hueclient::CommandLight::default().with_bri(brightness);
        for light in &bridge.get_all_lights().unwrap() {
            if light.light.name.contains("Bedroom 1") {
                bridge.set_light_state(light.id, &cmd).unwrap();
            }
        }
    }
}


fn get_computer_name() -> String {
    let os_name = hostname::get()
                    .expect("Failed to get name of computer");
    let str_name = os_name.to_str()
                    .expect("Failed to convert computer name to string");
    return String::from(str_name);
}

fn save_config(username : &str) {

    // Get the path to the config file
    let config_path = get_config_path();

    // Write to it
    let write_result = fs::write(&config_path, username);

    // Report any errors
    if let Err(_) = write_result {
        println!("ERROR: Failed to write config file > {}", config_path.to_str().unwrap() )    
    }
}

fn load_config() -> Result<String, String> {

    // Get the path to the config file
    let config_path = get_config_path();

    // Throw a nice error if it doesn't exist
    if config_path.exists() == false {
        return Err(format!("ERROR: config file {} does not exist.
                           \rTry Registering your computer with `lit --register`"
                           ,config_path.to_str().unwrap()));
    }

    // Read it if it exists
    let config_contents = fs::read(&config_path);

    //Check for a read error
    let config_username = match config_contents {
        Ok(contents) => contents,
        Err(_) => return Err(format!("ERROR: Error reading file {}", config_path.to_str().unwrap())),
    };

    // If there's nothing in it, return an error
    if config_username.len() == 0 {
        return Err(format!("ERROR: config file {} exists, but is empty", config_path.to_str().unwrap()));
    } else {
        return Ok(config_username[0].to_string());
    }
}

fn get_config_path() -> PathBuf {
    // Make sure a config file exists, and that we can pull a username from it
    let current_exe = std::env::current_exe().unwrap();
    let mut current_dir = current_exe.parent().unwrap().to_owned();
    current_dir.push("lit.config");
    current_dir
}


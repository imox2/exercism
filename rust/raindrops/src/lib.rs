use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
	let mut rain_sound_options = HashMap::new();
	rain_sound_options.insert(3, "Pling");
	rain_sound_options.insert(5, "Plang");
	rain_sound_options.insert(7, "Plong");

    let mut rain_sound: String = "".to_owned();
    let factors = [3, 5, 7];

    // loop over list of factors to find
    for factor in factors.iter() {
    	if n % factor == 0 {
    		// get the sound value from hashMap
    		 if let Some(&sound) = rain_sound_options.get(&factor) {
    		 	rain_sound.push_str(sound) 
    		 }
    	}
    }
    if rain_sound.trim().is_empty() {
    	rain_sound.push_str(&n.to_string());
    }

    rain_sound
}

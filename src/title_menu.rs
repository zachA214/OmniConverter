use rand::Rng;

//contains all combinations for ascii text

struct TitleCardOptions<'a> {
    block_one: [&'a str; 4],
    block_two: [&'a str; 4],
    block_three: [&'a str; 4],
    block_four: [&'a str; 4],
    block_five: [&'a str; 4],
}

pub fn display_title() {
    let text_omni = TitleCardOptions {
        block_one: [" ____  _     _     _                                 ", " ,-----.                   ,--.                                    ",
        "   ____                  _                       ", " ______  __    __  __   __  __                                             "], 
        block_two: ["/  _ \\/ \\__// \\  // \\                                ", "'  .-.  ',--,--,--.,--,--, `--'                                    ",
        "  / __ \\____ ___  ____  (_)                      ", "/\\  __ \\/\\ \"-./  \\/\\ \"-.\\ \\/\\ \\                                            "],
        block_three: ["| / \\|| |\\/|| |\\ || |                                ", "|  | |  ||        ||      \\,--.                                    ",
        " / / / / __ `__ \\/ __ \\/ /                       ", "\\ \\ \\/\\ \\ \\ \\-./\\ \\ \\ \\-.  \\ \\ \\                                           "],
        block_four: ["| \\_/|| |  || | \\|| |                                ", "'  '-'  '|  |  |  ||  ||  ||  |                                    ",
        "/ /_/ / / / / / / / / / /                        ", " \\ \\_____\\ \\_\\ \\ \\_\\ \\_\\\\\"\\_\\ \\_\\                                          "],
        block_five: ["\\____/\\_/  \\\\_/  \\\\_/                                ", " `-----' `--`--`--'`--''--'`--'                                    ",
        "\\____/_/ /_/ /_/_/ /_/_/                         ", "  \\/_____/\\/_/  \\/_/\\/_/ \\/_/\\/_/                                          "],
    };
    let text_converter = TitleCardOptions {
        block_one: [" ____  ____  _     _     _________  _____  _________ ", " ,-----.                                      ,--.                 ",
        "                                    __           ", " ______________  __   __  __   ________  ______  ______  ______  ______    "], 
        block_two: ["/   _\\/  _ \\/ \\  // \\ |\\/  __/  __\\/__ __\\/  __/  __\\", "'  .--./ ,---. ,--,--,--.  ,--.,---. ,--.--.,-'  '-. ,---. ,--.--. ",
        "   _____ ___  ____ _   _____  _____/ /____  _____", "/\\  ___\\/\\  __ \\/\\ \"-.\\ \\/\\ \\ / /\\  ___\\/\\  == \\/\\__  _\\/\\  ___\\/\\  == \\   "],
        block_three: ["|  /  | / \\|| |\\ || | //|  \\ |  \\/|  / \\  |  \\ |  \\/|", "|  |    | .-. ||      \\  `'  /| .-. :|  .--''-.  .-'| .-. :|  .--' ",
        " / /   / __ \\/ __ \\ | / / _ \\/ ___/ __/ _ \\/ ___/", "\\ \\ \\___\\ \\ \\/\\ \\ \\ \\-.  \\ \\ \\'/\\ \\  __\\\\ \\  __<\\/_/\\ \\/\\ \\  __\\\\ \\  __<   "],
        block_four: ["|  \\__| \\_/|| | \\|| \\// |  /_|    /  | |  |  /_|    /", "'  '--'\\' '-' '|  ||  |\\    / \\   --.|  |     |  |  \\   --.|  |    ",
        "/ /___/ /_/ / / / / |/ /  __/ /  / /_/  __/ /    ", " \\ \\_____\\ \\_____\\ \\_\\\\\"\\_\\ \\__| \\ \\_____\\ \\_\\ \\_\\ \\ \\_\\ \\ \\_____\\ \\_\\ \\_\\ "],
        block_five: ["\\____/\\____/\\_/  \\\\__/  \\____\\_/\\_\\  \\_/  \\____\\_/\\_\\", " `-----' `---' `--''--' `--'   `----'`--'     `--'   `----'`--'    ",
        "\\____/\\____/_/ /_/|___/\\___/_/   \\__/\\___/_/     ", "  \\/_____/\\/_____/\\/_/ \\/_/\\/_/   \\/_____/\\/_/ /_/  \\/_/  \\/_____/\\/_/ /_/ "],
    };

    let mut rng = rand::rng();

    let random_index = rng.random_range(0..4);

    //print text from text_omni
    println!("{}", text_omni.block_one[random_index]);
    println!("{}", text_omni.block_two[random_index]);
    println!("{}", text_omni.block_three[random_index]);
    println!("{}", text_omni.block_four[random_index]);
    println!("{}", text_omni.block_five[random_index]);
    //print text from text_converter
    println!("{}", text_converter.block_one[random_index]);
    println!("{}", text_converter.block_two[random_index]);
    println!("{}", text_converter.block_three[random_index]);
    println!("{}", text_converter.block_four[random_index]);
    println!("{}", text_converter.block_five[random_index]);
    

}
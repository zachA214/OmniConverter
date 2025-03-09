use rand::Rng;

//contains all combinations for ascii text

struct TitleCardOptions<'a> {
    block_one: [&'a str; 2],
    block_two: [&'a str; 2],
    block_three: [&'a str; 2],
    block_four: [&'a str; 2],
    block_five: [&'a str; 2],
}

pub fn display_title() {
    let text_omni = TitleCardOptions {
        block_one: [" ____  _     _     _                                 ", " ,-----.                   ,--.                                    "], 
        block_two: ["/  _ \\/ \\__// \\  // \\                                ", "'  .-.  ',--,--,--.,--,--, `--'                                    "],
        block_three: ["| / \\|| |\\/|| |\\ || |                                ", "|  | |  ||        ||      \\,--.                                    "],
        block_four: ["| \\_/|| |  || | \\|| |                                ", "'  '-'  '|  |  |  ||  ||  ||  |                                    "],
        block_five: ["\\____/\\_/  \\\\_/  \\\\_/                                ", " `-----' `--`--`--'`--''--'`--'                                    "],
    };
    let text_converter = TitleCardOptions {
        block_one: [" ____  ____  _     _     _________  _____  _________ ", "one"], 
        block_two: ["/   _\\/  _ \\/ \\  // \\ |\\/  __/  __\\/__ __\\/  __/  __\\", "two"],
        block_three: ["|  /  | / \\|| |\\ || | //|  \\ |  \\/|  / \\  |  \\ |  \\/|", "three"],
        block_four: ["|  \\__| \\_/|| | \\|| \\// |  /_|    /  | |  |  /_|    /", "four"],
        block_five: ["\\____/\\____/\\_/  \\\\__/  \\____\\_/\\_\\  \\_/  \\____\\_/\\_\\", "five"],
    };

    let mut rng = rand::rng();

    let random_index = rng.random_range(0..2);

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
pub static ONE: &str = include_str!("../example/1");
pub static TWO: &str = include_str!("../example/2");
pub static THREE: &str = include_str!("../example/3");
pub static FOUR: &str = include_str!("../example/4");
pub static FIVE: &str = include_str!("../example/5");
pub static SIX: &str = include_str!("../example/6");
pub static SEVEN: &str = include_str!("../example/7");
pub static EIGHT: &str = include_str!("../example/8");
pub static NINE: &str = include_str!("../example/9");
pub static TEN: &str = include_str!("../example/10");
pub static ELEVEN: &str = include_str!("../example/11");
pub static TWELVE: &str = include_str!("../example/12");
pub static THIRTEEN: &str = include_str!("../example/13");
pub static FOURTEEN: &str = include_str!("../example/14");
pub static FIFTEEN: &str = include_str!("../example/15");
pub static SIXTEEN: &str = include_str!("../example/16");
pub static SEVENTEEN: &str = include_str!("../example/17");
pub static EIGHTEEN: &str = include_str!("../example/18");
pub static NINETEEN: &str = include_str!("../example/19");
pub static TWENTY: &str = include_str!("../example/20");
pub static TWENTY_ONE: &str = include_str!("../example/21");
pub static TWENTY_TWO: &str = include_str!("../example/22");
pub static TWENTY_THREE: &str = include_str!("../example/23");
pub static TWENTY_FOUR: &str = include_str!("../example/24");
pub static TWENTY_FIVE: &str = include_str!("../example/25");
//noinspection ALL
fn get(index: usize) -> &'static str {
    match index {
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        10 => TEN,
        11 => ELEVEN,
        12 => TWELVE,
        13 => THIRTEEN,
        14 => FOURTEEN,
        15 => FIFTEEN,
        16 => SIXTEEN,
        17 => SEVENTEEN,
        18 => EIGHTEEN,
        19 => NINETEEN,
        20 => TWENTY,
        21 => TWENTY_ONE,
        22 => TWENTY_TWO,
        23 => TWENTY_THREE,
        24 => TWENTY_FOUR,
        25 => TWENTY_FIVE,
        _ => unreachable!(),
    }
}

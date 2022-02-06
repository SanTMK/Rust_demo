
struct Piece{
    piece_name: String,
    is_white: bool,
    start_clear_sqr: bool
}
impl Piece {
    //Construct a new piece
    fn new(name: &str, white: bool, clear: bool) -> Piece {
        Piece {
            piece_name: name.to_string(),
            is_white: white,
            start_clear_sqr: clear,
        }
    }
    //Get starting square
    fn get_start_sqr(&self) -> (char, u8) {
        let mut file: char = 'z';
        let mut rank: u8 = 0;
        // Asign piece rank
        if self.is_white {
            rank = 1;
        } else {
            rank = 8;
        }
        // Asign piece file
        if self.start_clear_sqr {
            file = 'f';
        } else {
            file = 'c';
        }

        (file, rank)
        
    }
}


pub fn run() {
    let bish_1 = Piece::new("bishop", true, true);
    let strt: (char, u8) = bish_1.get_start_sqr();

    println!("this piece is a {} that starts in {:?}", bish_1.piece_name, strt);
}
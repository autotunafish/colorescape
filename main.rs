
//prints all 255^2 foreground and background combinations.
//THIS PRINTS A LARGE OUTPUT! (Less if zoomed in)

fn main() {
	let mut x = 0;
	let mut y = 0;
	loop {
		print!("\x1b[48;5;{}m\x1b[38;5;{}m {}-{}\x1b[0m", &x, &y, &x, &y);
		if x >= 255 {
			y = y + 1;
			x = 0;
		}
		if y >= 256 {
		break
		}
		x = x + 1;
		continue
    }
}

pub fn finde_highest_serial_no() -> [isize; 14]{
	for i0 in (1..=9).rev(){
	for i1 in (1..=9).rev(){
	for i2 in (1..=9).rev(){
	for i3 in (1..=9).rev(){
	for i4 in (1..=9).rev(){
	for i5 in (1..=9).rev(){
	for i6 in (1..=9).rev(){
	for i7 in (1..=9).rev(){
	for i8 in (1..=9).rev(){
	for i9 in (1..=9).rev(){
	for i10 in (1..=9).rev(){
	for i11 in (1..=9).rev(){
	for i12 in (1..=9).rev(){
	for i13 in (1..=9).rev(){
	let no = [i0,i1,i2,i3,i4,i5,i6,i7,i8,i9,i10,i11,i12,i13];
	if calc_serial_no(no){
		return no;
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
panic!();
	[0; 14]
}
pub fn finde_lowest_serial_no() -> [isize; 14]{
	for i0 in (1..=9){
	for i1 in (1..=9){
	for i2 in (1..=9){
	for i3 in (1..=9){
	for i4 in (1..=9){
	for i5 in (1..=9){
	for i6 in (1..=9){
	for i7 in (1..=9){
	for i8 in (1..=9){
	for i9 in (1..=9){
	for i10 in (1..=9){
	for i11 in (1..=9){
	for i12 in (1..=9){
	for i13 in (1..=9){
	let no = [i0,i1,i2,i3,i4,i5,i6,i7,i8,i9,i10,i11,i12,i13];
	if calc_serial_no(no){
		return no;
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
	}
panic!();
	[0; 14]
}
fn calc_serial_no(no: [isize; 14]) -> bool {
   let mut w: isize = 0;
   let mut x: isize = 0;
   let mut y: isize = 0;
   let mut z: isize = 0;
  w = no[0];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 1;
  x = x + 11;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 1;
  y = y * x;
  z = z + y;
  w = no[1];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 1;
  x = x + 10;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 10;
  y = y * x;
  z = z + y;
  w = no[2];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 1;
  x = x + 13;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 2;
  y = y * x;
  z = z + y;
  w = no[3];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 26;
  x = x + -10;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 5;
  y = y * x;
  z = z + y;
  w = no[4];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 1;
  x = x + 11;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 6;
  y = y * x;
  z = z + y;
  w = no[5];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 1;
  x = x + 11;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 0;
  y = y * x;
  z = z + y;
  w = no[6];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 1;
  x = x + 12;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 16;
  y = y * x;
  z = z + y;
  w = no[7];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 26;
  x = x + -11;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 12;
  y = y * x;
  z = z + y;
  w = no[8];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 26;
  x = x + -7;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 15;
  y = y * x;
  z = z + y;
  w = no[9];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 1;
  x = x + 13;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 7;
  y = y * x;
  z = z + y;
  w = no[10];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 26;
  x = x + -13;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 6;
  y = y * x;
  z = z + y;
  w = no[11];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 26;
  x = x + 0;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 5;
  y = y * x;
  z = z + y;
  w = no[12];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 26;
  x = x + -11;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 6;
  y = y * x;
  z = z + y;
  w = no[13];
  x = x * 0;
  x = x + z;
  x = x % 26;
  z = z / 26;
  x = x + 0;
  x = (x == w) as isize;
  x = (x == 0) as isize;
  y = y * 0;
  y = y + 25;
  y = y * x;
  y = y + 1;
  z = z * y;
  y = y * 0;
  y = y + w;
  y = y + 15;
  y = y * x;
  z = z + y;
  w == 0
}

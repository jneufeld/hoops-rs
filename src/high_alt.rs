pub fn largest_altitude(gain: Vec<i32>) -> i32 {
   let mut cur = 0;
   let mut max = cur;

   for step in gain {
	   cur += step;
	   if cur > max {
		   max = cur;
	   }
   }

   max
}

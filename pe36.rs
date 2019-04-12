// Project Euler Problem 36
// Author: ubdussamad@gmail.com
// Double Base Pallindromes

 fn main() {
     let mut sum:i64 = 0;
     let mut i:i64 = 1;
     loop {
         if (format!("{:b}", i)==format!("{:b}", i).chars().rev().collect::<String>()) {
             if is_palindrome(i) {
                 sum+=i;
                 println!("{}",i);
                
             }
         }
         if (i>1000000) {break;}
         i+=1;
     }

     println!("Sum is {} " , sum ) ;
    

     return;
 }

    
pub fn is_palindrome( mut x : i64 )  -> bool  {
    let xcpy:i64 = x;
    let mut revint:i64 = 0;
    loop {
        if x==0 {
            break;
        }
        let rem = x%10;
        revint = revint*10 + rem;
        x /= 10;
        //intln!("x is {}" , x);
    }
    //intln!("xcpy {} || revint {}",xcpy , revint);
    return xcpy==revint;
}

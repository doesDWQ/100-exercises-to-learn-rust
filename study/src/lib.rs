pub fn get_long<'a,'b>(left: &'a u64, right: &'b u64) -> &'a u64 {
    left = left + right;
    &left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x1:u64 = 1;
        let t:&u64;
        {
            let x2:u64 = 2; 
            t = get_long(&x1, &x2);  
        }
        println!("{:?}", t)
    }
}

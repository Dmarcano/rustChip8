
// There will be a CPU that is responsible for executing 

pub mod chip8; 


// the keyboard and display will be Traits that take in and provide the CPU informaation

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

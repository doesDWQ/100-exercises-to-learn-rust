// TODO: Add `anyhow` as a dependency of this project.
//  Don't touch this import!

// When you import a type (`Error`) from a dependency, the import path must start
// with the crate name (`anyhow`, in this case).

pub mod anyhow {
    pub enum Error{
        Unknown,
        Ok,
    }
}

use anyhow::Error;

#[cfg(test)]
mod tests {
    

    #[test]
    fn a() {
        println!("{:?}", Error::Unknown);
    }
}
// TODO: Add `anyhow` as a dependency of this project.
//  Don't touch this import!

// When you import a type (`Error`) from a dependency, the import path must start
// with the crate name (`anyhow`, in this case).


use anyhow::Error;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = anyhow::add(2, 2);
        assert_eq!(result, 4);
    }
}

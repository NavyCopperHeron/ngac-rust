pub(crate) mod dag;
pub mod epp;
pub mod pap;
pub mod pdp;
pub mod pep;
pub mod pip;
pub mod prp;
pub mod policy;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

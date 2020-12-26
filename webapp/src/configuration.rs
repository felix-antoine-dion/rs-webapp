//!
//! L'ojectif est d'implémenter le [Options pattern](https://docs.microsoft.com/en-us/aspnet/core/fundamentals/configuration/options?view=aspnetcore-5.0)
//! en utilisant comme source de données les variables d'environnement.
//! 
//! 
mod Configuration {
    
    use std::env;

    pub fn get_options<T>() -> &T {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn happy_path() {
        struct Happy {
            TEST: &str;
            NUMBER: i32;
        }

        let result = Configuration::get_options::<Happy>();

        assert_eq!(true, true)
    }

    #[test]
    fn partial_info() {
        assert_eq!(true, true)
    }

    #[test]
    fn default_info() {
        assert_eq!(true, true)
    }
    
    #[test]
    fn missing_required() {
        assert_eq!(true, true)
    }

    #[test]
    fn convert_to_int() {
        assert_eq!(true, true)
    }

    #[test]
    fn convert_to_bool() {
        assert_eq!(true, true)
    }

    #[test]
    fn convert_to_wrong_type() {
        assert_eq!(true, true)
    }
}
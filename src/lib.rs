

pub mod postgres;
pub mod ui;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // This is a demo for implementing the ui::DisplayUI trait
    struct DemoThingy {
        id: i32,
        name: String,
    }

    impl ui::DisplayUI for DemoThingy {
        fn name(&self) -> String {
            self.name.to_string()
        }
        fn pk(&self) -> ui::PK {
            ui::PK::Int32(self.id)
        }
        fn data_type() -> &'static str {
            "demo_thingy"
        }
    }
}

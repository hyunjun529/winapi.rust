pub mod game {
    pub fn init() {
        println!("init");
    }
    
    pub fn update () {
        println!("update");
    }

    pub fn smile () -> (f64, f64) {
        return (100.0, 100.0);
    }
}
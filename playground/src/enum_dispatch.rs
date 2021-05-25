
pub trait Item {
    fn info(&self) -> String;
}

#[derive(Debug)]
pub struct ElectronicItem{
    name: String
}

impl Item for ElectronicItem {
    fn info(&self) -> String {
        format!("Electronic Item : {}", self.name)
    }
}

#[derive(Debug)]
pub struct GroceryItem {
    name: String
}

impl Item for GroceryItem {
    fn info(&self) -> String {
        format!("GroceryItem Item : {}", self.name)
    }
}

#[derive(Debug)]
pub enum Product {
    ElectronicItem(ElectronicItem),
    GroceryItem(GroceryItem),
}

impl Item for Product {
    fn info(&self) -> String{
        match self {
            Product::ElectronicItem(ei) => ei.info(),
            Product::GroceryItem(gi) => gi.info(),
        }
    }
}

pub fn enum_test(){
    let prods = vec![Product::ElectronicItem(ElectronicItem{name:format!("TV")}),
    Product::GroceryItem(GroceryItem{name:format!("Rice")})    
    ];

    process(&prods);
    process2(&prods);
    process3(&prods);    
    
}


pub fn process(items:  &Vec<impl Item>) {
    for item in items.iter() {
        println!("Info item : {}", item.info())
    }
}

pub fn process2<T>(items: &Vec<T>)
where T: Item {
    for item in items.iter() {
        println!("Info item : {}", item.info())
    }
}

pub fn process3(items:  &Vec<Product>) {
    for item in items.iter() {
        println!("Info item : {}", item.info())
    }
}


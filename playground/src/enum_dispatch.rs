
trait Item {
    fn info(&self) -> String;
}

#[derive(Debug)]
struct ElectronicItem{
    name: String
}

impl Item for ElectronicItem {
    fn info(&self) -> String {
        format!("Electronic Item : {}", self.name)
    }
}

#[derive(Debug)]
struct GroceryItem {
    name: String
}

impl Item for GroceryItem {
    fn info(&self) -> String {
        format!("GroceryItem Item : {}", self.name)
    }
}

#[derive(Debug)]
enum Product {
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
    let prods = [Product::ElectronicItem(ElectronicItem{name:format!("TV")}),
    Product::GroceryItem(GroceryItem{name:format!("Rice")})    
    ];
    
    for p in prods.iter() {
        println!("Info : {}", p.info())
    }
}
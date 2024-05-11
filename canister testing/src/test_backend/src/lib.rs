use std::{cell::RefCell, fmt::{Display, Formatter, Result}};


use ic_cdk::storage;

#[derive(Debug,Clone)]
struct LabeledImage {
    data: String,
    class: String,
}

#[derive(Debug)]
struct Dataset {
    name: String,
    imgs: Vec<LabeledImage>,
    classes: Vec<String>,
}


impl Display for Dataset {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Dataset Name: {}\nImages: {}\nClasses: {}",
            self.name,
            self.imgs.iter().map(|img| format!("{}: {}", img.data, img.class)).collect::<Vec<String>>().join(", "),
            self.classes.join(", ")
        )
    }
}


impl Dataset {
    pub fn new(name: String) -> Self {
        Dataset {
            name,
            imgs: Vec::new(),
            classes: Vec::new(),
        }
    }
    pub fn set_class(&mut self, class_name: String) {
        self.classes.push(class_name);
    }
    pub fn add_image(&mut self, img_base64: String, class: String) {
        let img = LabeledImage {
            data: img_base64,
            class: class,
        };
        // self.imgs.push(img.clone());
        let mut imgs = &mut self.imgs;
         storage::stable_save("my_variable", imgs);
        ic_cdk::print(format!("{:?}",self.imgs));
    }
}

thread_local! {
    static DATASET: RefCell<Dataset> = RefCell::new(Dataset::new("default".to_string()));
}

#[ic_cdk::u]
fn store_image(images: String, class: String) -> String{
    DATASET.with(|dataset| {
        dataset.borrow_mut().add_image(images.clone(), class.clone());
    });
    format!("Added {} {}",images,class)

}

#[ic_cdk::query]
fn add_class_to_dataset(class_name: String) {
    DATASET.with(|dataset| {
        dataset.borrow_mut().set_class(class_name);
    });
}

#[ic_cdk::query]
fn print_all_dataset() -> String {
    DATASET.with(|dataset| {
        let dataset = dataset.borrow();
        let mut dataset_str = format!("Dataset Name: {}\n", dataset.name);
        ic_cdk::print("Hello");
        for img in &dataset.imgs {
            ic_cdk::print(format!("{}",img.data));
            dataset_str.push_str(&format!("Image: {},: Class: {}\n", img.data, img.class));
        }
        for class in &dataset.classes {
            dataset_str.push_str(&format!("Class: {}\n", class));
        }
        dataset_str
    })
}
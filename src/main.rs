#[derive(Debug)]
struct Student{
    name:String,
    grade:char,
    age:u32,
    percentage:f64,
}

impl Student{
    fn data()->Student{
            Student{
            name:String::from("Talha"),
            grade:'A',
            age:25,
            percentage:76.01,
        }
    }
        fn data_1()->Student{
            Student{
            name:String::from("Omar"),
            grade:'B',
            age:23,
            percentage:69.01,
        }
    }
    fn data_2()->Student{
            Student{
            name:String::from("Haris"),
            grade:'A',
            age:26,
            percentage:74.5,
        }
    }
    fn Percent(&self){
        println!("The percentage of {} is {}",self.name,self.percentage);
    }
}
fn main(){
    let Student_0=Student::data();
    println!("{:#?}",Student_0);
    let Student_1=Student::data_1();
    println!("{:#?}",Student_1);
    let Student_2=Student::data_2();
    println!("{:#?}",Student_2);
    Student_0.Percent();
    Student_1.Percent();
    Student_2.Percent();
}


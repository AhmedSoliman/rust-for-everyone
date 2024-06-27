#[derive(Debug)]
enum Either<'a, 'b> {
    This(&'a str),
    That(&'b str),
}

#[derive(Clone)]
struct Employee {
    name: String,
    age: u32,
}

fn get_name<'a>(employee_1: &'a Employee) -> &'a str {
    &employee_1.name
}

fn who_is_older<'a, 'b>(employee_1: &'a Employee, employee_2: &'b Employee) -> Either<'a, 'b> {
    if employee_1.age > employee_2.age {
        Either::This(&employee_1.name)
    } else {
        Either::That(&employee_2.name)
    }
}
// fn who_is_older<'a>(employee_1: &'a Employee, employee_2: &'a Employee) -> &'a str {
//     if employee_1.age > employee_2.age {
//         &employee_1.name
//     } else {
//         &employee_2.name
//     }
// }

fn main() {
    let mahmoud = Employee {
        name: "Mahmoud".to_string(),
        age: 22,
    };

    let name = get_name(&mahmoud);

    println!("My name is {}", name);

    let ahmed = Employee {
        name: "Ahmed".to_string(),
        age: 20,
    };

    println!("The older is {:?}", who_is_older(&mahmoud, &ahmed));
}

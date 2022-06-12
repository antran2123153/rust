fn main() {
    let base_string = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let mut search_string = String::new();
    println!("Enter search string: ");
    std::io::stdin().read_line(&mut search_string).unwrap();

    let len_base = base_string.chars().count();
    let len_search = search_string.chars().count();

    let mut i = 0;
    let mut counter = 0;

    if len_search != 0 && len_search <= len_base {
        loop {
            if i >= len_base - len_search + 1 {
                break;
            }

            let mut j = 0;

            if base_string.chars().nth(i).unwrap() == search_string.chars().nth(j).unwrap() {
                let mut k = i + 1;
                j = j + 1;
    
                loop {
                    if j >= len_search - 1{
                        counter = counter + 1;
                        break;
                    }

                    if base_string.chars().nth(k).unwrap() != search_string.chars().nth(j).unwrap() {
                        break;
                    }

                    j = j + 1;
                    k = k + 1;
                }
            }
    
            i = i + 1;
        }
    }

    println!("Sub string appears {} times in base string", counter);
}
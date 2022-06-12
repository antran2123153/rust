fn main() {
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [6,8,10];

    let mut i = 0;

    let mut check = false;

    if sub_arr.len() == 0 {
        check = true;
    }

    if sub_arr.len() <= org_arr.len() {
        loop {
            if i >= org_arr.len() - sub_arr.len() + 1 {
                break;
            }
    
            let mut j = 0;
    
            if org_arr[i] == sub_arr[j] && !check {
                let mut k = i + 1;
                j = j + 1;
    
                loop {
                    if j >= sub_arr.len() {
                        check = true;
                        break;
                    }
    
                    if org_arr[k] != sub_arr[j] {
                        break;
                    }
    
                    j = j + 1;
                    k = k + 1;
                }
            }
    
            i = i + 1;
        }
    }

    if check {
        println!("The sub array is a subarray of the org array")
    } else {
        println!("The sub array is not a subarray of the org array")
    }
}
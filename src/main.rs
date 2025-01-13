use std::io::{self, Write};
use std::process::exit;
use std::{thread, time};



fn create_fibonacci(n: i32) -> Option<Vec<i128>> 
{
    if n < 0 
    {
        return None;  // Nếu n là số âm, trả về None.
    } 
    else if n == 0 
    {
        return Some(vec![0]);  // Trường hợp n = 0, chỉ có số 0.
    } 
    else if n == 1
    {
        return Some(vec![0, 1]);  // Trường hợp n = 1, dãy Fibonacci có 2 phần tử: [0, 1].
    } 
    else 
    {
        let mut ans = vec![0, 1];
        for _ in 2..=n 
        {
            ans.push(ans[ans.len() - 1] + ans[ans.len() - 2]);  // Tính các phần tử Fibonacci tiếp theo.
        }
        return Some(ans);  // Trả về dãy Fibonacci.
    }
}




// Cho phép nhập như trong Python
fn input(message: &str) -> String 
{
    _input(message, false)  // Gọi hàm phụ với giá trị mặc định true cho endl
}

// Cho phép nhập có thể có dấu cách
fn _input(message: &str, endl: bool) -> String 
{
    if endl 
    {
        println!("{}", message);  // In thông điệp và xuống dòng
    } 
    else 
    {
        print!("{}", message);  // In thông điệp mà không xuống dòng
    }

    let mut input = String::new();
    
    // Đảm bảo flush stdout đúng cách, xử lý lỗi nếu có
    io::stdout().flush().expect("Failed to flush stdout");

    // Đọc đầu vào và xử lý lỗi nếu có
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trả về chuỗi đã loại bỏ khoảng trắng dư thừa và chuyển thành String
    input.trim().to_string()
}




fn sleep(seconds: u32) 
{
    let duration = time::Duration::new(seconds as u64, 0);
    thread::sleep(duration);
}




fn main() 
{
    const N: i32 = 100;
    let fibonacci_opt = create_fibonacci(N);

    let fibonacci: Vec<i128>;
    match fibonacci_opt 
    {
        Some(fib) => 
        {
            fibonacci = fib;
        }
        None => 
        {
            println!("Không thể tạo dãy Fibonacci.");
            return;
        }
    }

    let menu = "Chương trình tính số Fibonacci. \n1. Tính số fibonacci thứ n \n2. In ra dãy số fibonacci. \n0. Thoát \n > ";

    loop 
    {
        let key = input(menu);
        if key == "0" 
        {
            println!("Chương trình đang tắt...");
            sleep(5);
            exit(0);
        } 
        else if key == "1" 
        {
            let k = input("Nhập k = ");
            match k.parse::<i32>() 
            {
                Ok(k_value) => 
                {
                    if 0 <= k_value && k_value <= N 
                    {
                        println!("Fibonacci[{}] = {}", k_value, fibonacci[k_value as usize]);
                    } 
                    else 
                    {
                        println!("k = {} không thuộc khoảng [0, {}]", k_value, N);
                    }
                }
                Err(_) => 
                {
                    println!("k không phải là một số hợp lệ.");
                }
            }
        } 
        else if key == "2" 
        {
            let k = input("Nhập số lượng số fibonacci được in ra: ");
            match k.parse::<i32>() 
            {
                Ok(k_value) => 
                {
                    if 0 <= k_value && k_value <= N 
                    {
                        for i in 0..=k_value 
                        {
                            println!("Fibonacci[{}] = {}", i, fibonacci[i as usize]);
                        }
                    } 
                    else 
                    {
                        println!("k = {} không thuộc khoảng [0, {}]", k_value, N);
                    }
                }
                Err(_) => 
                {
                    println!("k không phải là một số hợp lệ.");
                }
            }
        } 
        else 
        {
            println!("Không phải số. Xin thử lại");
        }
    }
}
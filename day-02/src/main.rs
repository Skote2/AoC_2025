//const INPUT:&str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const INPUT:&str = "194-253,81430782-81451118,7709443-7841298,28377-38007,6841236050-6841305978,2222204551-2222236166,2623-4197,318169-385942,9827-16119,580816-616131,646982-683917,147-181,90-120,3545483464-3545590623,4304-5747,246071-314284,8484833630-8484865127,743942-795868,42-53,1435-2086,50480-60875,16232012-16441905,94275676-94433683,61509567-61686956,3872051-4002614,6918792899-6918944930,77312-106847,282-387,829099-1016957,288251787-288311732,6271381-6313272,9877430-10095488,59-87,161112-224439,851833788-851871307,6638265-6688423,434-624,1-20,26-40,6700-9791,990-1307,73673424-73819233";



fn is_valid(serial_no:i64) -> bool {
    let mut serial_parts: Vec<(i64, i32)> = Vec::new();

    let mut temp_no = serial_no;
    let mut magnitude = 0;
    while temp_no != 0 {
        serial_parts.push((temp_no, magnitude));
        temp_no /= 10;
        magnitude += 1;
    }
    //println!("magnitude: {magnitude}");

    for part in serial_parts {
        //println!("digits part: ({},{})", part.0, part.1);
        if part.1 == (magnitude / 2) {
            let doubled = (part.0 * 10_i64.pow(part.1.try_into().unwrap())) + part.0;
            if doubled == serial_no {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let ranges = INPUT.split(',');
    let mut invalid_values:Vec<i64> = Vec::new();
    for range in ranges {
        //println!("range: {range}");
        let mut range_it = range.split('-');
        let start_str:&str = range_it.next().unwrap();
        let end_str:&str = range_it.next().unwrap();
        //println!("range: {start_str}-{end_str}");
        let start_int:i64 = start_str.parse::<i64>().unwrap();
        let end_int:i64 = end_str.parse::<i64>().unwrap();
        println!("range: {start_int}-{end_int}");

        for serial_no in start_int..=end_int {
            //println!("checking serial#: {serial_no}");

            if !is_valid(serial_no) {
                invalid_values.push(serial_no);
                println!("serial_no: {serial_no} is invalid");
            }
        }
    }
    let mut answer: i64 = 0;
    for invalid_value in invalid_values {
        answer += invalid_value;
    }
    println!("Answer: {answer}");
}

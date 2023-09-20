fn main() {
    println!("Hello, world!");
}

pub fn score_of_parentheses(s: String) -> i32 {
    let mut st: Vec<i32> = Vec::with_capacity(s.len());
    st.push(0);

    for c in s.chars() {
        if c == '(' {
            st.push(0);
        } else {
            let x = *st.last().unwrap();
            if x == 0 {
                st.pop();
                *st.last_mut().unwrap() += 1;
            } else {
                let mut y = *st.last_mut().unwrap();
                y *= 2;
                st.pop();
                *st.last_mut().unwrap() += y;
            }
        }
    }
    *st.last().unwrap()
}

pub fn score_of_parenthesesv1(s: String) -> i32 {
    let mut st: Vec<char> = Vec::with_capacity(s.len());
    let mut stn = vec![];

    for c in s.chars() {
        if c == '(' {
            st.push('(');
        } else {
            let top = *st.last().unwrap();
            if top == '(' {
                st.pop();
                st.push('1');
                stn.push(1);
            } else {
                let mut count = 0;
                while *st.last().unwrap() != '(' {
                    count += stn.last().unwrap();
                    // count += st.last().unwrap().to_digit(10).unwrap();
                    // count = if count <= 9 {
                    //     (count*2 + '0' as u32) as u8 as char;
                    // } else {
                    //         char
                    //     }

                    // let bush = if count <= 9 {
                    //     (count * 2 + '0' as u32) as u8 as char;
                    // } else {
                    //     char::from_digit(count, 10).unwrap()
                    // };

                    st.pop();
                    stn.pop();
                }
                st.pop();
                let bush = (count * 2 + '0' as u32) as u8 as char;
                // count *= 2;
                // let bush = char::from_digit(count, 10).unwrap();
                println!("DEBUGPRINT[1]: main.rs:23: bush={:#?}", bush);
                st.push('4');
                stn.push(count * 2);
            }
        }
    }

    println!("{:?}", st);

    let mut rez = 0;
    while let Some(top) = stn.last() {
        println!("{:?}", top);
        // let abood = top.to_digit(10).unwrap();
        let abood = top;
        rez += abood;
        stn.pop();
    }

    rez as i32
}

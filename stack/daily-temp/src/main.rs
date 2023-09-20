fn main() {
    let temperatures: Vec<i32> = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
    //vec of tuple as a stack to have index and value
    let mut st: Vec<(i32, usize)> = vec![];

    //init res with size of temp and all zeros
    let mut res: Vec<i32> = vec![0; temperatures.len()];

    for i in 0..temperatures.len() {
        //if there is
        //elements in the stack and the temp[i] > the st.last.0 (which in the tupple is the
        //value)
        //get the diff between the index i'm at and the
        while st.last().is_some() && temperatures[i] > st.last().unwrap().0 {
            //index of the top of the stack
            let rez = i - st.last().unwrap().1;

            //put it in the res vec which is the result
            res[st.last().unwrap().1] = rez as i32;

            //pop it as it has been already in the res vec
            st.pop();
        }

        //push the value and the index
        st.push((temperatures[i], i));
    }

    //[8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
    println!("{:?}", res);
}

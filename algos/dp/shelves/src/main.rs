fn main() {
    println!("Hello, world!");
    let books = vec![
        vec![1, 1],
        vec![2, 3],
        vec![2, 3],
        vec![1, 1],
        vec![1, 1],
        vec![1, 1],
        vec![1, 2],
    ];
    let shelf_width = 4;
    println!("{}", shelves(books, shelf_width));
}

fn shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut memory = vec![None; books.len()];
    _shelves(&books, &shelf_width, 0, &mut memory)
}

fn _shelves(
    books: &Vec<Vec<i32>>,
    shelf_width: &i32,
    st_idx: usize,
    memory: &mut Vec<Option<i32>>,
) -> i32 {
    if st_idx == books.len() {
        return 0;
    }

    if let Some(ret) = memory[st_idx] {
        return ret;
    }

    let mut sum_height = i32::MAX;
    // for every group the heighest book is the heights of the whole group
    let mut max_height_for_a_block = 0;
    let mut block_of_widths = 0;
    // w x h
    for end in st_idx..books.len() {
        if block_of_widths + books[end][0] <= *shelf_width {
            max_height_for_a_block = max_height_for_a_block.max(books[end][1]);
            block_of_widths += books[end][0];
        } else {
            break;
        }
        // if block_of_widths > *shelf_width {
        //     break;
        // }
        // else return 0?
        // or put the next two lines inside the if
        let total_block = max_height_for_a_block + _shelves(books, shelf_width, end + 1, memory);
        sum_height = sum_height.min(total_block)
    }

    memory[st_idx] = Some(sum_height);
    sum_height
}

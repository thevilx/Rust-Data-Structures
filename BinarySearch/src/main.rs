fn main() {

    let arr = vec![1 , 5 , 7 , 7 , 9 , 11 , 19 , 21];
    let item = 5;

    let result = binary_search(&arr, item);
    let result_rec = binary_search_rec(&arr , item , 0 , arr.len() - 1);

    dbg!(result , result_rec);
}


fn binary_search(arr :&[i32] , target :i32) -> Option<usize>{

    let mut left = 0;
    let mut right = arr.len();


    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}


fn binary_search_rec(arr :&[i32] , target :i32 , left :usize , right:usize) -> Option<usize>{

    if left > right {
        return None;
    }

    let middle :usize = left + (right - left) / 2;

    if arr[middle] == target {
        return Some(middle);
    }

    else if arr[middle] > target{
        return binary_search_rec(arr , target , left , middle);
    }

    else if arr[middle] < target{
        return binary_search_rec(arr , target , middle + 1 , right);
    }

    None
}
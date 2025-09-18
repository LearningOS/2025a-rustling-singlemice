/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    quick_sort(array, 0, array.len() - 1);
}

fn quick_sort<T: PartialOrd>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(array, low, high);

        // 递归排序左半部分
        if pivot_index > 0 {
            quick_sort(array, low, pivot_index - 1);
        }

        // 递归排序右半部分
        if pivot_index < high {
            quick_sort(array, pivot_index + 1, high);
        }
    }
}

fn partition<T: PartialOrd>(array: &mut [T], low: usize, high: usize) -> usize {
    // 选择最后一个元素作为基准点
    let mut i = low;

    for j in low..high {
        // 如果当前元素小于等于基准点，将其移到左边
        if array[j] <= array[high] {
            array.swap(i, j);
            i += 1;
        }
    }

    // 将基准点放到正确位置
    array.swap(i, high);
    i
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}

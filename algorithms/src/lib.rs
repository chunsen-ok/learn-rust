//! Algorithms Using Rust

/// 选择排序 selection sort
/// 思想：从待排序的序列中选择最小的元素，将它放到已排序序列的末尾，直到未排序的元素个数为零。
pub fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    for i in 0..a.len() {
        for j in (i+1)..a.len() {
            if a[i] > a[j] {
                a.swap(i,j);
            }
        }
    }
}

/// 冒泡排序 bubble sort
/// 思想是：比较相邻元素，大的在前，则交换位置；每次遍历都会找到最大的元素，重复比较，则到序列中没有元素需要比较。
pub fn bubble_sort<T: PartialOrd>(a: &mut [T]) {
    for i in 0..a.len() {
        for j in 0..a.len() - 1 - i {
            if a[j] > a[j+1] {
                a.swap(j,j+1);
            }
        }
    }
}

/// 插入排序 insertion sort
/// 思想是：将一个元素插入到有序表中适当的位置。
pub fn insertion_sort<T: PartialOrd>(a: &mut [T]) {
    if a.len() < 1 { return; }

    for i in 1..a.len() {
        if a[i-1] > a[i] {
            let mut j = i;
            while j > 0 && a[j-1] > a[j] {
                a.swap(j-1,j);
                j -= 1;
            }
        }
    }
}

/// 希尔排序/缩小增量排序 shell sort
pub fn shell_sort<T:PartialOrd>(a: &mut [T]) {
    // ... to do
}

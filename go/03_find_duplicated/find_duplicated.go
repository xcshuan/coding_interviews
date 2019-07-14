package find_duplicated

const N = 10

//思路：新建一个大小为N的数组，记录每个值（下标）出现的次数，然后进行统计。
func findDuplicatedNumbers(arr [N]int) (int, bool) {
	h := [N]int{}
	for _, x := range arr {
		if x < 0 || x > N-1 {
			return 0, false
		}
		h[x]++
	}
	for i, x := range h {
		if x > 1 {
			return i, true
		}
	}
	return 0, false
}

func findDuplicatedNumbers1(arr [N]int) (int, bool) {
	for _, x := range arr {
		if x < 0 || x > N-1 {
			return 0, false
		}
	}
	for i := range arr {
		for arr[i] != i {
			if arr[i] == arr[arr[i]] {
				return arr[i], true
			}
			temp := arr[i]
			arr[i] = arr[temp]
			arr[temp] = temp
		}
	}
	return 0, false
}

package find_in_2dmatrix

func findIn2DMatrix(a [][]int, rows, columns, number int) bool {
	var found = false
	if a != nil && rows > 0 && columns > 0 {
		var row = 0
		var column = columns - 1
		for row < rows && column >= 0 {
			if a[row][column] == number {
				found = true
				break
			} else if a[row][column] > number {
				column--
			} else if a[row][column] < number {
				row++
			}
		}
	}
	return found
}

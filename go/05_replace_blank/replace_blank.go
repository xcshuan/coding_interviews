package replace_blank

func replaceBlank(str string) string {
	var originlength = len(str)
	var newLength = originlength
	for i := 0; i < originlength; i++ {
		if str[i] == ' ' {
			newLength += 2
		}
	}
	newStr := make([]byte, newLength)
	var j = 0
	for i := 0; i < originlength; i++ {
		if str[i] == ' ' {
			newStr[j] = '%'
			j++
			newStr[j] = '2'
			j++
			newStr[j] = '0'
			j++
			continue
		}
		newStr[j] = str[i]
		j++
	}
	return string(newStr)
}

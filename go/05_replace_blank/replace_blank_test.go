package replaceblank

import (
	"testing"
)

func TestReplaceBlank(t *testing.T) {
	str := "www.b ai du. com"
	x := replaceBlank(str)
	t.Errorf(x)
}

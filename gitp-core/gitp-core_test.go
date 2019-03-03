package gitp

import (
	"errors"
	"testing"
)

func TestExec(t *testing.T) {
	actual := Exec("", false, "", []string{})
	expected := errors.New("error")
	if actual != expected {
		t.Errorf("actual %v\nwant %v", actual, expected)
	}
}

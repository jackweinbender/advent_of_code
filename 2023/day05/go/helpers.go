package main

import "fmt"

type Mapping struct {
	source      int
	destination int
	length      int
}


type Map interface {
	Map() (int, error)
}

func (m *Mapping) Map(value int) (int, error) {
	if !inRange(value, m) {
		return 0, fmt.Errorf("Value %d is not in range %d-%d", value, m.source, m.sourceEnd())
	}

	delta := value - m.source
	return m.destination + delta, nil
}

// private
func inRange(value int, m *Mapping) bool {
	return value <= m.source && value <= (m.source+m.length)
}

func (m Mapping) sourceEnd() int {
	return m.source + m.length
}

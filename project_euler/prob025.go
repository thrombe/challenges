// 1000-digit Fibonacci number

package main

import (
	"fmt"
	"math/big"
	"time"
)

func main() {
	start := time.Now()
	fmt.Println(ans(1000))//
	end := time.Now()
	diff := end.Sub(start).Seconds()
	fmt.Println((diff))

}

func ans(n int) int {
	fib := fibonacci()
	i := 0
	var next *big.Int
	var length int
	for length < n {
		i++
		next = fib()
		length = len(next.Text(10))
	}
	return i
}

func fibonacci() func() *big.Int {
	y := big.NewInt(int64(1))
	x := big.NewInt(int64(0))
	z := big.NewInt(int64(0))

	return func() *big.Int {
		z.Add(x, y)
		x.Set(y)
		y.Set(z)
		return x
	}
}

/*
func pow(n int, power int) int {
	result := 1
	for i := 0; i < power; i++ {
		result = result*n
	}
	return result
}

func sqrt(n int) float64 { // just so if i ever want to implement this myself
	return math.Sqrt(float64(n))
}
*/
// Reciprocal cycles

/* DONE -----EZ OPTIMISATION - start from upper limit instead of lower,
and stop when n < length of unique decimals in the largest known 
potential answer.
no. of unique decimals in any 1/n can be (n-1) max since in long
division, when the remainder repeats, the decimals start repeating
and the no. of unique remainders are (n-1) at max.
*/
/* DONE ------possible memory optimisation- create a goroutine which creates more
goroutines as needed. this will help limit the ram usage and maybe 
the processing too
buffer channels - make(chan int 8)
use switch statements- select {case ch <- x: do stuff after the 'ch<-' part}
use - item, ok := <- ch
*/

package main

import (
	"fmt"
	"time"
	// "sync"
)

func main() {
	start := time.Now()
	fmt.Println(ans3(1000))//
	end := time.Now()
	diff := end.Sub(start).Seconds()
	fmt.Println((diff))
}

func genRem(rem int, div int) func() (int, int) {
	var multiples [9]int
	for i := 1; i != 10; i++ {
		multiples[i-1] = div*i
	}
	return func() (int, int) {
		rem = 10*rem
		if rem < div {return rem, 0}
		for i := 8; true; i-- { // 8 cuz just 9 elements
			if multiples[i] <= rem {
				rem = rem - multiples[i]
				return rem, i+1 // +1 for changing from index
			}
		}
		return 0, 0 // compiler panic ?
	}
}

func decLength(div int) (int, string) {
	rems := make(map[int]struct{}) // set // stack said the "values" take absolutely no space
	var rem, quo int
	var decimals string
	divide := genRem(1, div)
	rem, quo = divide()
	var isPresent bool
	for isPresent != true {
		decimals += fmt.Sprintf("%v", quo)
		rems[rem] = struct{}{} // setting value to the set
		rem, quo = divide()
		_, isPresent = rems[rem]
	}
	return len(rems), decimals
}

func ans(n int) (int, int, string) { // without concurrency
	var max, num int
	var decimal string
	for i := n-1; i > 1; i-- {
		len, dec := decLength(i)
		if max < len {
			num, max, decimal = i, len, dec
		} else if i <= max {break}
	}
	return num, max, decimal
}

func ans2(n int) (int, int, string) { // with concurrency
	type element struct {
		i int
		len int
		dec string
	}
	// wg := sync.WaitGroup{}
	var max, num int
	var decimal string
	ch := make(chan element)
	for i := 2; i < n; i++ {
		// wg.Add(1)
		go func(i int, ch chan<- element) {
			ele := element{}
			ele.len, ele.dec = decLength(i)
			ele.i =  i
			ch <- ele
			// wg.Done()
			}(i, ch)
	}
	// wg.Wait()
	for i := 2; i < n; i++ {
		ele := <- ch
		if max < ele.len {num, max, decimal = ele.i, ele.len, ele.dec}
	}
	return num, max, decimal
}

func ans3(n int) (int, int, string) { // with limits on active goroutines and the smort optimisation
	type element struct {
		i int
		len int
		dec string
	}
	gorouLimit := 4 // can set it up to the no. of cores go can use
	var max, num int
	var decimal string
	quit := make(chan bool)
	sync := make(chan int, gorouLimit)
	ch := make(chan element, gorouLimit)
	go func(ch chan<-element, quit chan bool, sync chan int) {
		for i := n-1; i > 1; i-- { // starting from larger limit
			select {
			case <-quit: return
			default:
				sync <- 1 // using this o keep track of alive goroutines and limit it to gorouLimit
				go func(i int, ch chan<- element, sync chan int) {
					ele := element{}
					ele.len, ele.dec = decLength(i)
					ele.i =  i
					select {
					//case <-quit: return // commented cuz we wanna make sure to chack all nums in range limit > *i* > max
					case ch <- ele:
					}
					}(i, ch, sync)
			}
		}
	}(ch, quit, sync)
	var qit bool // only perpose is to store if we started the termination
	for i := 2; i < n; i++ {
		<- sync // so that the cache dosent just keep increasing cuz the process is done in parallel
		ele := <- ch
		if max < ele.len {num, max, decimal = ele.i, ele.len, ele.dec}
		if ele.i <= max && qit == false { // terminate
		    close(quit) // sends signal to the goroutine creator to stop
		    qit = true
		}
		if len(sync) == 0 && qit == true {break} // break only when all alive goroutines are checked for potential max
	}
	return num, max, decimal
}
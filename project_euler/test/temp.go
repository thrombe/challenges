// Reciprocal cycles

/* EZ OPTIMISATION - start from upper limit instead of lower,
and stop when n < length of unique decimals in the largest known 
potential answer.
no. of unique decimals in any 1/n can be (n-1) max since in long
division, when the remainder repeats, the decimals start repeating
and the no. of unique remainders are (n-1) at max.
*/
/*possible memory optimisation- create a goroutine which creates more
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
	"github.com/pkg/profile"
//commands->  go tool pprof --alloc_space <.pprof file path> // --inuse_space, --alloc_objects ... // top 20 - to view 20 items // list <function name> - to view details regarding the function

)
// var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to file")
/*
var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to `file`")
var memprofile = flag.String("memprofile", "", "write memory profile to `file`")

func main() {
    flag.Parse()
    if *cpuprofile != "" {
        f, err := os.Create(*cpuprofile)
        if err != nil {
            log.Fatal("could not create CPU profile: ", err)
        }
        defer f.Close() // error handling omitted for example
        if err := pprof.StartCPUProfile(f); err != nil {
            log.Fatal("could not start CPU profile: ", err)
        }
        defer pprof.StopCPUProfile()
    }

    start := time.Now()
    fmt.Println(ans3(100000))//
    end := time.Now()
    diff := end.Sub(start).Seconds()
    fmt.Println((diff))

    if *memprofile != "" {
        f, err := os.Create(*memprofile)
        if err != nil {
            log.Fatal("could not create memory profile: ", err)
        }
        defer f.Close() // error handling omitted for example
        runtime.GC() // get up-to-date statistics
        if err := pprof.WriteHeapProfile(f); err != nil {
            log.Fatal("could not write memory profile: ", err)
        }
    }
}*/
/*
func main() {
    flag.Parse()
    if *cpuprofile != "" {
        f, err := os.Create(*cpuprofile)
        if err != nil {
            log.Fatal(err)
        }
        pprof.StartCPUProfile(f)
        defer pprof.StopCPUProfile()
    }

    start := time.Now()
    fmt.Println(ans3(100000))//
    end := time.Now()
    diff := end.Sub(start).Seconds()
    fmt.Println((diff))
}*/

func main() {
    
    defer profile.Start(profile.MemProfile).Stop()
    
    start := time.Now()
    fmt.Println(ans3(50000))//
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
	//var rems []int
	var rem, quo int
	var decimals string
	divide := genRem(1, div)
	rem, quo = divide()
	var isPresent bool
	for isPresent != true {
		decimals += fmt.Sprintf("%v", quo)
		rems[rem] = struct{}{} // setting value to the set
		//rems = append(rems, rem)
		rem, quo = divide()
		_, isPresent = rems[rem]
		//for _, v := range rems {
		     //if v == rem {isPresent = true}
		//}
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

func ans3(n int) (int, int, string) { // with limits on active goroutines
	type element struct {
		i int
		len int
		dec string
	}
	// wg := sync.WaitGroup{}
	var max, num int
	var decimal string
	quit := make(chan bool)
	sync := make(chan int, 4)
	ch := make(chan element, 4)
	go func(ch chan<-element, quit chan bool, sync chan int) {
		for i := n-1; i > 1; i-- {
			sync <- 1
			select {
			case <-quit: return
			default:
				// wg.Add(1)
				go func(i int, ch chan<- element, sync chan int) {
					// defer func() {<- sync}()
					ele := element{}
					ele.len, ele.dec = decLength(i)
					ele.i =  i
					select {
					case <-quit: return
					case ch <- ele:
					}
					// wg.Done()
					}(i, ch, sync)
			}
		}
	}(ch, quit, sync)
		// wg.Wait()
	for i := 2; i < n; i++ {
		<- sync // so that the cache dosent just keep increasing cuz the process is done in parallel
		ele := <- ch
		if max < ele.len {num, max, decimal = ele.i, ele.len, ele.dec}
		// ele.i, ele.len, ele.dec = 0, 0, ""
	}
	close(quit)
	return num, max, decimal
}
/*
func ans4(n int) (int, int, string) { // try removing structs to see if ram improves

	element := make(map [int][]string)

	// wg := sync.WaitGroup{}
	var max, num int
	var decimal string
	quit := make(chan bool)
	sync := make(chan int, 150)
	ch := make(chan element, 150)
	go func(ch chan<-element, quit chan bool, sync chan int) {
		for i := n-1; i > 1; i-- {
			sync <- 1
			select {
			case <-quit: return
			default:
				// wg.Add(1)
				go func(i int, ch chan<- element, sync chan int) {
					// defer func() {<- sync}()
					ele := element{}
					ele.len, ele.dec = decLength(i)
					ele.i =  i
					select {
					case <-quit: return
					case ch <- ele:
					}
					// wg.Done()
					}(i, ch, sync)
			}
		}
	}(ch, quit, sync)
		// wg.Wait()
	for i := 2; i < n; i++ {
		<- sync // so that the cache dosent just keep increasing cuz the process is done in parallel
		ele := <- ch
		if max < ele.len {num, max, decimal = ele.i, ele.len, ele.dec}
		// ele.i, ele.len, ele.dec = 0, 0, ""
	}
	close(quit)
	return num, max, decimal
}
*/
// Lexicographic permutations
/*
summation n!*k such that k <= n and the term dosent make the summation more than or equal to the limit
a = 9!×2 + 8!×6 + 7!×6 + 6!×2 + 5!×5 + 4!×1 + 3!×2 + 2!×1 + 1!×1 + 0!×0 # = 99999 permutation[99999] (ie the 100000th element)
this gives 2662512110 which are basically the index of the character in a sorted character list
liss = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] #note for eg: for the 3rd no., liss[6] would be 8 and not 6
solve above #(originally done on a piece of paper with calculator)
*/

package main
import "fmt"

func main() {
    list := []string{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}
    fmt.Println(permute(1000000, list))
}

func factorial(n int) int {
    tot := 1
    for i := 2; i <= n; i++ {
        tot = tot*i
    }
    return tot
}

func permute(rank int, list []string) string {
    sum := 0
    n := len(list) - 1
    var result string
    for loop := len(list); loop != 0; loop-- {
        nfac := factorial(n)
        k := 0
        for sum + nfac*(k+1) < rank {
            k++
        }
        sum += nfac*k
        result += list[k]
        list = append(list[ :k], list[k+1: ]...)
        // fmt.Println(k, n)
        n--
    }
    return result
}
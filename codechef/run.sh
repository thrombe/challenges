name=$1

rustc "./src/problems/$name.rs" -o "./a.out"
# rustc -C opt-level=3 "./src/problems/$name.rs" -o "./a.out"
cat "./input_files/$name.txt" | ./a.out
rm ./a.out
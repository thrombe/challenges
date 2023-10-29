name=$1

if [ -f "./src/problems/$name.rs" ]; then
    echo "$name exists."
else 
    cp ./src/lib/template_v3.rs "./src/problems/$name.rs"
    touch "./input_files/$name.txt"
    sed -i "2imod $name;" ./src/problems/mod.rs

    code "./src/problems/$name.rs"
    code "./input_files/$name.txt"
fi

